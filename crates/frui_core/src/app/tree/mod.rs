use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell, RefMut, UnsafeCell},
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
    sync::Arc,
};

use druid_shell::IdleToken;

use crate::{
    api::{
        contexts::{render::LayoutCtxOS, RawBuildCtx},
        local_key::LocalKeyAny,
        pointer_events::events::PointerEvent,
        IntoWidgetPtr, WidgetPtr,
    },
    app::runner::window_handler::{APP_HANDLE, NEED_REBUILD},
    macro_exports::PaintCtxOS,
    render::{Canvas, Constraints, Offset, Size},
};

use self::pointer_handler::PointerHandler;

pub mod pointer_handler;

pub(crate) struct WidgetTree {
    /// Root widget contains necessary configuration to support `InheritedWidget`s.
    /// Child of that dummy node is the actual root node.
    dummy_root: UnsafeCell<Box<WidgetNode>>,
    pointer_handler: PointerHandler,
}

impl WidgetTree {
    pub fn new(root_widget: WidgetPtr) -> Self {
        // Dummy node used to initialize first InheritedWidget.
        let mut dummy_root = WidgetNode::default();

        // Actual root widget.
        let root = WidgetNode::new(root_widget, None, WidgetNode::node_ref(&dummy_root));

        // Make that root widget child of dummy node.
        unsafe { (&mut *dummy_root.children_ptr_mut()).push(root) };

        Self {
            dummy_root,
            pointer_handler: PointerHandler::default(),
        }
    }

    pub fn layout(&mut self, constraints: Constraints) {
        LayoutCtxOS::new(self.root()).layout(constraints);
    }

    pub fn paint(&mut self, piet: &mut Canvas) {
        PaintCtxOS::new(self.root()).paint(piet, &Offset::default());
    }

    pub fn handle_pointer_event(&mut self, event: PointerEvent) {
        let root = self.root();
        self.pointer_handler.handle_pointer_event(root, event)
    }

    /// Returns the root widget node, extracting it from the dummy node.
    fn root(&mut self) -> WidgetNodeRef {
        unsafe { WidgetNode::node_ref(&(&*self.dummy_root.children_ptr_mut())[0]) }
    }
}

impl Default for WidgetTree {
    fn default() -> Self {
        Self {
            dummy_root: WidgetNode::default(),
            pointer_handler: Default::default(),
        }
    }
}

impl Drop for WidgetTree {
    fn drop(&mut self) {
        // Safety: `drop_mut` is called only once here.
        unsafe { WidgetNode::drop_mut(&mut self.dummy_root) };
    }
}

#[derive(Clone)]
pub struct IsAlive(pub Arc<Cell<bool>>);

impl IsAlive {
    pub fn new() -> Self {
        IsAlive(Arc::new(Cell::new(true)))
    }
}

pub(crate) enum Inheritance {
    /// This is the `InheritedWidget`.
    Inheritor {
        /// This `HashMap` contains all `InheritedWidget` ancestors that are accessible from this subtree.
        active_inheritors: HashMap<TypeId, WidgetNodeRef>,
        /// This `HashSet` contains all descendant widgets which inherit from this widget.
        inheriting_widgets: HashSet<WidgetNodeRef>,
    },
    /// This is any widget that inherits from `InheritedWidget` ancestor.
    Inheritee {
        /// This is the closest `InheritedWidget` ancestor of this widget.
        inherited_ancestor: WidgetNodeRef,
        /// This is used to efficiently unregister this widget from all inherited ancestors.
        inherits_from: HashSet<WidgetNodeRef>,
    },
}

pub(crate) struct WidgetInner {
    pub dirty: bool,
    pub state: Box<dyn Any>,
    pub render_data: RenderData,
    pub inheritance: Inheritance,
}

pub(crate) struct WidgetNode {
    widget_ptr: WidgetPtr<'static>,

    context: RawBuildCtx,
    inner: RefCell<WidgetInner>,

    parent: Option<WidgetNodeRef>,
    children: Vec<UnsafeCell<Box<WidgetNode>>>,
}

impl WidgetNode {
    /// Build a subtree of widgets from given `widget`.
    fn new(
        widget: WidgetPtr,
        parent: Option<WidgetNodeRef>,
        mut inherited_ancestor: WidgetNodeRef,
    ) -> UnsafeCell<Box<Self>> {
        // Safety: We enforce correct lifetime of each pointer manually.
        let widget_ptr =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(widget.clone()) };

        let mut this = UnsafeCell::new(Box::new(WidgetNode {
            widget_ptr,
            context: RawBuildCtx {
                node: WidgetNodeRef {
                    is_alive: Rc::new(Cell::new(true)),
                    ptr: std::ptr::null_mut(),
                },
            },
            inner: RefCell::new(WidgetInner {
                dirty: false,
                state: widget.raw().create_state(),
                render_data: RenderData::new(&widget),
                inheritance: Inheritance::new(&widget, &inherited_ancestor),
            }),
            parent,
            children: Vec::new(),
        }));

        // We access fields of `WidgetNode` in this way to not pop tag for `context.node_ref.ptr`
        // pointer from the borrow stack.
        let (inner_ref, context_ref_mut, children_ref_mut) = (
            unsafe { &*this.inner_ptr() },
            unsafe { &mut *this.context_ptr_mut() },
            unsafe { &mut *this.children_ptr_mut() },
        );

        context_ref_mut.node.ptr = this.node_ptr_mut();

        let node_ref = WidgetNode::node_ref(&this);

        //
        // Insert this node to `active_inheritors`.

        let mut inner = inner_ref.borrow_mut();

        if let Inheritance::Inheritor {
            ref mut active_inheritors,
            ..
        } = inner.inheritance
        {
            inherited_ancestor = node_ref.clone();
            active_inheritors.insert(widget.inherited_key(), node_ref.clone());
        }

        drop(inner);

        //
        // Build children.

        // From this point on, `WidgetNode` cannot be accessed mutably or otherwise its
        // pointer tag will be popped and `context` invalidated.

        let children = unsafe {
            widget
                .build(&*this.context_ptr())
                .into_iter()
                .map(|child_widget_ptr| {
                    WidgetNode::new(
                        child_widget_ptr,
                        Some(node_ref.clone()),
                        inherited_ancestor.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };

        *children_ref_mut = children;

        WidgetNode::mount(&this);

        this
    }

    /// Update subtree starting at this node.
    pub fn update_subtree(s: &UnsafeCell<Box<Self>>) {
        unsafe { Self::update_subtree_ptr(std::ptr::addr_of_mut!(*(*s.get()))) }
    }

    /// Safety:
    ///
    /// There are more assumptions, but the main one is that `s` must come from the
    /// [`UnsafeCell`] that was received from the [`WidgetNode::new`].
    pub unsafe fn update_subtree_ptr(s: *mut WidgetNode) {
        // We access fields of `WidgetNode` in this way to not pop tag for `context.node_ref.ptr`
        // pointer from the borrow stack.
        let (inner_ref, widget_ref, context_ref, old_children_ref) = (
            &*s.inner_ptr(),
            &*s.widget_ptr(),
            &*s.context_ptr(),
            &mut *s.children_ptr_mut(),
        );

        // Todo: In the calling function, first check that WidgetKey of passed `widget` is indeed
        // `dirty`, as it may have been already updated by a previous call to this function.
        inner_ref.borrow_mut().dirty = false;

        let inherited_ancestor = &inner_ref
            .borrow_mut()
            .inheritance
            .inherited_ancestor(&context_ref.node);

        let mut old_children = std::mem::take(old_children_ref)
            .into_iter()
            .map(|c| Some(c))
            .collect::<Vec<_>>();
        let new_children_build = widget_ref.build(context_ref);
        let mut new_children = Vec::with_capacity(new_children_build.len());

        for (n, new_child) in new_children_build.into_iter().enumerate() {
            if new_child.has_key() {
                if let Some(n) = old_children.find_key(&new_child) {
                    // Remove old_child from old_children.
                    let old_child = std::mem::take(old_children.get_mut(n).unwrap()).unwrap();

                    // Try to update old_child with new_child.
                    new_children.push(WidgetNode::update(old_child, new_child));
                } else {
                    // Build new_child.
                    let child = WidgetNode::new(
                        new_child,
                        Some(context_ref.node.clone()),
                        inherited_ancestor.clone(),
                    );

                    new_children.push(child);
                }
            } else {
                if let Some(Some(old_child)) = old_children.get(n) {
                    if (&*old_child.widget_ptr()).has_key() {
                        // Build new_child.
                        let child = WidgetNode::new(
                            new_child,
                            Some(context_ref.node.clone()),
                            inherited_ancestor.clone(),
                        );

                        new_children.push(child);
                    } else {
                        // Remove old_child from old_children.
                        let old_child = std::mem::take(old_children.get_mut(n).unwrap()).unwrap();

                        // Try to update old_child with new_child.
                        new_children.push(WidgetNode::update(old_child, new_child));
                    }
                } else {
                    // Build new_child.
                    let child = WidgetNode::new(
                        new_child,
                        Some(context_ref.node.clone()),
                        inherited_ancestor.clone(),
                    );

                    new_children.push(child);
                }
            }
        }

        // Drop children which didn't get reused.
        for old_child in old_children.into_iter() {
            if let Some(old_child) = old_child {
                WidgetNode::drop(old_child);
            }
        }

        // Update children keys.
        *old_children_ref = new_children;
    }

    pub fn update(mut s: UnsafeCell<Box<Self>>, new_widget: WidgetPtr) -> UnsafeCell<Box<Self>> {
        // We enforce correct lifetime of each pointer manually.
        let new_widget =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(new_widget) };

        // We access fields of `WidgetNode` in this way to not pop tag for `context.node_ref.ptr`
        // pointer from the borrow stack.
        let (widget_ref, parent_ref, context_ref) =
            unsafe { (&*s.widget_ptr(), &*s.parent_ptr(), &*s.context_ptr()) };

        // If widgets share the same generic-independent TypeId and types of states of both
        // widgets match (most likely a bug if they don't) we preserve that widget's state.
        if widget_ref.can_update(&new_widget) {
            // Safety:
            //
            // We compare and reuse configurations of widgets to optimize unnecessary view
            // rebuilds if old and new widgets are identical.
            //
            // This operation must be performed with special scrutiny, since preserving
            // pointers to a widget which we dropped earlier will lead to use after free.
            //
            // For that reason the following `eq` method uses our own `CheapEq`
            // implementation which ensures that each widget is correctly compared (that
            // includes fields that are references).
            //
            // To be precise, we can compare and reuse widgets only if they are either:
            // - owned and equal
            // - borrowed, equal and have the same pointer addresses
            //
            // Second condition comes from the fact that two borrows of different pointer
            // addresses (1) reference different widgets or (2) reference the same widgets
            // where one of them is owned by a widget that got deallocated.
            //
            // Note:
            //
            // We could theoretically reuse borrowed widgets of different pointer addresses
            // if they are ZST, however it is currently treated as UB to dereference ZST to
            // deallocated memory, so we don't do it.
            if widget_ref.eq(&new_widget) {
                // Safety: Since we reuse old widget ptr, we can drop the newly created one.
                unsafe { WidgetPtr::drop(new_widget) };

                return s;
            } else {
                // Unmount old widget.
                WidgetNode::unmount(&s);

                // Safety:
                //
                // Old pointer is still alive (and didn't move, since it is either boxed or
                // it points to a value in ascenstor widget), so it is fine to unmount/drop
                // children of this widget (in `update_subtree` below).
                //
                // Additionally, we don't access `widget_ref` after this call, so we don't
                // cause aliasing UB.
                let old_widget_ptr =
                    unsafe { std::mem::replace(&mut *s.widget_ptr_mut(), new_widget) };

                // Update descendants of this node, stopping at equal widgets or a leaf node.
                WidgetNode::update_subtree(&s);

                // Safety:
                //
                // There are no children widgets referencing the old pointer (because we
                // updated them all in `update_subtree` above), so we can safely drop it.
                unsafe { WidgetPtr::drop(old_widget_ptr) };

                // Mount updated widget.
                WidgetNode::mount(&s);

                return s;
            }
        } else {
            let parent = parent_ref.clone();

            // We use the parent's inherited ancestor, since `s` could be an InheritedWidget
            // itself. After we deallocate `s` this would cause use after free.
            let inherited_ancestor = parent
                .as_ref()
                .unwrap()
                .borrow()
                .inheritance
                .inherited_ancestor(&context_ref.node);

            // Unmount and drop subtree starting at this node.
            WidgetNode::drop(s);

            // Build new subtree in its place.
            return WidgetNode::new(new_widget, parent, inherited_ancestor);
        }
    }

    /// Drop this widget node and all its descendants.
    pub fn drop(mut s: UnsafeCell<Box<Self>>) {
        // Safety: `drop_mut` can be called only once, since we own `s`.
        unsafe { WidgetNode::drop_mut(&mut s) };
    }

    /// Drop this widget node and all its descendants.
    ///
    /// # Safety
    ///
    /// This function can only be called once on a given [`WidgetNode`].
    pub unsafe fn drop_mut(s: &mut UnsafeCell<Box<Self>>) {
        let s_ptr_mut = std::ptr::addr_of_mut!(**s.get());
        let inner_ref = std::ptr::addr_of!((*s_ptr_mut).inner);
        let children_ref_mut = std::ptr::addr_of_mut!((*s_ptr_mut).children);

        let mut children = std::mem::take(&mut *children_ref_mut).into_iter();

        while let Some(child) = children.next() {
            WidgetNode::drop(child);
        }

        WidgetNode::unmount(&s);

        // Remove this widget from its parent's list of inheriting widgets.
        match &(&*inner_ref).borrow().inheritance {
            Inheritance::Inheritee {
                inherited_ancestor: _,
                inherits_from,
            } => {
                let node_ref = &WidgetNode::node_ref(s);

                for inheritor in inherits_from.iter() {
                    let mut inheritor_ref = inheritor.borrow_mut();
                    let inheriting_widgets = inheritor_ref.inheritance.inheriting_widgets();
                    inheriting_widgets.remove(node_ref);
                }
            }
            Inheritance::Inheritor { .. } => {}
        };

        // From this point on we `Context` will not be accessed again immutably,
        // thus we can access whole `Self` mutably.
        let s = &mut *s_ptr_mut;

        if let Some(ptr) = s.widget_ptr.owned {
            drop(Box::from_raw(ptr));
        }

        s.context.node.is_alive.set(false);
    }

    //
    //

    #[allow(unused)]
    pub fn debug_name_short(s: &UnsafeCell<Box<Self>>) -> &'static str {
        let widget = unsafe { &*s.widget_ptr() };

        widget.raw().debug_name_short()
    }

    pub fn mount(s: &UnsafeCell<Box<Self>>) {
        let widget = unsafe { &*s.widget_ptr() };
        let context = unsafe { &*s.context_ptr() };

        widget.mount(context)
    }

    pub fn unmount(s: &UnsafeCell<Box<Self>>) {
        let widget = unsafe { &*s.widget_ptr() };
        let context = unsafe { &*s.context_ptr() };

        widget.unmount(context)
    }

    //
    //

    pub fn node_ref(s: &UnsafeCell<Box<Self>>) -> WidgetNodeRef {
        unsafe { (&*s.context_ptr()).node.clone() }
    }

    pub fn local_key<'a>(s: &UnsafeCell<Box<WidgetNode>>) -> Option<LocalKeyAny<'a>> {
        unsafe { (&*s.widget_ptr()).raw().local_key() }
    }
}

#[derive(Clone)]
pub struct WidgetNodeRef {
    is_alive: Rc<Cell<bool>>,
    ptr: *mut WidgetNode,
}

impl WidgetNodeRef {
    #[track_caller]
    pub(crate) fn borrow(&self) -> Ref<'_, WidgetInner> {
        assert_eq!(self.is_alive.get(), true);
        unsafe { (&*self.ptr.inner_ptr()).borrow() }
    }

    #[track_caller]
    pub(crate) fn borrow_mut(&self) -> RefMut<'_, WidgetInner> {
        assert_eq!(self.is_alive.get(), true);
        unsafe { (&*self.ptr.inner_ptr()).borrow_mut() }
    }

    //

    pub fn update_subtree(&self) {
        assert_eq!(self.is_alive.get(), true);
        unsafe { WidgetNode::update_subtree_ptr(self.ptr) }
    }

    pub fn mark_dirty(&self) {
        assert_eq!(self.is_alive.get(), true);

        APP_HANDLE.with(|handle| {
            handle
                .borrow_mut()
                .as_mut()
                .expect("APP_HANDLE wasn't set")
                .schedule_idle(IdleToken::new(0));
        });

        if !self.borrow_mut().dirty {
            self.borrow_mut().dirty = true;

            NEED_REBUILD.with(|dirty| {
                dirty.lock().unwrap().push(self.clone());
            });
        }
    }

    pub fn mark_dependent_widgets_as_dirty(&self) {
        assert_eq!(self.is_alive.get(), true);

        match &self.borrow().inheritance {
            Inheritance::Inheritor {
                inheriting_widgets, ..
            } => {
                for widget in inheriting_widgets.iter() {
                    widget.mark_dirty()
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn depend_on_inherited_widget_of_key<'a, K>(&'a self) -> Option<WidgetNodeRef>
    where
        K: 'static,
    {
        assert_eq!(self.is_alive.get(), true);
        let key = TypeId::of::<K>();

        let mut node_ref = self.borrow_mut();

        let (inherited_ancestor, inherits_from) = match &mut node_ref.inheritance {
            Inheritance::Inheritee {
                inherited_ancestor,
                inherits_from,
            } => (inherited_ancestor, inherits_from),
            _ => unreachable!(),
        };

        let inherited_ref = inherited_ancestor.borrow();

        // Closest InheritedWidget ancestor.
        let active_inheritors = match &inherited_ref.inheritance {
            Inheritance::Inheritor {
                active_inheritors, ..
            } => active_inheritors,
            _ => unreachable!(),
        };

        // Target InheritedWidget (matching K).
        let inherited_widget = active_inheritors.get(&key)?.clone();
        drop(inherited_ref);
        let mut inherited_widget_ref = inherited_widget.borrow_mut();

        let inheriting_widgets = match &mut inherited_widget_ref.inheritance {
            Inheritance::Inheritor {
                inheriting_widgets, ..
            } => inheriting_widgets,
            _ => unreachable!(),
        };

        // Register this node in InheritedWidget.
        inheriting_widgets.insert(self.clone());

        // Remember which InheritedWidgets we are inheriting from.
        inherits_from.insert(inherited_widget.clone());

        Some(inherited_widget.clone())
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.get()
    }

    #[track_caller]
    pub fn widget<'a>(&'a self) -> &'a WidgetPtr<'static> {
        assert!(self.is_alive.get());
        unsafe { &*self.ptr.widget_ptr() }
    }

    #[track_caller]
    pub(crate) fn children<'a>(&'a self) -> &'a [UnsafeCell<Box<WidgetNode>>] {
        assert!(self.is_alive.get());
        unsafe { &*self.ptr.children_ptr() }
    }

    #[allow(unused)]
    pub fn debug_name_short(&self) -> &'static str {
        self.widget().raw().debug_name_short()
    }
}

impl PartialEq for WidgetNodeRef {
    fn eq(&self, other: &Self) -> bool {
        self.is_alive.as_ptr() == other.is_alive.as_ptr()
    }
}

impl Eq for WidgetNodeRef {}

impl Hash for WidgetNodeRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_alive.as_ptr().hash(state);
    }
}

impl std::fmt::Debug for WidgetNodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_alive.get() {
            write!(
                f,
                "WidgetNodeRef ({})",
                self.widget().raw().debug_name_short()
            )
        } else {
            f.debug_struct("WidgetNodeRef (removed)")
                .field("ptr", &self.ptr)
                .finish()
        }
    }
}

pub(crate) struct RenderData {
    /// Render state used by this widget.
    pub state: Box<dyn Any>,
    /// Data that can be accessed from parent widget.
    pub parent_data: Box<dyn Any>,
    /// Size computed during last layout.
    pub size: Size,
    // Todo: Use Point instead of Offset.
    /// Offset received during last paint.
    pub local_offset: Offset,
    /// Incoming constraints received during last layout.
    pub constraints: Constraints,
    /// Whether child was laid out. Used to display an error message when
    /// painting children without laying them out beforehand.
    pub laid_out: bool,
}

impl RenderData {
    fn new(widget: &WidgetPtr) -> RenderData {
        RenderData {
            state: widget.raw().create_render_state(),
            parent_data: widget.raw().create_parent_data(),
            size: Size::default(),
            local_offset: Offset::default(),
            constraints: Constraints::default(),
            laid_out: false,
        }
    }
}

impl Inheritance {
    fn new(widget: &WidgetPtr, inherited_ancestor: &WidgetNodeRef) -> Self {
        if widget.is_inherited_widget() {
            Inheritance::Inheritor {
                active_inheritors: inherited_ancestor
                    .borrow()
                    .inheritance
                    .active_inheritors()
                    .clone(),
                inheriting_widgets: HashSet::new(),
            }
        } else {
            Inheritance::Inheritee {
                inherited_ancestor: inherited_ancestor.clone(),
                inherits_from: HashSet::new(),
            }
        }
    }

    fn active_inheritors(&self) -> &HashMap<TypeId, WidgetNodeRef> {
        match self {
            Self::Inheritor {
                active_inheritors, ..
            } => active_inheritors,
            Self::Inheritee { .. } => unreachable!(),
        }
    }

    fn inherited_ancestor(&self, self_node: &WidgetNodeRef) -> WidgetNodeRef {
        match self {
            Inheritance::Inheritor { .. } => self_node.clone(),
            Inheritance::Inheritee {
                inherited_ancestor, ..
            } => inherited_ancestor.clone(),
        }
    }

    fn inheriting_widgets(&mut self) -> &mut HashSet<WidgetNodeRef> {
        match self {
            Inheritance::Inheritor {
                inheriting_widgets, ..
            } => inheriting_widgets,
            Inheritance::Inheritee { .. } => unreachable!(),
        }
    }
}

impl Default for Inheritance {
    fn default() -> Self {
        Inheritance::Inheritor {
            active_inheritors: HashMap::new(),
            inheriting_widgets: HashSet::new(),
        }
    }
}

// Helpers.

trait FindKey {
    fn find_key(&mut self, key: &WidgetPtr) -> Option<usize>;
}

impl FindKey for Vec<Option<UnsafeCell<Box<WidgetNode>>>> {
    fn find_key(&mut self, key: &WidgetPtr) -> Option<usize> {
        if let None = key.raw().local_key() {
            return None;
        }

        self.iter_mut()
            .enumerate()
            .find(|(_, w)| match w {
                Some(w) => match WidgetNode::local_key(w) {
                    Some(k) => k == key.raw().local_key().unwrap(),
                    None => false,
                },
                None => false,
            })
            .map(|(n, _)| n)
    }
}

impl WidgetNode {
    fn default() -> UnsafeCell<Box<Self>> {
        // We enforce correct lifetime of each pointer manually.
        let widget_ptr =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(().into_widget_ptr()) };

        let mut this = UnsafeCell::new(Box::new(WidgetNode {
            widget_ptr: widget_ptr.clone(),
            context: RawBuildCtx {
                node: WidgetNodeRef {
                    is_alive: Rc::new(Cell::new(true)),
                    ptr: std::ptr::null_mut(),
                },
            },
            inner: RefCell::new(WidgetInner {
                dirty: false,
                state: widget_ptr.raw().create_state(),
                render_data: RenderData::new(&widget_ptr),
                inheritance: Inheritance::Inheritor {
                    active_inheritors: HashMap::new(),
                    inheriting_widgets: HashSet::new(),
                },
            }),
            parent: None,
            children: Vec::new(),
        }));

        // Safety:
        //
        // We can mutably dereference `context` because it is valid for reads and comes
        // from `context_ptr_mut` which doesn't pop stacked borrows for `this`.
        unsafe {
            // Set it inside the Context.
            (&mut *this.context_ptr_mut()).node.ptr = this.node_ptr_mut(); // <-- Override that null.
        };

        this
    }
}

/// Helper methods used to prevent popping `node.context.node.ptr` pointer tag from
/// stacked borrows when accessing other fields of [`WidgetNode`].
trait WidgetNodePtrExt {
    fn inner_ptr(&self) -> *const RefCell<WidgetInner>;
    fn widget_ptr(&self) -> *const WidgetPtr<'static>;
    fn parent_ptr(&self) -> *const Option<WidgetNodeRef>;
    fn context_ptr(&self) -> *const RawBuildCtx;
    fn children_ptr(&self) -> *const Vec<UnsafeCell<Box<WidgetNode>>>;

    fn inner_ptr_mut(&self) -> *mut RefCell<WidgetInner>;
    fn widget_ptr_mut(&self) -> *mut WidgetPtr<'static>;
    fn parent_ptr_mut(&self) -> *mut Option<WidgetNodeRef>;
    fn context_ptr_mut(&self) -> *mut RawBuildCtx;
    fn children_ptr_mut(&self) -> *mut Vec<UnsafeCell<Box<WidgetNode>>>;
}

impl WidgetNodePtrExt for *mut WidgetNode {
    fn context_ptr(&self) -> *const RawBuildCtx {
        unsafe { std::ptr::addr_of!((**self).context) }
    }

    fn widget_ptr(&self) -> *const WidgetPtr<'static> {
        unsafe { std::ptr::addr_of!((**self).widget_ptr) }
    }

    fn inner_ptr(&self) -> *const RefCell<WidgetInner> {
        unsafe { std::ptr::addr_of!((**self).inner) }
    }

    fn parent_ptr(&self) -> *const Option<WidgetNodeRef> {
        unsafe { std::ptr::addr_of!((**self).parent) }
    }

    fn children_ptr(&self) -> *const Vec<UnsafeCell<Box<WidgetNode>>> {
        unsafe { std::ptr::addr_of!((**self).children) }
    }

    fn context_ptr_mut(&self) -> *mut RawBuildCtx {
        unsafe { std::ptr::addr_of_mut!((**self).context) }
    }

    fn widget_ptr_mut(&self) -> *mut WidgetPtr<'static> {
        unsafe { std::ptr::addr_of_mut!((**self).widget_ptr) }
    }

    fn inner_ptr_mut(&self) -> *mut RefCell<WidgetInner> {
        unsafe { std::ptr::addr_of_mut!((**self).inner) }
    }

    fn parent_ptr_mut(&self) -> *mut Option<WidgetNodeRef> {
        unsafe { std::ptr::addr_of_mut!((**self).parent) }
    }

    fn children_ptr_mut(&self) -> *mut Vec<UnsafeCell<Box<WidgetNode>>> {
        unsafe { std::ptr::addr_of_mut!((**self).children) }
    }
}

/// Helper methods used to prevent popping `node.context.node.ptr` pointer tag from
/// stacked borrows when accessing other fields of [`WidgetNode`].
trait UnsafeCellWidgetNodePtrExt {
    fn node_ptr(&self) -> *const WidgetNode;
    fn inner_ptr(&self) -> *const RefCell<WidgetInner>;
    fn widget_ptr(&self) -> *const WidgetPtr<'static>;
    fn parent_ptr(&self) -> *const Option<WidgetNodeRef>;
    fn context_ptr(&self) -> *const RawBuildCtx;
    fn children_ptr(&self) -> *const Vec<UnsafeCell<Box<WidgetNode>>>;

    fn node_ptr_mut(&self) -> *mut WidgetNode;
    fn inner_ptr_mut(&mut self) -> *mut RefCell<WidgetInner>;
    fn widget_ptr_mut(&mut self) -> *mut WidgetPtr<'static>;
    fn parent_ptr_mut(&mut self) -> *mut Option<WidgetNodeRef>;
    fn context_ptr_mut(&mut self) -> *mut RawBuildCtx;
    fn children_ptr_mut(&mut self) -> *mut Vec<UnsafeCell<Box<WidgetNode>>>;
}

impl UnsafeCellWidgetNodePtrExt for UnsafeCell<Box<WidgetNode>> {
    fn node_ptr(&self) -> *const WidgetNode {
        unsafe { std::ptr::addr_of!(**self.get()) }
    }

    fn inner_ptr(&self) -> *const RefCell<WidgetInner> {
        unsafe { std::ptr::addr_of!((*self.get()).inner) }
    }

    fn widget_ptr(&self) -> *const WidgetPtr<'static> {
        unsafe { std::ptr::addr_of!((*self.get()).widget_ptr) }
    }

    fn parent_ptr(&self) -> *const Option<WidgetNodeRef> {
        unsafe { std::ptr::addr_of!((*self.get()).parent) }
    }

    fn context_ptr(&self) -> *const RawBuildCtx {
        unsafe { std::ptr::addr_of!((*self.get()).context) }
    }

    fn children_ptr(&self) -> *const Vec<UnsafeCell<Box<WidgetNode>>> {
        unsafe { std::ptr::addr_of!((*self.get()).children) }
    }

    fn node_ptr_mut(&self) -> *mut WidgetNode {
        unsafe { std::ptr::addr_of_mut!(**self.get()) }
    }

    fn inner_ptr_mut(&mut self) -> *mut RefCell<WidgetInner> {
        unsafe { std::ptr::addr_of_mut!((*self.get()).inner) }
    }

    fn widget_ptr_mut(&mut self) -> *mut WidgetPtr<'static> {
        unsafe { std::ptr::addr_of_mut!((*self.get()).widget_ptr) }
    }

    fn parent_ptr_mut(&mut self) -> *mut Option<WidgetNodeRef> {
        unsafe { std::ptr::addr_of_mut!((*self.get()).parent) }
    }

    fn context_ptr_mut(&mut self) -> *mut RawBuildCtx {
        unsafe { std::ptr::addr_of_mut!((*self.get()).context) }
    }

    fn children_ptr_mut(&mut self) -> *mut Vec<UnsafeCell<Box<WidgetNode>>> {
        unsafe { std::ptr::addr_of_mut!((*self.get()).children) }
    }
}
