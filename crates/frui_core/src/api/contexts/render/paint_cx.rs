use std::marker::PhantomData;

use crate::{
    app::{runner::Canvas, tree::NodeRef},
    prelude::Widget,
};

use super::{ext::RenderExt, Offset, RenderOSExt};

pub struct PaintCx<T> {
    cx: PaintCxOS,
    _p: PhantomData<T>,
}

impl<T> PaintCx<T> {
    pub fn new(cx: PaintCxOS) -> Self {
        Self {
            cx,
            _p: PhantomData,
        }
    }
}

impl<W: Widget> RenderExt<W> for PaintCx<W> {
    fn node(&self) -> &NodeRef {
        &self.node
    }
}

impl<T> std::ops::Deref for PaintCx<T> {
    type Target = PaintCxOS;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<T> std::ops::DerefMut for PaintCx<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

#[derive(Clone)]
pub struct PaintCxOS {
    node: NodeRef,
    // Following are used to correctly register local transformation of the
    // offset. It is used to automatically transform point during hit testing.
    /// (global)
    offset: Offset,
    /// (global)
    parent_offset: Offset,
}

impl RenderOSExt for PaintCxOS {
    fn node(&self) -> &NodeRef {
        &self.node
    }
}

impl PaintCxOS {
    pub(crate) fn new(node: NodeRef) -> Self {
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

        self.node.widget().paint(self.clone(), piet, offset);
    }

    #[track_caller]
    pub fn child(&mut self, index: usize) -> PaintCxOS {
        let child = self
            .node
            .child(index)
            .expect("specified node didn't have child at that index");

        PaintCxOS {
            node: child,
            offset: Offset::default(),
            parent_offset: self.offset.clone(),
        }
    }

    pub fn children<'a>(&'a mut self) -> impl Iterator<Item = PaintCxOS> + 'a {
        self.node.children().into_iter().map(|child| PaintCxOS {
            node: child,
            offset: Offset::default(),
            parent_offset: self.offset.clone(),
        })
    }
}
