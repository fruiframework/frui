use std::{
    cell::{Cell, Ref, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};

use druid_shell::IdleToken;

use super::build_ctx::STATE_UPDATE_SUPRESSED;
use crate::{
    app::{
        runner::{window_handler::APP_HANDLE, PaintContext},
        tree::{WidgetNode, WidgetNodeRef},
    },
    prelude::{WidgetState, InheritedState, InheritedWidget},
};

mod common;
pub mod ext;
mod parent_data;
mod render_state;

pub use common::*;
pub use parent_data::*;
pub use render_state::*;

pub type RenderContext<'a, T> = &'a _RenderContext<'a, T>;

pub struct _RenderContext<'a, T> {
    ctx: &'a AnyRenderContext,
    _p: PhantomData<T>,
}

impl<'a, T> _RenderContext<'a, T> {
    pub(crate) fn new(any: &'a AnyRenderContext) -> Self {
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
    pub fn rstate_mut(&self) -> RefMut<T::State>
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

    pub fn child(&self, index: usize) -> ChildContext {
        self.ctx.child(index)
    }

    pub fn children(&self) -> ChildrenIter {
        self.ctx.children()
    }

    pub fn size(&self) -> Size {
        self.ctx.size()
    }

    pub fn depend_on_inherited_widget<W>(&self) -> Option<InheritedState<W::State>>
    where
        W: InheritedWidget + WidgetState,
    {
        // Register and get inherited widget of specified key.
        let node = self
            .ctx
            .node
            .depend_on_inherited_widget_of_key::<W::UniqueTypeId>()?;

        // Fixme: reuse in both BuildContext and RenderContext

        Some(InheritedState {
            node,
            _p: PhantomData,
        })
    }
}

pub struct ChildContext<'a> {
    ctx: AnyRenderContext,
    _p: PhantomData<&'a ()>,
}

impl<'a> ChildContext<'a> {
    pub fn new(ctx: AnyRenderContext) -> Self {
        Self {
            ctx,
            _p: PhantomData,
        }
    }

    pub fn layout(&self, constraints: Constraints) -> Size {
        self.ctx.layout(constraints.clone())
    }

    pub fn paint(&self, canvas: &mut PaintContext, offset: &Offset) {
        self.ctx.paint(canvas, offset)
    }

    pub fn size(&self) -> Size {
        self.ctx.size()
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

    pub fn try_parent_data_mut<T: 'static>(&self) -> Option<RefMut<T>> {
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

    pub fn set_parent_data<T: 'static>(&self, data: T) {
        self.ctx.node.borrow_mut().render_data.parent_data = Box::new(data);
    }
}

pub struct AnyRenderContext {
    node: WidgetNodeRef,
    /// (global)
    offset: Cell<Offset>,
    /// (global)
    parent_offset: Cell<Offset>,
}

impl AnyRenderContext {
    pub(crate) fn new(node: WidgetNodeRef) -> Self {
        Self {
            node,
            offset: Cell::default(),
            parent_offset: Cell::default(),
        }
    }

    pub fn layout(&self, constraints: Constraints) -> Size {
        let widget = self.node.widget().clone();
        let size = widget.raw().layout(self, constraints);

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

    pub fn paint(&self, piet: &mut PaintContext, offset: &Offset) {
        assert!(
            self.node.borrow().render_data.laid_out,
            "child was not laid out before paint"
        );

        // Used to calculate local offset of self (see Drop impl).
        self.offset.set(offset.clone());

        // Update local offset of this node.
        let local_offset = *offset - self.parent_offset.get();
        self.node.borrow_mut().render_data.local_offset = local_offset;

        self.node.widget().clone().raw().paint(self, piet, offset);
    }

    pub fn child(&self, index: usize) -> ChildContext {
        self.try_child(index)
            .expect("specified node didn't have any children")
    }

    pub fn children(&self) -> ChildrenIter {
        ChildrenIter {
            child_idx: 0,
            parent_ctx: self,
        }
    }

    fn try_child(&self, index: usize) -> Option<ChildContext> {
        let child = self.node.children().get(index)?;

        let ctx = AnyRenderContext::new(WidgetNode::node_ref(child));

        // Used to calculate local offset of self (see Drop impl).
        ctx.parent_offset.set(self.offset.get());

        Some(ChildContext::new(ctx))
    }

    fn size(&self) -> Size {
        self.node.borrow().render_data.size
    }
}

pub struct ChildrenIter<'a> {
    child_idx: usize,
    parent_ctx: &'a AnyRenderContext,
}

impl<'a> ChildrenIter<'a> {
    pub fn len(&self) -> usize {
        self.parent_ctx.node.children().len()
    }
}

impl Clone for ChildrenIter<'_> {
    fn clone(&self) -> Self {
        Self {
            // Reset iterator.
            child_idx: 0,
            parent_ctx: self.parent_ctx,
        }
    }
}

impl<'a> Iterator for ChildrenIter<'a> {
    type Item = ChildContext<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.parent_ctx.try_child(self.child_idx);
        self.child_idx += 1;
        r
    }
}
