use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};

use druid_shell::{kurbo::Point, IdleToken};

use super::build_ctx::STATE_UPDATE_SUPRESSED;
use crate::{
    api::events::Event,
    app::{
        runner::{handler::APP_HANDLE, PaintContext},
        tree::WidgetNodeRef,
    },
    prelude::{MultiChildWidget, SingleChildWidget, WidgetState},
};

mod parent_data;
mod render_state;
mod types;

pub use parent_data::*;
pub use render_state::*;
pub use types::*;

pub type RenderContext<'a, T> = &'a mut _RenderContext<'a, T>;

pub struct _RenderContext<'a, T> {
    ctx: &'a mut AnyRenderContext,
    _p: PhantomData<T>,
}

impl<'a, T> _RenderContext<'a, T> {
    pub(crate) fn new(any: &'a mut AnyRenderContext) -> Self {
        Self {
            ctx: any,
            _p: PhantomData,
        }
    }

    /// Render state.
    pub fn rstate(&self) -> Ref<T::State>
    where
        T: RenderState,
    {
        Ref::map(self.ctx.node.borrow(), |node| {
            node.render_data.state.deref().downcast_ref().unwrap()
        })
    }

    /// Render state mutably.
    pub fn rstate_mut(&mut self) -> RefMut<T::State>
    where
        T: RenderState,
    {
        RefMut::map(self.ctx.node.borrow_mut(), |node| {
            node.render_data.state.deref_mut().downcast_mut().unwrap()
        })
    }

    /// Widget state.
    pub fn wstate(&self) -> Ref<T::State>
    where
        T: WidgetState,
    {
        Ref::map(self.ctx.node.borrow(), |node| {
            node.state.deref().downcast_ref().unwrap()
        })
    }

    /// Widget state mutably.
    pub fn wstate_mut(&self) -> RefMut<T::State>
    where
        T: WidgetState,
    {
        if !STATE_UPDATE_SUPRESSED.load(Ordering::SeqCst) {
            self.ctx.node.mark_dirty();
        }

        RefMut::map(self.ctx.node.borrow_mut(), |node| {
            node.state.deref_mut().downcast_mut().unwrap()
        })
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

    //

    pub fn child(&mut self) -> ChildContext
    where
        T: SingleChildWidget,
    {
        self.ctx.child()
    }

    pub fn children(&mut self) -> ChildContextIter
    where
        T: MultiChildWidget,
    {
        self.ctx.children()
    }

    //

    #[track_caller]
    pub fn size(&self) -> Size {
        self.ctx.node.borrow().render_data.size
    }

    #[track_caller]
    pub fn offset(&self) -> Offset {
        self.ctx.node.borrow().render_data.offset
    }

    pub fn point_in_layout_bounds(&self, point: Point) -> bool {
        let Offset { x: o_x, y: o_y } = self.offset();
        let Point { x, y } = point;

        // Make point position local to the tested widget origin.
        let (x, y) = (x - o_x, y - o_y);

        let Size { width, height } = self.size();

        // Check if that point is in the widget bounds computed during layout.
        x >= 0.0 && x <= width && y >= 0.0 && y <= height
    }
}

pub struct ChildContext<'a> {
    ctx: AnyRenderContext,
    _p: PhantomData<&'a ()>,
}

impl<'a> ChildContext<'a> {
    pub fn size(&self) -> Size {
        self.ctx.node.borrow().render_data.size
    }

    pub fn try_parent_data<T: 'static>(&self) -> Option<Ref<T>> {
        // Check parent data type early.
        self.ctx
            .node
            .borrow()
            .render_data
            .parent_data
            .downcast_ref::<T>()?;

        Some(Ref::map(self.ctx.node.borrow(), |node| {
            node.render_data.parent_data.downcast_ref().unwrap()
        }))
    }

    pub fn try_parent_data_mut<T: 'static>(&mut self) -> Option<RefMut<T>> {
        // Check parent data type early.
        self.ctx
            .node
            .borrow_mut()
            .render_data
            .parent_data
            .downcast_mut::<T>()?;

        Some(RefMut::map(self.ctx.node.borrow_mut(), |node| {
            node.render_data.parent_data.downcast_mut().unwrap()
        }))
    }

    pub fn layout(&mut self, constraints: Constraints) -> Size {
        self.ctx.layout(constraints.clone())
    }

    #[track_caller]
    pub fn paint(&mut self, canvas: &mut PaintContext, offset: &Offset) {
        self.ctx.paint(canvas, offset)
    }

    #[track_caller]
    pub fn handle_event(&mut self, event: &Event) {
        self.ctx.handle_event(event)
    }
}

pub struct AnyRenderContext {
    node: WidgetNodeRef,
}

impl AnyRenderContext {
    pub(crate) fn new(node: WidgetNodeRef) -> Self {
        Self { node }
    }

    pub(crate) fn child(&mut self) -> ChildContext {
        let child_node = self
            .node
            .children()
            .get(0)
            .expect("specified node didn't have any children");

        ChildContext {
            ctx: AnyRenderContext::new(crate::app::tree::WidgetNode::node_ref(child_node)),
            _p: PhantomData,
        }
    }

    pub(crate) fn children(&mut self) -> ChildContextIter {
        ChildContextIter {
            child_idx: 0,
            parent: &self.node,
        }
    }

    pub(crate) fn layout(&mut self, constraints: Constraints) -> Size {
        let widget = self.node.widget().clone();
        let size = widget.raw().layout(self, constraints);

        if cfg!(debug_assertions) {
            if size > constraints.max() {
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

    pub(crate) fn paint(&mut self, piet: &mut PaintContext, offset: &Offset) {
        assert!(
            self.node.borrow().render_data.laid_out,
            "child was not laid out before paint"
        );

        // This should probably be calculated during layout probably.
        self.node.borrow_mut().render_data.offset = offset.clone();

        self.node.widget().clone().raw().paint(self, piet, offset);
    }

    pub(crate) fn handle_event(&mut self, event: &Event) {
        self.node.widget().clone().raw().handle_event(self, event);
    }
}

pub struct ChildContextIter<'a> {
    child_idx: usize,
    parent: &'a WidgetNodeRef,
}

impl<'a> ChildContextIter<'a> {
    pub fn len(&self) -> usize {
        self.parent.children().len()
    }
}

impl<'a> Iterator for ChildContextIter<'a> {
    type Item = ChildContext<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_child = match self.parent.children().get(self.child_idx) {
            Some(c) => c,
            None => return None,
        };

        self.child_idx += 1;

        Some(ChildContext {
            ctx: AnyRenderContext::new(crate::app::tree::WidgetNode::node_ref(next_child)),
            _p: PhantomData,
        })
    }
}
