use std::marker::PhantomData;

use druid_shell::IdleToken;

use super::{
    ext::{RenderExt, RenderOSExt},
    Constraints, Size,
};

use crate::{
    app::{
        runner::window_handler::APP_HANDLE,
        tree::{WidgetNode, WidgetNodeRef},
    },
    prelude::{InheritedState, InheritedWidget, Widget, WidgetState},
};

pub struct LayoutCtx<T> {
    ctx: LayoutCtxOS,
    _p: PhantomData<T>,
}

impl<T> LayoutCtx<T> {
    pub(crate) fn new(any: LayoutCtxOS) -> Self {
        Self {
            ctx: any,
            _p: PhantomData,
        }
    }

    pub fn depend_on_inherited_widget<W>(&self) -> Option<InheritedState<W::State>>
    where
        W: InheritedWidget + WidgetState,
    {
        // Register and get inherited widget of specified key.
        let node = self
            .node
            .depend_on_inherited_widget_of_key::<W::UniqueTypeId>()?;

        // Todo:
        //
        // 1. Get node above.
        // 2. Increase rc/borrow count.
        // 3. Get reference to the widget's state (can be done at once in step
        //    above).
        // 4. Return InheritedGuard<'a> with that `node`, `refcell` guard, and
        //    extracted reference. Possibly transmute.

        Some(InheritedState {
            node,
            _p: PhantomData,
        })
    }
}

impl<W: Widget> RenderExt<W> for LayoutCtx<W> {
    fn node(&self) -> &WidgetNodeRef {
        &self.ctx.node
    }
}

impl<T> std::ops::Deref for LayoutCtx<T> {
    type Target = LayoutCtxOS;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<T> std::ops::DerefMut for LayoutCtx<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

#[derive(Clone)]
pub struct LayoutCtxOS {
    node: WidgetNodeRef,
}

impl RenderOSExt for LayoutCtxOS {
    fn node(&self) -> &WidgetNodeRef {
        &self.node
    }
}

impl LayoutCtxOS {
    pub(crate) fn new(node: WidgetNodeRef) -> Self {
        Self { node }
    }

    pub fn layout(&self, constraints: Constraints) -> Size {
        let widget = self.node.widget().clone();
        let size = widget.raw().layout(self.clone(), constraints);

        if cfg!(debug_assertions) {
            if size > constraints.biggest() {
                if widget.raw().debug_name_short() != "DebugContainer" {
                    log::warn!("`{}` overflowed", widget.raw().debug_name_short());
                }
            }
        }

        let render_data = &mut self.node.borrow_mut().render_data;

        render_data.size = size;
        render_data.laid_out = true;
        render_data.constraints = constraints;

        size
    }

    pub fn child(&self, index: usize) -> LayoutCtxOS {
        self.try_child(index)
            .expect("specified node didn't have any children")
    }

    pub fn children(&self) -> LayoutCtxIter {
        LayoutCtxIter {
            child_idx: 0,
            parent_ctx: self,
        }
    }

    fn try_child(&self, index: usize) -> Option<LayoutCtxOS> {
        let child = self.node.children().get(index)?;

        let node = WidgetNode::node_ref(child);

        Some(LayoutCtxOS::new(node))
    }

    pub fn schedule_layout(&mut self) {
        APP_HANDLE.with(|handle| {
            handle
                .borrow_mut()
                .as_mut()
                .expect("APP_HANDLE wasn't set")
                .schedule_idle(IdleToken::new(0));
        });
    }
}

pub struct LayoutCtxIter<'a> {
    child_idx: usize,
    parent_ctx: &'a LayoutCtxOS,
}

impl<'a> LayoutCtxIter<'a> {
    pub fn len(&self) -> usize {
        self.parent_ctx.node.children().len()
    }
}

impl Clone for LayoutCtxIter<'_> {
    fn clone(&self) -> Self {
        Self {
            // Reset iterator.
            child_idx: 0,
            parent_ctx: self.parent_ctx,
        }
    }
}

impl<'a> Iterator for LayoutCtxIter<'a> {
    type Item = LayoutCtxOS;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.parent_ctx.try_child(self.child_idx);
        self.child_idx += 1;
        r
    }
}
