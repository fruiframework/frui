use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
};

use druid_shell::IdleToken;

use self::ext::RenderExt;

use crate::{
    app::{
        runner::window_handler::APP_HANDLE,
        tree::{WidgetNode, WidgetNodeRef},
    },
    prelude::Widget,
};

mod common;
pub mod ext;
pub mod paint_ctx;
mod parent_data;
mod render_state;

pub use common::*;
pub use parent_data::*;
pub use render_state::*;

pub struct RenderContext<T> {
    ctx: RenderContextOS,
    _p: PhantomData<T>,
}

impl<T> RenderContext<T> {
    pub(crate) fn new(any: RenderContextOS) -> Self {
        Self {
            ctx: any,
            _p: PhantomData,
        }
    }
}

impl<W: Widget> RenderExt<W> for RenderContext<W> {
    fn node(&self) -> &WidgetNodeRef {
        &self.ctx.node
    }
}

impl<T> std::ops::Deref for RenderContext<T> {
    type Target = RenderContextOS;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<T> std::ops::DerefMut for RenderContext<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

#[derive(Clone)]
pub struct RenderContextOS {
    node: WidgetNodeRef,
}

impl RenderContextOS {
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

    pub fn child(&self, index: usize) -> RenderContextOS {
        self.try_child(index)
            .expect("specified node didn't have any children")
    }

    pub fn children(&self) -> LayoutCtxChildIter {
        LayoutCtxChildIter {
            child_idx: 0,
            parent_ctx: self,
        }
    }

    fn try_child(&self, index: usize) -> Option<RenderContextOS> {
        let child = self.node.children().get(index)?;

        let node = WidgetNode::node_ref(child);

        Some(RenderContextOS::new(node))
    }

    pub fn try_parent_data<T: 'static>(&self) -> Option<Ref<T>> {
        // Check parent data type early.
        self.node
            .borrow()
            .render_data
            .parent_data
            .downcast_ref::<T>()?;

        Some(Ref::map(self.node.borrow(), |node| {
            node.render_data.parent_data.downcast_ref().unwrap()
        }))
    }

    pub fn try_parent_data_mut<T: 'static>(&self) -> Option<RefMut<T>> {
        // Check parent data type early.
        self.node
            .borrow_mut()
            .render_data
            .parent_data
            .downcast_mut::<T>()?;

        Some(RefMut::map(self.node.borrow_mut(), |node| {
            node.render_data.parent_data.downcast_mut().unwrap()
        }))
    }

    pub fn set_parent_data<T: 'static>(&self, data: T) {
        self.node.borrow_mut().render_data.parent_data = Box::new(data);
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

    pub fn size(&self) -> Size {
        self.node.borrow().render_data.size
    }
}

pub struct LayoutCtxChildIter<'a> {
    child_idx: usize,
    parent_ctx: &'a RenderContextOS,
}

impl<'a> LayoutCtxChildIter<'a> {
    pub fn len(&self) -> usize {
        self.parent_ctx.node.children().len()
    }
}

impl Clone for LayoutCtxChildIter<'_> {
    fn clone(&self) -> Self {
        Self {
            // Reset iterator.
            child_idx: 0,
            parent_ctx: self.parent_ctx,
        }
    }
}

impl<'a> Iterator for LayoutCtxChildIter<'a> {
    type Item = RenderContextOS;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.parent_ctx.try_child(self.child_idx);
        self.child_idx += 1;
        r
    }
}
