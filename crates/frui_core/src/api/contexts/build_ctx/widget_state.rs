use std::any::{Any, TypeId};

use frui_macros::sealed;

use crate::macro_exports::RawBuildCtx;

use super::{BuildCtx, _BuildCtx};

pub trait WidgetState: Sized {
    type State: 'static;

    fn create_state(&self) -> Self::State;

    /// Called when the widget is mounted into the tree (before build).
    ///
    /// Accessing `state_mut` of this [`BuildCtx`] will not schedule rebuild.
    fn mount<'a>(&'a self, ctx: BuildCtx<'a, Self>) {
        let _ = ctx;
    }

    /// Called when the widget is unmounted from the tree. At this point given
    /// widget may be dropped or mounted again with its configuration updated.
    ///
    /// Accessing `state_mut` of this [`BuildCtx`] will not schedule rebuild.
    fn unmount<'a>(&'a self, ctx: BuildCtx<'a, Self>) {
        let _ = ctx;
    }
}

#[sealed(crate)]
pub trait WidgetStateOS {
    fn state_type_id(&self) -> TypeId;
    fn create_state(&self) -> Box<dyn Any>;

    fn mount(&self, build_ctx: &RawBuildCtx);
    fn unmount(&self, build_ctx: &RawBuildCtx);
}

impl<T> WidgetStateOS for T {
    default fn state_type_id(&self) -> TypeId {
        struct WidgetHasNoState;
        TypeId::of::<WidgetHasNoState>()
    }

    default fn create_state(&self) -> Box<dyn Any> {
        Box::new(())
    }

    default fn mount(&self, _: &RawBuildCtx) {}

    default fn unmount(&self, _: &RawBuildCtx) {}
}

impl<T: WidgetState> WidgetStateOS for T {
    fn state_type_id(&self) -> TypeId {
        TypeId::of::<T::State>()
    }

    fn create_state(&self) -> Box<dyn Any> {
        Box::new(T::create_state(&self))
    }

    fn mount(&self, ctx: &RawBuildCtx) {
        let ctx = unsafe { std::mem::transmute::<&RawBuildCtx, &_BuildCtx<T>>(ctx) };

        T::mount(&self, ctx)
    }

    fn unmount(&self, ctx: &RawBuildCtx) {
        let ctx = unsafe { std::mem::transmute::<&RawBuildCtx, &_BuildCtx<T>>(ctx) };

        T::unmount(&self, ctx)
    }
}
