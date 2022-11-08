use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};

use druid_shell::kurbo::{Affine, Point};

use crate::{
    api::contexts::render_ctx::ext::RenderExt,
    app::tree::{WidgetNode, WidgetNodeRef},
    prelude::{Size, Widget},
};

/// This context allows to access state of given widget during hit testing (see
/// [`CtxStateExt`] implementation below). Additionally, some methods useful for
/// hit-testing are accessible through [`HitTestCtxOS`] to which this structure
/// derefs.
pub struct HitTestCtx<W> {
    pub(crate) inner: HitTestCtxOS,
    _p: PhantomData<W>,
}

impl<W: Widget> RenderExt<W> for HitTestCtx<W> {
    fn node(&self) -> &WidgetNodeRef {
        &self.inner.node
    }
}

impl<W> HitTestCtx<W> {
    pub(crate) fn new(ctx: HitTestCtxOS) -> HitTestCtx<W> {
        Self {
            inner: ctx,
            _p: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct HitTestCtxOS {
    pub(crate) node: WidgetNodeRef,
    pub(crate) hit_entries: Rc<RefCell<HashMap<WidgetNodeRef, Affine>>>,
    /// All affine transformations applied to point at this depth.
    pub(crate) affine: Affine,
}

impl HitTestCtxOS {
    pub(crate) fn new(
        node: &WidgetNodeRef,
        hit_entries: Rc<RefCell<HashMap<WidgetNodeRef, Affine>>>,
        affine: Affine,
    ) -> HitTestCtxOS {
        Self {
            node: node.clone(),
            hit_entries,
            affine,
        }
    }

    pub fn child(&self, index: usize) -> HitTestCtxOS {
        HitTestCtxOS {
            node: WidgetNode::node_ref(&self.node.children()[index]),
            hit_entries: self.hit_entries.clone(),
            affine: self.affine,
        }
    }

    pub fn children<'a>(&'a mut self) -> ChildrenIter<'a> {
        self.node.children().iter().map(|child| {
            let mut r = self.clone();
            r.node = WidgetNode::node_ref(child);
            r
        })
    }

    /// Add comment.
    pub fn hit_test(&mut self, point: Point) -> bool {
        let widget = self.node.widget().clone();
        widget.raw().hit_test_os(self.clone(), point)
    }

    /// Add comment.
    pub fn hit_test_with_paint_offset(&mut self, point: Point) -> bool {
        let offset = self.node.borrow().render_data.local_offset;
        let affine = Affine::translate((-offset.x, -offset.y));
        self.hit_test_with_transform(point, affine)
    }

    // Todo: Add `hit_test_with_paint_transform` by defining our own
    // `canvas.save()` which will check `canvas.current_transform()` and store
    // it in the RenderState.

    /// Add comment.
    pub fn hit_test_with_transform(&mut self, point: Point, transform: Affine) -> bool {
        let mut ctx = self.clone();
        let widget = self.node.widget().clone();

        let point_after = transform * point;
        ctx.affine = transform * self.affine;

        widget.raw().hit_test_os(ctx, point_after)
    }

    pub fn layout_box(&self) -> Size {
        self.node.borrow().render_data.size
    }
}

type ChildrenIter<'a> =
    impl Iterator<Item = HitTestCtxOS> + 'a + DoubleEndedIterator + ExactSizeIterator;

impl<W> std::ops::Deref for HitTestCtx<W> {
    type Target = HitTestCtxOS;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<W> std::ops::DerefMut for HitTestCtx<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}