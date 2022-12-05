use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

use druid_shell::IdleToken;

use crate::{
    api::{
        contexts::{render::LayoutCtxOS, RawBuildCtx},
        pointer_events::events::PointerEvent,
        IntoWidgetPtr, WidgetPtr,
    },
    app::runner::window_handler::{APP_HANDLE, NEED_REBUILD},
    macro_exports::{PaintCtxOS, RawWidget},
    render::{Canvas, Constraints, Offset, Size},
};

use self::pointer_handler::PointerHandler;

pub mod pointer_handler;

pub(crate) struct WidgetTree {
    /// Node containing necessary configuration to support [`InheritedWidget`].
    /// Child of this node is the `root_node`.
    ///
    /// [`InheritedWidget`]: crate::prelude::InheritedWidget
    dummy_node: *mut Node,

    root_node: *mut Node,
    pointer_handler: PointerHandler,
}

impl WidgetTree {
    pub fn new(root_widget: WidgetPtr) -> Self {
        let dummy_node = Node::default();

        let root_node = unsafe {
            let root = Node::new(root_widget, None, NodeRef::new(dummy_node));

            (&*root).inner.borrow_mut().children.push(root);

            root
        };

        Self {
            root_node,
            dummy_node,
            pointer_handler: PointerHandler::default(),
        }
    }

    pub fn layout(&mut self, constraints: Constraints) {
        LayoutCtxOS::new(self.root_node()).layout(constraints);
    }

    pub fn paint(&mut self, piet: &mut Canvas) {
        PaintCtxOS::new(self.root_node()).paint(piet, &Offset::default());
    }

    pub fn handle_pointer_event(&mut self, event: PointerEvent) {
        self.pointer_handler
            .handle_pointer_event(self.root_node(), event)
    }

    fn root_node(&self) -> NodeRef {
        unsafe { NodeRef::new(self.root_node) }
    }
}

impl Default for WidgetTree {
    fn default() -> Self {
        WidgetTree::new(().into_widget_ptr())
    }
}

impl Drop for WidgetTree {
    fn drop(&mut self) {
        // Safety: `drop` is called only once here.
        unsafe { Node::drop(self.dummy_node) };
    }
}

pub(crate) enum Inheritance {
    /// This is the `InheritedWidget`.
    Inheritor {
        /// This `HashMap` contains all `InheritedWidget` ancestors that are accessible
        /// from this subtree.
        active_inheritors: HashMap<TypeId, NodeRef>,
        /// This `HashSet` contains all descendant widgets which inherit from this widget.
        inheriting_widgets: HashSet<NodeRef>,
    },
    /// This is any widget that inherits from `InheritedWidget` ancestor.
    Inheritee {
        /// This is the closest `InheritedWidget` ancestor of this widget.
        inherited_ancestor: NodeRef,
        /// This is used to efficiently unregister this widget from all inherited ancestors.
        inherits_from: HashSet<NodeRef>,
    },
}

pub(crate) struct NodeInner {
    is_alive: Arc<Cell<*mut Node>>,

    widget_ptr: WidgetPtr<'static>,
    parent: Option<NodeRef>,
    children: Vec<*mut Node>,

    pub dirty: bool,
    pub state: Box<dyn Any>,
    pub render_data: RenderData,
    pub inheritance: Inheritance,
}

pub(crate) struct Node {
    inner: RefCell<NodeInner>,
}

impl Node {
    /// Build a subtree of widgets from given `widget`.
    ///
    /// Safety: `widget` must be valid.
    unsafe fn new(
        widget: WidgetPtr,
        parent: Option<NodeRef>,
        mut inherited_ancestor: NodeRef,
    ) -> *mut Node {
        // We manage lifetime of widgets manually.
        let widget_ptr = std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(widget.clone());

        let node = Box::into_raw(Box::new(Node {
            inner: RefCell::new(NodeInner {
                is_alive: Arc::new(Cell::new(std::ptr::null_mut())),
                widget_ptr,
                parent,
                children: Vec::new(),
                dirty: false,
                state: widget.raw().create_state(),
                render_data: RenderData::new(&widget),
                inheritance: Inheritance::new(&widget, &inherited_ancestor),
            }),
        }));

        let inner = &mut *(&*node).inner.borrow_mut();

        inner.is_alive.set(node); // <- Here we override that null

        let node_ref = NodeRef::new(node);

        //
        // Insert this node to `active_inheritors`.

        if let Inheritance::Inheritor {
            ref mut active_inheritors,
            ..
        } = inner.inheritance
        {
            inherited_ancestor = node_ref.clone();
            active_inheritors.insert(widget.inherited_key(), node_ref.clone());
        }

        //
        // Mount state.

        Node::mount(node);

        //
        // Build children.

        // From this point on, `Node` cannot be accessed mutably until any
        // references to it are gone (one such reference is now `BuildCx`).

        let cx = std::mem::transmute::<*mut Node, &RawBuildCtx>(node);

        let children = widget
            .build(cx)
            .into_iter()
            .map(|child_widget_ptr| {
                Node::new(
                    child_widget_ptr,
                    Some(node_ref.clone()),
                    inherited_ancestor.clone(),
                )
            })
            .collect::<Vec<_>>();

        inner.children = children;

        node
    }

    /// Update subtree by rebuilding, starting at this node.
    ///
    /// Safety: `s` must be valid.
    pub unsafe fn update_subtree(s: *mut Node) {
        let rc = NodeRef::new(s);
        let inner = &mut (&*s).inner.borrow_mut();

        // Todo: In the calling function, check that given WidgetNode of passed `widget`
        // is indeed `dirty`, as it may have been already updated by a previous call to
        // this function.
        inner.dirty = false;

        let inherited_ancestor = &inner.inheritance.inherited_ancestor(&rc);
        let cx = std::mem::transmute::<*mut Node, &RawBuildCtx>(s);

        let mut old_children = std::mem::take(&mut inner.children)
            .into_iter()
            .map(|c| Some(c))
            .collect::<Vec<_>>();
        let new_children_build = inner.widget_ptr.build(cx);
        let mut new_children = Vec::with_capacity(new_children_build.len());

        for (n, new_child) in new_children_build.into_iter().enumerate() {
            if new_child.has_key() {
                if let Some(n) = old_children.find_key(&new_child) {
                    // Remove old_child from old_children.
                    let old_child = std::mem::take(old_children.get_mut(n).unwrap()).unwrap();

                    // Try to update old_child with new_child.
                    new_children.push(Node::update(old_child, new_child));
                } else {
                    // Build new_child.
                    let child = Node::new(new_child, Some(rc.clone()), inherited_ancestor.clone());

                    new_children.push(child);
                }
            } else {
                if let Some(Some(old_child)) = old_children.get(n) {
                    let old_widget_ptr = (&**old_child).inner.borrow().widget_ptr.clone();

                    if old_widget_ptr.has_key() {
                        // Build new_child.
                        let child =
                            Node::new(new_child, Some(rc.clone()), inherited_ancestor.clone());

                        new_children.push(child);
                    } else {
                        // Remove old_child from old_children.
                        let old_child = std::mem::take(old_children.get_mut(n).unwrap()).unwrap();

                        // Try to update old_child with new_child.
                        new_children.push(Node::update(old_child, new_child));
                    }
                } else {
                    // Build new_child.
                    let child = Node::new(new_child, Some(rc.clone()), inherited_ancestor.clone());

                    new_children.push(child);
                }
            }
        }

        // Drop children which didn't get reused.
        for old_child in old_children.into_iter() {
            if let Some(old_child) = old_child {
                Node::drop(old_child);
            }
        }

        // Update children keys.
        inner.children = new_children;
    }

    /// Safety:
    ///
    /// `s` and `new_widget` must be valid.
    pub unsafe fn update(s: *mut Node, new_widget: WidgetPtr) -> *mut Node {
        // We manage lifetime of widgets manually.
        let new_widget = std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(new_widget);

        let rc = NodeRef::new(s);
        let inner = &mut *(&*s).inner.borrow_mut();

        let old_widget = inner.widget_ptr.clone();

        // If widgets share the same generic-independent TypeId and types of states of both
        // widgets match (most likely a bug if they don't) we preserve that widget's state.
        if old_widget.can_update(&new_widget) {
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
            if old_widget.eq(&new_widget) {
                // Safety: Since we reuse old widget ptr, we can drop the newly created one.
                WidgetPtr::drop(new_widget);

                return s;
            } else {
                // Unmount old widget.
                Node::unmount(s);

                // Safety:
                //
                // Old pointer is still alive (and didn't move, since it is either boxed or
                // it points to a value in ascenstor widget), so it is fine to unmount/drop
                // children of this widget (in `update_subtree` below).
                //
                // Additionally, we don't access `widget_ref` after this call, so we don't
                // cause aliasing UB.
                let old_widget_ptr = std::mem::replace(&mut inner.widget_ptr, new_widget);

                // Update descendants of this node, stopping at equal widgets or a leaf node.
                Node::update_subtree(s);

                // Safety:
                //
                // There are no children widgets referencing the old pointer (because we
                // updated them all in `update_subtree` above), so we can safely drop it.
                WidgetPtr::drop(old_widget_ptr);

                // Mount updated widget.
                Node::mount(s);

                return s;
            }
        } else {
            let parent = inner.parent.clone();

            // We use the parent's inherited ancestor, since `s` could be an InheritedWidget
            // itself. After we deallocate `s` this would cause use after free.
            let inherited_ancestor = parent
                .as_ref()
                .unwrap()
                .borrow()
                .inheritance
                .inherited_ancestor(&rc);

            // Unmount and drop subtree starting at this node.
            Node::drop(s);

            // Build new subtree in its place.
            return Node::new(new_widget, parent, inherited_ancestor);
        }
    }

    /// Drop this widget node and all its descendants.
    ///
    /// # Safety
    ///
    /// This function can only be called once on a given [`WidgetNode`] and `s`
    /// must be valid.
    pub unsafe fn drop(s: *mut Node) {
        let inner = &mut *(&*s).inner.borrow_mut();

        //
        // Unmount state.

        Node::unmount(s);

        //
        // Deallocate children.

        let children_ref_mut = &mut inner.children;

        let mut children = std::mem::take(children_ref_mut).into_iter();

        while let Some(child) = children.next() {
            Node::drop(child);
        }

        //
        // Remove this widget from inherited widgets it depends on.

        match &inner.inheritance {
            Inheritance::Inheritee {
                inherited_ancestor: _,
                inherits_from,
            } => {
                let node_ref = &NodeRef::new(s);

                for inheritor in inherits_from.iter() {
                    let mut inheritor_ref = inheritor.borrow_mut();
                    let inheriting_widgets = inheritor_ref.inheritance.inheriting_widgets();
                    inheriting_widgets.remove(node_ref);
                }
            }
            Inheritance::Inheritor { .. } => {}
        };

        inner.is_alive.set(std::ptr::null_mut());

        //
        // Drop `widget_ptr` and the `node` itself.

        // From this point on we `Context` will not be accessed again immutably,
        // thus we can access whole `Self` mutably.
        let s = &mut *s;

        // Drop widget structure.
        //
        // Inlined `WidgetPtr::drop`.
        if let Some(ptr) = inner.widget_ptr.owned {
            drop(Box::from_raw(ptr));
        }

        // Drop this node.
        drop(Box::from_raw(s));
    }

    //
    //

    /// Safety: `s` must be valid.
    pub unsafe fn mount(s: *mut Node) {
        let widget = (&*s).inner.borrow().widget_ptr.clone();
        let context = std::mem::transmute::<*mut Node, &RawBuildCtx>(s);

        widget.mount(context)
    }

    /// Safety: `s` must be valid.
    pub unsafe fn unmount(s: *mut Node) {
        let widget = (&*s).inner.borrow().widget_ptr.clone();
        let context = std::mem::transmute::<*mut Node, &RawBuildCtx>(s);

        widget.unmount(context)
    }
}

#[derive(Clone)]
pub struct NodeRef {
    ptr: Arc<Cell<*mut Node>>,
}

impl NodeRef {
    pub(crate) unsafe fn new(s: *mut Node) -> NodeRef {
        Self {
            ptr: (&*s).inner.borrow().is_alive.clone(),
        }
    }

    #[track_caller]
    pub(crate) fn borrow(&self) -> Ref<'_, NodeInner> {
        assert!(self.is_alive());
        unsafe { (&*self.ptr.get()).inner.borrow() }
    }

    #[track_caller]
    pub(crate) fn borrow_mut(&self) -> RefMut<'_, NodeInner> {
        assert!(self.is_alive());
        unsafe { (&*self.ptr.get()).inner.borrow_mut() }
    }

    //

    pub fn update_subtree(&self) {
        assert!(self.is_alive());
        unsafe { Node::update_subtree(self.ptr.get()) }
    }

    pub fn mark_dirty(&self) {
        assert!(self.is_alive());

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
        assert!(self.is_alive());

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

    pub fn depend_on_inherited_widget_of_key<'a, K>(&'a self) -> Option<NodeRef>
    where
        K: 'static,
    {
        assert!(self.is_alive());

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
        !self.ptr.get().is_null()
    }

    #[track_caller]
    pub fn widget<'a>(&'a self) -> Ref<'a, dyn RawWidget + 'a> {
        assert!(self.is_alive());

        // let a: WidgetPtr<'static> = self.borrow().widget_ptr.clone();
        // let a: WidgetPtr<'a> = a;

        Ref::map(self.borrow(), |v| v.widget_ptr.kind)
    }

    #[track_caller]
    pub(crate) fn child(&self, n: usize) -> Option<*mut Node> {
        assert!(self.is_alive());
        self.borrow().children.get(n).map(|v| *v)
    }

    #[track_caller]
    pub(crate) fn children(&self) -> Vec<*mut Node> {
        assert!(self.is_alive());
        self.borrow().children.clone()
    }

    #[allow(unused)]
    pub fn debug_name_short(&self) -> &'static str {
        self.widget().debug_name_short()
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        self.ptr.as_ptr() == other.ptr.as_ptr()
    }
}

impl Eq for NodeRef {}

impl Hash for NodeRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.as_ptr().hash(state);
    }
}

impl std::fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_alive() {
            write!(f, "WidgetNodeRef ({})", self.debug_name_short())
        } else {
            f.debug_struct("WidgetNodeRef (removed)")
                .field("ptr", &self.ptr)
                .finish()
        }
    }
}

struct RawRef<'a, T: ?Sized> {
    ptr: Ref<'a, T>,
    // _p: PhantomData<&'a ()>,
}

impl<'a, T: ?Sized> std::ops::Deref for RawRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.ptr.deref()
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
    fn new(widget: &WidgetPtr, inherited_ancestor: &NodeRef) -> Self {
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

    fn active_inheritors(&self) -> &HashMap<TypeId, NodeRef> {
        match self {
            Self::Inheritor {
                active_inheritors, ..
            } => active_inheritors,
            Self::Inheritee { .. } => unreachable!(),
        }
    }

    fn inherited_ancestor(&self, self_node: &NodeRef) -> NodeRef {
        match self {
            Inheritance::Inheritor { .. } => self_node.clone(),
            Inheritance::Inheritee {
                inherited_ancestor, ..
            } => inherited_ancestor.clone(),
        }
    }

    fn inheriting_widgets(&mut self) -> &mut HashSet<NodeRef> {
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
    /// `key` must be valid as well as pointers in self`.
    unsafe fn find_key(&mut self, key: &WidgetPtr) -> Option<usize>;
}

impl FindKey for Vec<Option<*mut Node>> {
    unsafe fn find_key(&mut self, key: &WidgetPtr) -> Option<usize> {
        if let None = key.raw().local_key() {
            return None;
        }

        self.iter_mut()
            .enumerate()
            .find(|(_, w)| match w {
                Some(w) => {
                    let node = (&**w).inner.borrow();
                    let local_key_any = node.widget_ptr.raw().local_key();

                    match local_key_any {
                        Some(k) => k == key.raw().local_key().unwrap(),
                        None => false,
                    }
                }
                None => false,
            })
            .map(|(n, _)| n)
    }
}

impl Node {
    fn default() -> *mut Node {
        // We manage lifetime of widgets manually.
        let widget_ptr =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(().into_widget_ptr()) };

        let this = Box::into_raw(Box::new(Node {
            inner: RefCell::new(NodeInner {
                is_alive: Arc::new(Cell::new(std::ptr::null_mut())),
                widget_ptr: widget_ptr.clone(),
                parent: None,
                children: Vec::new(),
                dirty: false,
                state: widget_ptr.raw().create_state(),
                render_data: RenderData::new(&widget_ptr),
                inheritance: Inheritance::Inheritor {
                    active_inheritors: HashMap::new(),
                    inheriting_widgets: HashSet::new(),
                },
            }),
        }));

        unsafe {
            // Set it inside the Context.
            (&*this).inner.borrow().is_alive.set(this); // <- Here we override that null.
        };

        this
    }
}

// /// Helper methods used to prevent popping `node.context.node.ptr` pointer tag from
// /// stacked borrows when accessing other fields of [`WidgetNode`].
// trait WidgetNodePtrExt {
//     fn inner_ptr(&self) -> *const RefCell<NodeInner>;
//     fn widget_ptr(&self) -> *const WidgetPtr<'static>;
//     fn parent_ptr(&self) -> *const Option<NodeRef>;
//     fn context_ptr(&self) -> *const RawBuildCtx;
//     fn children_ptr(&self) -> *const Vec<*mut Node>;

//     fn inner_ptr_mut(&self) -> *mut RefCell<NodeInner>;
//     fn widget_ptr_mut(&self) -> *mut WidgetPtr<'static>;
//     fn parent_ptr_mut(&self) -> *mut Option<NodeRef>;
//     fn context_ptr_mut(&self) -> *mut RawBuildCtx;
//     fn children_ptr_mut(&self) -> *mut Vec<*mut Node>;
// }

// impl WidgetNodePtrExt for *mut Node {
//     fn context_ptr(&self) -> *const RawBuildCtx {
//         unsafe { std::ptr::addr_of!((**self).context) }
//     }

//     fn widget_ptr(&self) -> *const WidgetPtr<'static> {
//         unsafe { std::ptr::addr_of!((**self).widget_ptr) }
//     }

//     fn inner_ptr(&self) -> *const RefCell<NodeInner> {
//         unsafe { std::ptr::addr_of!((**self).inner) }
//     }

//     fn parent_ptr(&self) -> *const Option<NodeRef> {
//         unsafe { std::ptr::addr_of!((**self).parent) }
//     }

//     fn children_ptr(&self) -> *const Vec<*mut Node> {
//         unsafe { std::ptr::addr_of!((**self).children) }
//     }

//     fn context_ptr_mut(&self) -> *mut RawBuildCtx {
//         unsafe { std::ptr::addr_of_mut!((**self).context) }
//     }

//     fn widget_ptr_mut(&self) -> *mut WidgetPtr<'static> {
//         unsafe { std::ptr::addr_of_mut!((**self).widget_ptr) }
//     }

//     fn inner_ptr_mut(&self) -> *mut RefCell<NodeInner> {
//         unsafe { std::ptr::addr_of_mut!((**self).inner) }
//     }

//     fn parent_ptr_mut(&self) -> *mut Option<NodeRef> {
//         unsafe { std::ptr::addr_of_mut!((**self).parent) }
//     }

//     fn children_ptr_mut(&self) -> *mut Vec<*mut Node> {
//         unsafe { std::ptr::addr_of_mut!((**self).children) }
//     }
// }
