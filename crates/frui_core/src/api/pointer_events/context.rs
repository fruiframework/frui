use std::{
    cell::{RefCell, UnsafeCell},
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
};

use druid_shell::kurbo::{Affine, Point};

use crate::{
    api::contexts::build_ctx::widget_state::CtxStateExt,
    app::tree::{WidgetNode, WidgetNodeRef},
    prelude::{Size, Widget},
};

/// This context allows to access state of given widget during hit testing (see
/// [`CtxStateExt`] implementation below). Useful hit-testing methods are routed
/// through [`HitTestCtxOS`] to which this structure derefs.
pub struct HitTestCtx<W> {
    pub(crate) inner: HitTestCtxOS,
    _p: PhantomData<W>,
}

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

impl<W: Widget> CtxStateExt<W> for HitTestCtx<W> {
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

    pub fn children(&mut self) -> HitTestCtxIter {
        HitTestCtxIter {
            children: self.node.children(),
            parent: self.clone(),
            idx: 0,
        }
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

    // Todo:
    // Add `hit_test_with_paint_transform` by wrapping `canvas.save()` with
    // `canvas.current_transform()` and save it to the RenderState.

    /// Add comment.
    pub fn hit_test_with_transform(&mut self, point: Point, transform: Affine) -> bool {
        let point_after = transform * point;

        let widget = self.node.widget().clone();

        // if point_after != point {
        //     println!("hit_test_with_offset:");
        //     println!("point before = {:?}", point);
        //     println!("point after  = {:?}", point_after);
        // }

        let mut ctx = self.clone();
        ctx.affine = transform * self.affine; // is affine getting aplied?

        // println!("affine before   = {:?}", self.affine);
        // println!("affine combined = {:?}", ctx.affine);

        widget.raw().hit_test_os(ctx, point_after)
    }

    pub fn layout_box(&self) -> Size {
        self.node.borrow().render_data.size
    }
}

pub struct HitTestCtxIter<'a> {
    children: &'a [UnsafeCell<Box<WidgetNode>>],
    parent: HitTestCtxOS,
    idx: usize,
}

impl<'a> HitTestCtxIter<'a> {
    pub fn len(&self) -> usize {
        self.children.len()
    }
}

impl<'a> Iterator for HitTestCtxIter<'a> {
    type Item = HitTestCtxOS;

    fn next(&mut self) -> Option<Self::Item> {
        let mut r = self.parent.clone();
        r.node = WidgetNode::node_ref(self.children.get(self.idx)?);
        self.idx += 1;

        Some(r)
    }
}
