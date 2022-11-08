use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};

use crate::{
    api::contexts::build_ctx::{StateGuard, StateGuardMut, STATE_UPDATE_SUPRESSED},
    app::tree::WidgetNodeRef,
    prelude::{Widget, WidgetState},
};

use super::RenderState;

/// Extension trait for all rendering contexts.
///
/// Todo: Implement for `LayoutCtx`, `PaintCtx`, ...
pub trait RenderExt<W: Widget> {
    #[doc(hidden)]
    fn node(&self) -> &WidgetNodeRef;

    fn widget_state(&self) -> StateGuard<W::State>
    where
        W: WidgetState,
    {
        StateGuard {
            guard: Ref::map(self.node().borrow(), |node| node.state.deref()),
            _p: PhantomData,
        }
    }

    fn widget_state_mut(&self) -> StateGuardMut<W::State>
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

    fn render_state(&self) -> Ref<W::State>
    where
        W: RenderState,
    {
        Ref::map(self.node().borrow(), |node| {
            node.render_data.state.deref().downcast_ref().unwrap()
        })
    }

    fn render_state_mut(&self) -> RefMut<W::State>
    where
        W: RenderState,
    {
        RefMut::map(self.node().borrow_mut(), |node| {
            node.render_data.state.deref_mut().downcast_mut().unwrap()
        })
    }
}
