use std::any::{Any, TypeId};

use crate::prelude::{Constraints, Offset, PaintContext, Size};

use self::{
    contexts::{build_ctx::STATE_UPDATE_SUPRESSED, render_ctx::AnyRenderContext, Context},
    events::Event,
    implementors::{
        inherited::InheritedWidgetOS, leaf::LeafWidgetOS, multi::MultiChildWidgetOS,
        single::SingleChildWidgetOS, view::ViewWidgetOS,
    },
    local_key::LocalKeyAny,
};

pub mod contexts;
pub mod events;
pub mod implementors;
pub mod impls;
pub mod local_key;
pub mod structural_eq;

pub trait Widget: WidgetDebug {
    /// Implementation should return the same unique TypeId for given structure definition,
    /// even if that structure contains generic parameters. This is used to preserve state
    /// between generic widgets.
    fn unique_type(&self) -> TypeId;

    fn kind(&self) -> WidgetKind;
}

#[derive(Clone, Copy)]
pub enum WidgetKind<'a> {
    View(&'a (dyn ViewWidgetOS + 'a)),
    Inherited(&'a (dyn InheritedWidgetOS + 'a)),
    Leaf(&'a (dyn LeafWidgetOS + 'a)),
    SingleChild(&'a (dyn SingleChildWidgetOS + 'a)),
    MultiChild(&'a (dyn MultiChildWidgetOS + 'a)),
}

#[derive(Clone)]
pub struct WidgetPtr<'a> {
    /// Reference to the exact type of this widget. Used to properly dispatch methods.
    pub(crate) kind: WidgetKind<'a>,

    /// Whether this pointer references or owns a widget. Used to properly drop it.
    pub(crate) owned: Option<*mut (dyn Widget + 'a)>,
}

impl<'a> WidgetPtr<'a> {
    pub fn from_ref(widget: &'a (dyn Widget + 'a)) -> Self {
        Self {
            kind: widget.kind(),
            owned: None,
        }
    }

    /// # Note
    ///
    /// `widget` will not be dropped until you manually call [`WidgetPtr::drop`].
    pub fn from_owned(widget: Box<dyn Widget + 'a>) -> Self {
        // Safety:
        //
        // `widget.kind()` returns `WidgetKind` which holds a reference to the `widget`.
        // Since that reference points to a heap, we can safely extend lifetime of it to
        // the lifetime of `WidgetPtr` until `drop` is called.
        unsafe {
            Self {
                kind: std::mem::transmute::<WidgetKind, WidgetKind>(widget.kind()),
                owned: Some(Box::into_raw(widget)),
            }
        }
    }

    /// ## Safety
    ///
    /// Data referenced by this [`WidgetPtr`] didn't move.
    ///
    /// Additionally, make sure there is no other [`WidgetPtr`] that may reference
    /// this [`WidgetPtr`] after [`WidgetNode::drop`] has been called on it.
    pub unsafe fn drop(self) {
        if let Some(widget) = self.owned {
            drop(Box::from_raw(widget));
        }
    }

    /// # Note
    ///
    /// Returned [`WidgetPtr`] has erased lifetime, but its invariants must be upheld.
    pub fn build(&self, ctx: &Context) -> Vec<WidgetPtr<'static>> {
        let ptrs = match self.kind {
            WidgetKind::View(w) => vec![w.build(ctx)],
            WidgetKind::Leaf(_) => vec![],
            WidgetKind::SingleChild(w) => vec![w.build(ctx)],
            WidgetKind::MultiChild(w) => w.build(ctx),
            WidgetKind::Inherited(w) => vec![w.child()],
        };

        // Safety: Consumers of `WidgetPtr` must upheld correct invariants.
        unsafe { std::mem::transmute::<Vec<WidgetPtr>, Vec<WidgetPtr>>(ptrs) }
    }

    pub fn create_state(&self) -> Box<dyn Any> {
        match self.kind {
            WidgetKind::View(w) => w.create_state(),
            WidgetKind::Leaf(w) => w.create_state(),
            WidgetKind::SingleChild(w) => w.create_state(),
            WidgetKind::MultiChild(w) => w.create_state(),
            WidgetKind::Inherited(w) => w.create_state(),
        }
    }

    pub fn create_render_state(&self) -> Box<dyn Any> {
        match self.kind {
            WidgetKind::View(_) => Box::new(()),
            WidgetKind::Leaf(w) => w.create_render_state(),
            WidgetKind::MultiChild(w) => w.create_render_state(),
            WidgetKind::SingleChild(w) => w.create_render_state(),
            WidgetKind::Inherited(w) => w.create_render_state(),
        }
    }

    pub fn layout<'b>(
        &self,
        render_ctx: &'b mut AnyRenderContext,
        constraints: Constraints,
    ) -> Size {
        match self.kind {
            WidgetKind::View(w) => w.layout(render_ctx, constraints),
            WidgetKind::Leaf(w) => w.layout(render_ctx, constraints),
            WidgetKind::MultiChild(w) => w.layout(render_ctx, constraints),
            WidgetKind::SingleChild(w) => w.layout(render_ctx, constraints),
            WidgetKind::Inherited(w) => w.layout(render_ctx, constraints),
        }
    }

    pub fn paint<'b>(
        &self,
        render_ctx: &'b mut AnyRenderContext,
        piet: &mut PaintContext,
        offset: &Offset,
    ) {
        match self.kind {
            WidgetKind::View(w) => w.paint(render_ctx, piet, offset),
            WidgetKind::Leaf(w) => w.paint(render_ctx, piet, offset),
            WidgetKind::MultiChild(w) => w.paint(render_ctx, piet, offset),
            WidgetKind::SingleChild(w) => w.paint(render_ctx, piet, offset),
            WidgetKind::Inherited(w) => w.paint(render_ctx, piet, offset),
        }
    }

    pub fn mount(&self, build_ctx: &Context) {
        STATE_UPDATE_SUPRESSED.store(true, std::sync::atomic::Ordering::SeqCst);

        match self.kind {
            WidgetKind::View(w) => w.mount(build_ctx),
            WidgetKind::Leaf(w) => w.mount(build_ctx),
            WidgetKind::MultiChild(w) => w.mount(build_ctx),
            WidgetKind::SingleChild(w) => w.mount(build_ctx),
            WidgetKind::Inherited(w) => w.mount(build_ctx),
        }

        STATE_UPDATE_SUPRESSED.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn unmount(&self, build_ctx: &Context) {
        STATE_UPDATE_SUPRESSED.store(true, std::sync::atomic::Ordering::SeqCst);

        match self.kind {
            WidgetKind::View(w) => w.unmount(build_ctx),
            WidgetKind::Leaf(w) => w.unmount(build_ctx),
            WidgetKind::MultiChild(w) => w.unmount(build_ctx),
            WidgetKind::SingleChild(w) => w.unmount(build_ctx),
            WidgetKind::Inherited(w) => w.unmount(build_ctx),
        }

        STATE_UPDATE_SUPRESSED.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// Returned `bool` indicates whether the event was consumed.
    ///
    /// # Note
    ///
    /// This is a bad prototype.
    pub(crate) fn handle_event(&self, render_ctx: &mut AnyRenderContext, event: &Event) -> bool {
        match self.kind {
            WidgetKind::View(w) => w.handle_event(render_ctx, event),
            WidgetKind::Leaf(w) => w.handle_event(render_ctx, event),
            WidgetKind::SingleChild(w) => w.handle_event(render_ctx, event),
            WidgetKind::MultiChild(w) => w.handle_event(render_ctx, event),
            WidgetKind::Inherited(w) => w.handle_event(render_ctx, event),
        }
    }

    //
    //

    pub fn can_update(&self, other: &WidgetPtr) -> bool {
        self.unique_type_id() == other.unique_type_id()
            && self.state_type_id() == other.state_type_id()
    }

    /// Compares configurations of widgets.
    pub fn eq(&self, other: &WidgetPtr) -> bool {
        // If widget configurations are not owned, their pointer addresses
        // must be equal before we can compare them using `CheapEq`.
        if self.is_borrowed() {
            if self.widget_ptr() != other.widget_ptr() {
                return false;
            }
        }

        match self.kind {
            WidgetKind::View(w) => {
                if let WidgetKind::View(wo) = other.kind {
                    return w.eq(wo.as_any_ext());
                }
            }
            WidgetKind::Leaf(w) => {
                if let WidgetKind::Leaf(wo) = other.kind {
                    return w.eq(wo.as_any_ext());
                }
            }
            WidgetKind::SingleChild(w) => {
                if let WidgetKind::SingleChild(wo) = other.kind {
                    return w.eq(wo.as_any_ext());
                }
            }
            WidgetKind::MultiChild(w) => {
                if let WidgetKind::MultiChild(wo) = other.kind {
                    return w.eq(wo.as_any_ext());
                }
            }
            WidgetKind::Inherited(w) => {
                if let WidgetKind::Inherited(wo) = other.kind {
                    return w.eq(wo.as_any_ext());
                }
            }
        }

        false
    }

    pub fn has_key(&self) -> bool {
        self.local_key().is_some()
    }

    pub fn local_key(&self) -> Option<LocalKeyAny<'a>> {
        match self.kind {
            WidgetKind::View(w) => w.local_key(),
            WidgetKind::Leaf(w) => w.local_key(),
            WidgetKind::MultiChild(w) => w.local_key(),
            WidgetKind::SingleChild(w) => w.local_key(),
            WidgetKind::Inherited(w) => w.local_key(),
        }
    }

    fn unique_type_id(&self) -> TypeId {
        match self.kind {
            WidgetKind::View(w) => w.unique_type(),
            WidgetKind::Leaf(w) => w.unique_type(),
            WidgetKind::SingleChild(w) => w.unique_type(),
            WidgetKind::MultiChild(w) => w.unique_type(),
            WidgetKind::Inherited(w) => w.unique_type(),
        }
    }

    fn state_type_id(&self) -> TypeId {
        match self.kind {
            WidgetKind::View(w) => w.state_type_id(),
            WidgetKind::Leaf(w) => w.state_type_id(),
            WidgetKind::SingleChild(w) => w.state_type_id(),
            WidgetKind::MultiChild(w) => w.state_type_id(),
            WidgetKind::Inherited(w) => w.state_type_id(),
        }
    }

    pub fn debug_name(&self) -> &'static str {
        match self.kind {
            WidgetKind::View(w) => w.debug_name(),
            WidgetKind::Leaf(w) => w.debug_name(),
            WidgetKind::SingleChild(w) => w.debug_name(),
            WidgetKind::MultiChild(w) => w.debug_name(),
            WidgetKind::Inherited(w) => w.debug_name(),
        }
    }

    pub fn debug_name_short(&self) -> &'static str {
        match self.kind {
            WidgetKind::View(w) => w.debug_name_short(),
            WidgetKind::Leaf(w) => w.debug_name_short(),
            WidgetKind::SingleChild(w) => w.debug_name_short(),
            WidgetKind::MultiChild(w) => w.debug_name_short(),
            WidgetKind::Inherited(w) => w.debug_name_short(),
        }
    }

    pub fn inherited_key(&self) -> TypeId {
        match self.kind {
            WidgetKind::Inherited(w) => w.inherited_key(),
            _ => panic!("Widget::inherited_key() called on non-inherited widget"),
        }
    }

    fn is_borrowed(&self) -> bool {
        self.owned.is_none()
    }

    fn widget_ptr(&self) -> *const () {
        match self.kind {
            WidgetKind::View(w) => w as *const _ as *const (),
            WidgetKind::Inherited(w) => w as *const _ as *const (),
            WidgetKind::Leaf(w) => w as *const _ as *const (),
            WidgetKind::SingleChild(w) => w as *const _ as *const (),
            WidgetKind::MultiChild(w) => w as *const _ as *const (),
        }
    }
}

impl Default for WidgetPtr<'_> {
    fn default() -> Self {
        WidgetPtr::from_owned(Box::new(()))
    }
}

pub trait WidgetDebug {
    fn debug_name(&self) -> &'static str;
    fn debug_name_short(&self) -> &'static str;
}

impl<T> WidgetDebug for T {
    default fn debug_name(&self) -> &'static str {
        let full_name = std::any::type_name::<T>();
        full_name
    }

    fn debug_name_short(&self) -> &'static str {
        let full_name = std::any::type_name::<T>();

        let mut start = 0;
        let mut end = full_name.len();

        for (n, char) in full_name.chars().enumerate() {
            if char == '<' {
                end = n;
                break;
            } else if char == ':' {
                start = n + 1;
            }
        }

        &full_name[start..end]
    }
}

pub trait WidgetUniqueType {
    fn unique_type(&self) -> TypeId;
}

impl<T> WidgetUniqueType for T {
    default fn unique_type(&self) -> TypeId {
        unreachable!()
    }
}

impl<T: Widget> WidgetUniqueType for T {
    fn unique_type(&self) -> TypeId {
        T::unique_type(self)
    }
}

pub(crate) trait IntoWidgetPtr {
    fn into_widget_ptr<'a>(self) -> WidgetPtr<'a>
    where
        Self: 'a;
}

impl<T: Widget> IntoWidgetPtr for T {
    default fn into_widget_ptr<'a>(self) -> WidgetPtr<'a>
    where
        Self: 'a,
    {
        WidgetPtr::from_owned(Box::new(self))
    }
}

impl<T: Widget> IntoWidgetPtr for &T {
    default fn into_widget_ptr<'a>(self) -> WidgetPtr<'a>
    where
        Self: 'a,
    {
        WidgetPtr::from_ref(self)
    }
}

impl IntoWidgetPtr for &dyn Widget {
    default fn into_widget_ptr<'a>(self) -> WidgetPtr<'a>
    where
        Self: 'a,
    {
        WidgetPtr::from_ref(self)
    }
}

pub(crate) use any_ext::*;

mod any_ext {
    use std::{
        any::{Any, TypeId},
        marker::PhantomData,
    };

    /// This trait allows us to acquire `TypeId` of any `T` (not just `T: 'static`),
    /// which is used to downcast trait objects containing non-static fields to a
    /// concrete type.
    pub trait AnyExt: AsAny {
        fn type_id(&self) -> TypeId;

        /// Helper function.
        fn as_any_ext<'a>(&'a self) -> &'a (dyn AnyExt + 'a);
    }

    impl<T> AnyExt for T {
        fn type_id(&self) -> TypeId {
            get_type_id::<T>()
        }

        fn as_any_ext<'a>(&'a self) -> &'a dyn AnyExt {
            self
        }
    }

    impl<'a> dyn AnyExt + 'a {
        /// Downcasts reference `self` to `T` or returns `None`.
        ///
        /// # Safety
        ///
        /// Downcasted `&T` may contain references of lifetimes that are
        /// different between two structures even if `TypeId`s match.
        ///
        /// One must ensure that this cannot cause UB.
        ///
        /// # Example
        ///
        /// Using internal mutabilty one can swap `'a` and `'static` references
        /// causing dangling references and use-after-free.
        ///
        /// ```
        /// struct Test<'a> {
        ///     r: RefCell<&'a str>,
        /// }
        ///
        /// impl<'a> Test<'a> {
        ///     fn swap(&'a self, other: &'a Test<'a>) {
        ///         *self.r.borrow_mut() = *other.r.borrow();
        ///     }
        /// }
        ///
        /// let string = String::from("non 'static");
        ///
        /// let static_ = Test {
        ///     r: RefCell::new("'static str"),
        /// };
        /// let non_static = Test {
        ///     r: RefCell::new(&string),
        /// };
        ///
        /// let static_any: &dyn AnyExt = &static_;
        /// let non_static_any: &dyn AnyExt = &non_static;
        ///
        /// fn uh_oh(static_: &dyn AnyExt, non_static: &dyn AnyExt) {
        ///     unsafe {
        ///         let static_ = static_.downcast_ref::<Test>().unwrap();
        ///         let non_static = non_static.downcast_ref::<Test>().unwrap();
        ///
        ///         static_.swap(non_static);
        ///     }
        /// }
        ///
        /// uh_oh(static_any, non_static_any);
        ///
        /// drop(string);
        /// println!("{}", static_.r.borrow()); // uh-oh
        /// ```
        pub unsafe fn downcast_ref<T>(&self) -> Option<&T> {
            match AnyExt::type_id(self) == get_type_id::<T>() {
                true => Some(&*(self as *const _ as *const T)),
                false => None,
            }
        }

        /// # Safety
        ///
        /// See `downcast_ref`.
        pub unsafe fn downcast_mut<T>(&mut self) -> Option<&mut T> {
            match AnyExt::type_id(self) == get_type_id::<T>() {
                true => Some(&mut *(self as *mut _ as *mut T)),
                false => None,
            }
        }
    }

    struct TypeIdKey<T>(PhantomData<T>);

    impl<T> TypeIdKey<T> {
        fn new() -> Self {
            TypeIdKey(PhantomData)
        }
    }

    fn get_type_id<T>() -> TypeId {
        unsafe {
            let key = <TypeIdKey<T>>::new();

            // Safety: We cast &key to 'static to be able to cast it to `Any` to acquire TypeId.
            // This is because `TypeId::of::<TypeIdKey<T>>()` won't work since T isn't 'static.
            //
            // That `&'static key` reference is not used any longer than it would normally be.
            let any = std::mem::transmute::<&dyn AsAny, &'static dyn AsAny>(&key);
            let any = any.as_any();
            Any::type_id(any)
        }
    }

    /// Helper trait used in [`get_type_id`] above.
    pub trait AsAny {
        fn as_any(&'static self) -> &dyn Any;
    }

    impl<T> AsAny for T {
        fn as_any(&'static self) -> &dyn Any {
            self
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn should_downcast() {
            unsafe {
                assert!((&16usize as &dyn AnyExt).downcast_ref::<usize>().is_some());
                assert!((&String::new() as &dyn AnyExt)
                    .downcast_ref::<String>()
                    .is_some());
                assert!((&std::sync::Mutex::new(2u8) as &dyn AnyExt)
                    .downcast_ref::<std::sync::Mutex<u8>>()
                    .is_some());
            }
        }

        #[test]
        fn should_not_downcast() {
            unsafe {
                assert!((&16usize as &dyn AnyExt).downcast_ref::<u8>().is_none());
                assert!((&std::sync::Mutex::new(2u8) as &dyn AnyExt)
                    .downcast_ref::<std::sync::Mutex<usize>>()
                    .is_none());
            }
        }
    }
}
