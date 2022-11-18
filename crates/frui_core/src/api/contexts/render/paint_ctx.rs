use std::marker::PhantomData;

use crate::{
    app::{
        runner::Canvas,
        tree::{WidgetNode, WidgetNodeRef},
    },
    prelude::Widget,
};

use super::{ext::RenderExt, Offset, RenderOSExt};

pub struct PaintCtx<T> {
    ctx: PaintCtxOS,
    _p: PhantomData<T>,
}

impl<T> PaintCtx<T> {
    pub fn new(ctx: PaintCtxOS) -> Self {
        Self {
            ctx,
            _p: PhantomData,
        }
    }
}

impl<W: Widget> RenderExt<W> for PaintCtx<W> {
    fn node(&self) -> &WidgetNodeRef {
        &self.node
    }
}

impl<T> std::ops::Deref for PaintCtx<T> {
    type Target = PaintCtxOS;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<T> std::ops::DerefMut for PaintCtx<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

#[derive(Clone)]
pub struct PaintCtxOS {
    node: WidgetNodeRef,
    // Following are used to correctly register local transformation of the
    // offset. It is used to automatically transform point during hit testing.
    /// (global)
    offset: Offset,
    /// (global)
    parent_offset: Offset,
}

impl RenderOSExt for PaintCtxOS {
    fn node(&self) -> &WidgetNodeRef {
        &self.node
    }
}

impl PaintCtxOS {
    pub(crate) fn new(node: WidgetNodeRef) -> Self {
        Self {
            node,
            offset: Offset::default(),
            parent_offset: Offset::default(),
        }
    }

    pub fn paint(&mut self, piet: &mut Canvas, offset: &Offset) {
        assert!(
            self.node.borrow().render_data.laid_out,
            "child was not laid out before paint"
        );

        // Used to calculate local offset of self (see Drop impl).
        self.offset = offset.clone();

        // Update local offset of this node.
        let local_offset = *offset - self.parent_offset;
        self.node.borrow_mut().render_data.local_offset = local_offset;

        self.node
            .widget()
            .clone()
            .raw()
            .paint(self.clone(), piet, offset);
    }

    #[track_caller]
    pub fn child(&mut self, index: usize) -> PaintCtxOS {
        let child = self
            .node
            .children()
            .get(index)
            .expect("specified node didn't have child at that index");

        PaintCtxOS {
            node: WidgetNode::node_ref(child),
            offset: Offset::default(),
            parent_offset: self.offset.clone(),
        }
    }

    pub fn children<'a>(&'a mut self) -> impl Iterator<Item = PaintCtxOS> + 'a {
        self.node.children().iter().map(|c| PaintCtxOS {
            node: WidgetNode::node_ref(c),
            offset: Offset::default(),
            parent_offset: self.offset.clone(),
        })
    }
}