use std::{
    cell::{Cell, Ref, RefMut},
    marker::PhantomData,
};

use crate::{
    app::tree::{WidgetNode, WidgetNodeRef},
    prelude::Widget,
};

use super::{ext::RenderExt, Offset, Size};

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
    // Todo:
    //
    // Remove the Cells!
    /// (global)
    offset: Cell<Offset>,
    /// (global)
    parent_offset: Cell<Offset>,
}

impl PaintCtxOS {
    pub(crate) fn new(node: WidgetNodeRef) -> Self {
        Self {
            node,
            offset: Cell::default(),
            parent_offset: Cell::default(),
        }
    }

    pub fn paint(&mut self, piet: &mut crate::prelude::Canvas, offset: &Offset) {
        assert!(
            self.node.borrow().render_data.laid_out,
            "child was not laid out before paint"
        );

        // Used to calculate local offset of self (see Drop impl).
        self.offset.set(offset.clone());

        // Update local offset of this node.
        let local_offset = *offset - self.parent_offset.get();
        self.node.borrow_mut().render_data.local_offset = local_offset;

        self.node
            .widget()
            .clone()
            .raw()
            .paint(self.clone(), piet, offset);
    }

    #[track_caller]
    pub fn child(&mut self, index: usize) -> PaintCtxOS {
        self.try_child(index)
            .expect("specified node didn't have any children")
    }

    pub fn children<'a>(&'a mut self) -> impl Iterator<Item = PaintCtxOS> + 'a {
        self.node.children().iter().map(|c| PaintCtxOS {
            node: WidgetNode::node_ref(c),
            offset: Cell::default(),
            parent_offset: self.offset.clone(),
        })
    }

    // Todo: Maybe inline.
    fn try_child(&self, index: usize) -> Option<PaintCtxOS> {
        let child = self.node.children().get(index)?;

        Some(PaintCtxOS {
            node: WidgetNode::node_ref(child),
            offset: Cell::default(),
            parent_offset: self.offset.clone(),
        })
    }

    //
    //

    pub fn size(&self) -> Size {
        self.node.borrow().render_data.size
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
}
