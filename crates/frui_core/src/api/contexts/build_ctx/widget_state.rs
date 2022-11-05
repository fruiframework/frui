use std::{
    any::{Any, TypeId},
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};

use frui_macros::sealed;

use crate::{app::tree::WidgetNodeRef, macro_exports::Context, prelude::Widget};

use super::{BuildContext, StateGuard, StateGuardMut, _BuildContext, STATE_UPDATE_SUPRESSED};

pub trait WidgetState: Sized {
    type State: 'static;

    fn create_state(&self) -> Self::State;

    /// Called when the widget is mounted into the tree (before build).
    ///
    /// Accessing `state_mut` of the provided `BuildContext` will not cause a
    /// rebuild of this widget to be scheduled.
    fn mount<'a>(&'a self, ctx: BuildContext<'a, Self>) {
        let _ = ctx;
    }

    /// Called when the widget is unmounted from the tree. At this point given
    /// widget may be dropped or mounted again with its configuration updated.
    ///
    /// Accessing `state_mut` of the provided `BuildContext` will not cause a
    /// rebuild of this widget to be scheduled.
    fn unmount<'a>(&'a self, ctx: BuildContext<'a, Self>) {
        let _ = ctx;
    }
}

#[sealed(crate)]
pub trait WidgetStateOS {
    fn state_type_id(&self) -> TypeId;
    fn create_state(&self) -> Box<dyn Any>;

    fn mount(&self, build_ctx: &Context);
    fn unmount(&self, build_ctx: &Context);
}

impl<T> WidgetStateOS for T {
    default fn state_type_id(&self) -> TypeId {
        struct WidgetHasNoState;
        TypeId::of::<WidgetHasNoState>()
    }

    default fn create_state(&self) -> Box<dyn Any> {
        Box::new(())
    }

    default fn mount(&self, _: &Context) {}

    default fn unmount(&self, _: &Context) {}
}

impl<T: WidgetState> WidgetStateOS for T {
    fn state_type_id(&self) -> TypeId {
        TypeId::of::<T::State>()
    }

    fn create_state(&self) -> Box<dyn Any> {
        Box::new(T::create_state(&self))
    }

    fn mount(&self, ctx: &Context) {
        let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

        T::mount(&self, ctx)
    }

    fn unmount(&self, ctx: &Context) {
        let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

        T::unmount(&self, ctx)
    }
}

pub trait CtxStateExt<W: Widget> {
    #[doc(hidden)]
    fn node(&self) -> &WidgetNodeRef;

    fn state(&self) -> StateGuard<W::State>
    where
        W: WidgetState,
    {
        StateGuard {
            guard: Ref::map(self.node().borrow(), |node| node.state.deref()),
            _p: PhantomData,
        }
    }

    fn state_mut(&self) -> StateGuardMut<W::State>
    where
        W: WidgetState,
    {
        if !STATE_UPDATE_SUPRESSED.load(Ordering::SeqCst) {
            self.node().mark_dirty();
        }

        StateGuardMut {
            guard: RefMut::map(self.node().borrow_mut(), |node| node.state.deref_mut()),
            _p: PhantomData,
        }
    }
}

// impl<W: Widget> CtxStateExt<W> for _BuildContext<'_, W> {
//     fn node(&self) -> &WidgetNodeRef {
//         &self.node
//     }
// }
