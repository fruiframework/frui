use std::any::{Any, TypeId};

use frui_macros::sealed;

use crate::macro_exports::RawBuildCx;

use super::{BuildCx, _BuildCx};

pub trait WidgetState: Sized {
    type State: 'static;

    fn create_state(&self) -> Self::State;

    /// Called when the widget is mounted into the tree (before build).
    ///
    /// Accessing `state_mut` of this [`BuildCx`] will not schedule rebuild.
    fn mount<'a>(&'a self, cx: BuildCx<'a, Self>) {
        let _ = cx;
    }

    /// Called when the widget is unmounted from the tree. At this point given
    /// widget may be dropped or mounted again with its configuration updated.
    ///
    /// Accessing `state_mut` of this [`BuildCx`] will not schedule rebuild.
    fn unmount<'a>(&'a self, cx: BuildCx<'a, Self>) {
        let _ = cx;
    }
}

#[sealed(crate)]
pub trait WidgetStateOS {
    fn state_type_id(&self) -> TypeId;
    fn create_state(&self) -> Box<dyn Any>;

    fn mount(&self, build_cx: &RawBuildCx);
    fn unmount(&self, build_cx: &RawBuildCx);
}

impl<T> WidgetStateOS for T {
    default fn state_type_id(&self) -> TypeId {
        struct WidgetHasNoState;
        TypeId::of::<WidgetHasNoState>()
    }

    default fn create_state(&self) -> Box<dyn Any> {
        Box::new(())
    }

    default fn mount(&self, _: &RawBuildCx) {}

    default fn unmount(&self, _: &RawBuildCx) {}
}

impl<T: WidgetState> WidgetStateOS for T {
    fn state_type_id(&self) -> TypeId {
        TypeId::of::<T::State>()
    }

    fn create_state(&self) -> Box<dyn Any> {
        Box::new(T::create_state(&self))
    }

    fn mount(&self, cx: &RawBuildCx) {
        let cx = unsafe { std::mem::transmute::<&RawBuildCx, &_BuildCx<T>>(cx) };

        T::mount(&self, cx)
    }

    fn unmount(&self, cx: &RawBuildCx) {
        let cx = unsafe { std::mem::transmute::<&RawBuildCx, &_BuildCx<T>>(cx) };

        T::unmount(&self, cx)
    }
}
