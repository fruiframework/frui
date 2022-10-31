use std::any::TypeId;

use self::{
    any_ext::AnyExt,
    contexts::{build_ctx::STATE_UPDATE_SUPRESSED, Context},
    implementors::{
        InheritedWidgetOS, LeafWidgetOS, MultiChildWidgetOS, RawWidgetOS, SingleChildWidgetOS,
        ViewWidgetOS,
    },
    structural_eq::StructuralEqOS,
};

pub(crate) mod any_ext;
pub(crate) mod contexts;
pub(crate) mod events;
pub(crate) mod implementors;
pub(crate) mod impls;
pub(crate) mod local_key;
pub(crate) mod structural_eq;

pub trait Widget: RawWidgetOS {
    /// Implementation should return the same unique TypeId for given structure definition,
    /// even if that structure contains generic parameters. This is used to preserve state
    /// between generic widgets.
    fn unique_type(&self) -> TypeId;

    fn as_os(&self) -> &dyn RawWidgetOS;
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
    kind: &'a dyn RawWidgetOS,

    /// Whether this pointer references or owns a widget. Used to properly drop it.
    pub(crate) owned: Option<*mut (dyn Widget + 'a)>,
}

impl<'a> WidgetPtr<'a> {
    pub fn from_ref(widget: &'a dyn RawWidgetOS) -> Self {
        Self {
            kind: widget,
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
                kind: std::mem::transmute::<&dyn RawWidgetOS, &dyn RawWidgetOS>(widget.as_os()),
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
        STATE_UPDATE_SUPRESSED.store(true, std::sync::atomic::Ordering::SeqCst);

        let ptrs = self.kind.build(ctx);

        STATE_UPDATE_SUPRESSED.store(false, std::sync::atomic::Ordering::SeqCst);

        // Safety: Consumers of `WidgetPtr` must upheld correct invariants.
        unsafe { std::mem::transmute::<Vec<WidgetPtr>, Vec<WidgetPtr>>(ptrs) }
    }

    pub fn can_update(&self, other: &WidgetPtr) -> bool {
        self.raw().unique_type() == other.raw().unique_type()
            && self.raw().state_type_id() == other.raw().state_type_id()
    }

    pub fn inherited_key(&self) -> TypeId {
        match self.kind.inherited_key() {
            Some(k) => k,
            None => unreachable!("inherited_key() called on non-inherited widget"),
        }
    }

    pub fn eq(&self, other: &WidgetPtr) -> bool {
        // If widget configurations are not owned, their pointer addresses
        // must be equal before we can compare them using `CheapEq`.
        if self.is_borrowed() {
            if self.widget_ptr() != other.widget_ptr() {
                return false;
            }
        }

        StructuralEqOS::eq(self.kind, other.as_any_ext())
    }

    pub fn mount(&self, build_ctx: &Context) {
        STATE_UPDATE_SUPRESSED.store(true, std::sync::atomic::Ordering::SeqCst);

        self.kind.mount(build_ctx);

        STATE_UPDATE_SUPRESSED.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn unmount(&self, build_ctx: &Context) {
        STATE_UPDATE_SUPRESSED.store(true, std::sync::atomic::Ordering::SeqCst);

        self.kind.unmount(build_ctx);

        STATE_UPDATE_SUPRESSED.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    //
    //

    pub fn raw(&self) -> &dyn RawWidgetOS {
        self.kind
    }

    //
    //

    pub fn as_any_ext(&self) -> &dyn AnyExt {
        self.kind.as_any_ext()
    }

    pub fn has_key(&self) -> bool {
        self.raw().local_key().is_some()
    }

    pub fn is_inherited_widget(&self) -> bool {
        self.kind.inherited_key().is_some()
    }

    fn is_borrowed(&self) -> bool {
        self.owned.is_none()
    }

    fn widget_ptr(&self) -> *const () {
        self.kind as *const _ as *const ()
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
        <T as Widget>::unique_type(self)
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
        WidgetPtr::from_ref(self.as_os())
    }
}

impl IntoWidgetPtr for &dyn Widget {
    default fn into_widget_ptr<'a>(self) -> WidgetPtr<'a>
    where
        Self: 'a,
    {
        WidgetPtr::from_ref(self.as_os())
    }
}
