use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

use druid_shell::IdleToken;

use crate::{
    api::{
        contexts::{render::LayoutCxOS, RawBuildCx},
        pointer_events::events::PointerEvent,
        IntoWidgetPtr, WidgetPtr,
    },
    app::runner::window_handler::{APP_HANDLE, NEED_REBUILD},
    macro_exports::{PaintCxOS, RawWidget},
    render::{Canvas, Constraints, Offset, Size},
};

use self::pointer_handler::PointerHandler;

pub mod pointer_handler;

pub(crate) struct WidgetTree {
    /// Node containing necessary configuration to support [`InheritedWidget`].
    /// Child of this node is the `root_node`.
    ///
    /// [`InheritedWidget`]: crate::prelude::InheritedWidget
    dummy_node: NodeRef,

    root_node: NodeRef,
    pointer_handler: PointerHandler,
}

impl WidgetTree {
    pub fn new(root_widget: WidgetPtr) -> Self {
        let dummy_node = Node::default();

        let root_node = Node::new(root_widget, None, dummy_node.clone());

        dummy_node.borrow_mut().children.push(root_node.clone());

        Self {
            root_node,
            dummy_node,
            pointer_handler: PointerHandler::default(),
        }
    }

    pub fn layout(&mut self, constraints: Constraints) {
        LayoutCxOS::new(self.root_node.clone()).layout(constraints);
    }

    pub fn paint(&mut self, piet: &mut Canvas) {
        PaintCxOS::new(self.root_node.clone()).paint(piet, &Offset::default());
    }

    pub fn handle_pointer_event(&mut self, event: PointerEvent) {
        self.pointer_handler
            .handle_pointer_event(self.root_node.clone(), event)
    }
}

impl Drop for WidgetTree {
    fn drop(&mut self) {
        self.dummy_node.drop();
    }
}

impl Default for WidgetTree {
    fn default() -> Self {
        WidgetTree::new(().into_widget_ptr())
    }
}

pub(crate) struct NodeInner {
    pub is_alive: Rc<Cell<*mut Node>>,

    widget_ptr: WidgetPtr<'static>,
    parent: Option<NodeRef>,
    children: Vec<NodeRef>,

    pub dirty: bool,
    pub state: Box<dyn Any>,
    pub render_data: RenderData,
    pub inheritance: Inheritance,
}

pub(crate) struct Node {
    pub inner: RefCell<NodeInner>,
}

impl Node {
    fn new(widget: WidgetPtr, parent: Option<NodeRef>, mut inherited_ancestor: NodeRef) -> NodeRef {
        // We manage lifetime of widgets manually.
        let widget_ptr =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(widget.clone()) };

        let node = Box::into_raw(Box::new(Node {
            inner: RefCell::new(NodeInner {
                is_alive: Rc::new(Cell::new(std::ptr::null_mut())),
                widget_ptr,
                parent,
                children: Vec::new(),
                dirty: false,
                state: widget.raw().create_state(),
                render_data: RenderData::new(widget.raw()),
                inheritance: Inheritance::new(&widget, &inherited_ancestor),
            }),
        }));

        let node_ref = {
            let is_alive = unsafe { (&*node).inner.borrow().is_alive.clone() };

            is_alive.set(node); // <- Here we override that null

            NodeRef { ptr: is_alive }
        };

        //
        // Insert this node to `active_inheritors`.

        if let Inheritance::Inheritor {
            ref mut active_inheritors,
            ..
        } = node_ref.borrow_mut().inheritance
        {
            inherited_ancestor = node_ref.clone();
            active_inheritors.insert(widget.inherited_key(), node_ref.clone());
        }

        //
        // Mount state.

        node_ref.mount();

        //
        // Build children.

        // From this point on, `Node` cannot be accessed mutably until any
        // references to it are gone (one such reference is now `BuildCx`).

        let cx = unsafe { std::mem::transmute::<*mut Node, &RawBuildCx>(node) };

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

        node_ref.borrow_mut().children = children;

        node_ref
    }
}

/// Basically `Rc<Node>` but allows [`Node`] to be deallocated separately from
/// [`NodeRef`] itself (`*mut Node == NULL` case) which is used to optimize
/// rebuilds.
#[derive(Clone)]
pub struct NodeRef {
    pub(crate) ptr: Rc<Cell<*mut Node>>,
}

impl NodeRef {
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

    pub fn is_alive(&self) -> bool {
        !self.ptr.get().is_null()
    }

    #[track_caller]
    pub(crate) fn widget(&self) -> &dyn RawWidget {
        assert!(self.is_alive());
        self.borrow().widget_ptr.kind
    }

    #[track_caller]
    pub(crate) fn child(&self, n: usize) -> Option<NodeRef> {
        assert!(self.is_alive());
        self.borrow().children.get(n).map(|v| v.clone())
    }

    #[track_caller]
    pub(crate) fn children(&self) -> Vec<NodeRef> {
        assert!(self.is_alive());
        self.borrow().children.clone()
    }

    #[allow(unused)]
    #[track_caller]
    pub fn debug_name_short(&self) -> &'static str {
        self.widget().debug_name_short()
    }
}

/// Tree editing methods.

impl NodeRef {
    /// Update subtree by rebuilding, starting at this node.
    pub fn update_subtree(&self) {
        assert!(self.is_alive());

        // Todo: In the calling function, check that given WidgetNode of passed
        // `widget` is indeed `dirty`, as it may have been already updated by
        // previous call to this function.
        self.borrow_mut().dirty = false;

        let inherited_ancestor = &self.borrow().inheritance.inherited_ancestor(self);

        let mut old_children = std::mem::take(&mut self.borrow_mut().children)
            .into_iter()
            .map(|c| Some(c))
            .collect::<Vec<_>>();

        let cx = unsafe { std::mem::transmute::<*mut Node, &RawBuildCx>(self.ptr.get()) };
        let new_children_build = self.widget().build(cx);
        let mut new_children = Vec::with_capacity(new_children_build.len());

        for (n, new_child) in new_children_build.into_iter().enumerate() {
            if new_child.has_key() {
                if let Some(n) = old_children.find_key(&new_child) {
                    // Remove old_child from old_children.
                    let old_child = std::mem::take(old_children.get_mut(n).unwrap()).unwrap();

                    // Try to update old_child with new_child.
                    new_children.push(old_child.update(new_child));
                } else {
                    // Build new_child.
                    let child =
                        Node::new(new_child, Some(self.clone()), inherited_ancestor.clone());

                    new_children.push(child);
                }
            } else {
                if let Some(Some(old_child)) = old_children.get(n) {
                    let old_widget_ptr = old_child.borrow().widget_ptr.clone();

                    if old_widget_ptr.has_key() {
                        // Build new_child.
                        let child =
                            Node::new(new_child, Some(self.clone()), inherited_ancestor.clone());

                        new_children.push(child);
                    } else {
                        // Remove old_child from old_children.
                        let old_child = std::mem::take(old_children.get_mut(n).unwrap()).unwrap();

                        // Try to update old_child with new_child.
                        new_children.push(old_child.update(new_child));
                    }
                } else {
                    // Build new_child.
                    let child =
                        Node::new(new_child, Some(self.clone()), inherited_ancestor.clone());

                    new_children.push(child);
                }
            }
        }

        // Drop children which didn't get reused.
        for old_child in old_children.into_iter() {
            if let Some(old_child) = old_child {
                old_child.drop();
            }
        }

        // Update children keys.
        self.borrow_mut().children = new_children;
    }

    pub fn update(&self, new_widget: WidgetPtr) -> NodeRef {
        assert!(self.is_alive());

        // We manage lifetime of widgets manually.
        let new_widget =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(new_widget) };

        let old_widget = self.borrow().widget_ptr.clone();

        // If widgets share the same generic-independent TypeId and types of states of both
        // widgets match (most likely a bug if they don't) we preserve that widget's state.
        if old_widget.can_update(&new_widget) {
            // Safety:
            //
            // We compare and reuse configurations of widgets to optimize unnecessary view
            // rebuilds if old and new widgets are identical.
            //
            // This operation must be performed with special scrutiny, since preserving
            // pointers to widgets which were dropped earlier will cause use-after-free.
            //
            // For that reason the following `eq` method uses our own `CheapEq`
            // implementation which ensures that each widget is correctly compared (that
            // includes fields which are references/pointers).
            //
            // To be exact, we can reuse widgets if they either:
            // - 'static, or
            // - non-'static, but then every reference and pointer in such widget's
            //   structure has to have identical pointer addresses.
            //
            // Note/Todo:
            //
            // We can reuse "non-'static" widgets of different pointer addresses if those
            // are ZST. However those ZST-s may point to deallocated memory (and it is
            // considered UB to dereference such pointers, even if they point to ZST). To
            // work around this, we can use `NonNull::dangling()` (which will effectively
            // return 0x1 pointer address which is ?always safe to dereference for ZST-s?).
            // Then we just need to update pointer address in `&dyn RawWidget` with that
            // above and then we will be able optimize ZST rebuilds.
            if old_widget.eq(&new_widget) {
                // Safety: Since we reuse old widget ptr, we can drop the newly created one.
                unsafe { WidgetPtr::drop(&new_widget) };

                return self.clone();
            } else {
                // Unmount old widget.
                self.unmount();

                // Safety:
                //
                // Old pointer is still alive (and didn't move, since it is either boxed or
                // it points to a value in ascenstor widget), so it is fine to unmount/drop
                // children of this widget (in `update_subtree` below).
                let old_widget_ptr =
                    std::mem::replace(&mut self.borrow_mut().widget_ptr, new_widget);

                // Update descendants of this node, stopping at equal widget configurations
                // or a leaf node.
                self.update_subtree();

                // Safety:
                //
                // There are no children widgets referencing old pointer (because we dropped
                // them all in `update_subtree` above), so we can safely drop it.
                unsafe { WidgetPtr::drop(&old_widget_ptr) };

                // Mount updated widget.
                self.mount();

                return self.clone();
            }
        } else {
            let parent = self.borrow().parent.clone();

            let inherited_ancestor = parent
                .as_ref()
                .unwrap()
                .borrow()
                .inheritance
                .inherited_ancestor(self);

            // Unmount and drop subtree starting at this node.
            self.drop();

            // Build new subtree in its place.
            return Node::new(new_widget, parent, inherited_ancestor);
        }
    }

    pub fn mount(&self) {
        assert!(self.is_alive());

        let widget = self.borrow().widget_ptr.clone();
        let context = unsafe { std::mem::transmute::<*mut Node, &RawBuildCx>(self.ptr.get()) };

        widget.mount(context)
    }

    pub fn unmount(&self) {
        assert!(self.is_alive());

        let widget = self.borrow().widget_ptr.clone();
        let context = unsafe { std::mem::transmute::<*mut Node, &RawBuildCx>(self.ptr.get()) };

        widget.mount(context)
    }

    /// Drop this widget [`Node`] and all its descendants.
    pub fn drop(&self) {
        assert!(self.is_alive());

        //
        // Unmount state.

        self.unmount();

        //
        // Deallocate children.

        let mut children = std::mem::take(&mut self.borrow_mut().children).into_iter();

        while let Some(child) = children.next() {
            child.drop();
        }

        //
        // Remove this widget from inherited widgets it depends on.

        if let Inheritance::Inheritee { inherits_from, .. } = &self.borrow().inheritance {
            for inheritor in inherits_from.iter() {
                let mut inheritor_ref = inheritor.borrow_mut();
                let inheriting_widgets = inheritor_ref.inheritance.inheriting_widgets();
                inheriting_widgets.remove(self);
            }
        }

        //
        // Drop `widget_ptr` and the `node` itself.

        unsafe {
            let widget_ptr = self.borrow().widget_ptr.clone();
            WidgetPtr::drop(&widget_ptr);

            let node_ptr = self.ptr.get();
            drop(Box::from_raw(node_ptr));

            //
            // From this point on `RawBuildCx` should not be accessed again.
        }

        //
        // Disable this widget.

        self.ptr.set(std::ptr::null_mut());
    }
}

/// Used by API.

impl NodeRef {
    pub fn mark_dirty(&self) {
        assert!(self.is_alive());

        APP_HANDLE.with(|handle| {
            handle
                .borrow_mut()
                .as_mut()
                .expect("APP_HANDLE wasn't set")
                .schedule_idle(IdleToken::new(0));
        });

        if !self.borrow().dirty {
            self.borrow_mut().dirty = true;

            NEED_REBUILD.with(|dirty| {
                dirty.lock().unwrap().push(self.clone());
            });
        }
    }

    pub fn mark_dependent_widgets_as_dirty(&self) {
        assert!(self.is_alive());

        if let Inheritance::Inheritor {
            inheriting_widgets, ..
        } = &self.borrow().inheritance
        {
            for widget in inheriting_widgets.iter() {
                widget.mark_dirty()
            }
        } else {
            unreachable!()
        }
    }

    pub fn depend_on_inherited_widget_of_key<'a, K>(&'a self) -> Option<NodeRef>
    where
        K: 'static,
    {
        assert!(self.is_alive());

        let key = TypeId::of::<K>();

        let mut inner = self.borrow_mut();

        let (inherited_ancestor, inherits_from) = match &mut inner.inheritance {
            Inheritance::Inheritee {
                inherited_ancestor,
                inherits_from,
            } => (inherited_ancestor, inherits_from),
            _ => unreachable!(),
        };

        let inherited_widget = {
            let inherited_ref = inherited_ancestor.borrow();

            // All inherited widgets accessible from this node.
            let active_inheritors = inherited_ref.inheritance.active_inheritors();

            // Closest inhertied ancestor of type K.
            active_inheritors.get(&key)?.clone()
        };

        let mut inherited_widget_ref = inherited_widget.borrow_mut();

        let inheriting_widgets = inherited_widget_ref.inheritance.inheriting_widgets();

        // Register this node in InheritedWidget.
        inheriting_widgets.insert(self.clone());

        // Remember which InheritedWidgets we are inheriting from.
        inherits_from.insert(inherited_widget.clone());

        Some(inherited_widget.clone())
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

pub(crate) struct RenderData {
    /// Render state used by this widget.
    pub state: Box<dyn Any>,
    /// Data that can be accessed from parent widget.
    pub parent_data: Box<dyn Any>,

    /// Size computed during last layout.
    pub size: Size,
    /// Offset received during last paint.
    pub local_offset: Offset,
    /// Incoming constraints received during last layout.
    pub constraints: Constraints,

    /// Whether child was laid out. Used to display an error message when
    /// painting children without laying them out beforehand.
    pub laid_out: bool,
}

impl RenderData {
    fn new(widget: &dyn RawWidget) -> RenderData {
        RenderData {
            state: widget.create_render_state(),
            parent_data: widget.create_parent_data(),
            size: Size::default(),
            local_offset: Offset::default(),
            constraints: Constraints::default(),
            laid_out: false,
        }
    }
}

/// Whether this node *inherits-from* [`InheritedWidget`] ([`Inheritee`] ) or
/// *is* an [`InheritedWidget`] itself ([`Inheritor`]).
///
/// [`Inheritor`]: Inheritance::Inheritor
/// [`Inheritee`]: Inheritance::Inheritee
/// [`InheritedWidget`]: crate::prelude::InheritedWidget
pub(crate) enum Inheritance {
    Inheritor {
        /// All [`InheritedWidget`](crate::prelude::InheritedWidget) ancestors that are
        /// accessible from this node (that are higher in the tree).
        active_inheritors: HashMap<TypeId, NodeRef>,
        /// All descendant widgets which inherit from this widget.
        inheriting_widgets: HashSet<NodeRef>,
    },
    Inheritee {
        /// This is the closest [`InheritedWidget`](crate::prelude::InheritedWidget)
        /// ancestor of this widget.
        inherited_ancestor: NodeRef,
        /// This is used to efficiently unregister from all inherited ancestors
        /// ancestors.
        inherits_from: HashSet<NodeRef>,
    },
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

//
// Helpers.

trait FindKey {
    /// `key` must be valid as well as pointers in self`.
    fn find_key(&mut self, key: &WidgetPtr) -> Option<usize>;
}

impl FindKey for Vec<Option<NodeRef>> {
    fn find_key(&mut self, key: &WidgetPtr) -> Option<usize> {
        if let None = key.raw().local_key() {
            return None;
        }

        self.iter_mut()
            .enumerate()
            .find(|(_, w)| match w {
                Some(w) => {
                    let node = w.widget();
                    let local_key_any = node.local_key();

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
    fn default() -> NodeRef {
        // We manage lifetime of widgets manually.
        let widget_ptr =
            unsafe { std::mem::transmute::<WidgetPtr, WidgetPtr<'static>>(().into_widget_ptr()) };

        let this = Box::into_raw(Box::new(Node {
            inner: RefCell::new(NodeInner {
                is_alive: Rc::new(Cell::new(std::ptr::null_mut())),
                widget_ptr: widget_ptr.clone(),
                parent: None,
                children: Vec::new(),
                dirty: false,
                state: widget_ptr.raw().create_state(),
                render_data: RenderData::new(widget_ptr.raw()),
                inheritance: Inheritance::Inheritor {
                    active_inheritors: HashMap::new(),
                    inheriting_widgets: HashSet::new(),
                },
            }),
        }));

        unsafe {
            let is_alive = (&*this).inner.borrow().is_alive.clone();

            is_alive.set(this); // <- Here we override that null.

            NodeRef { ptr: is_alive }
        }
    }
}
