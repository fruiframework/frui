use std::marker::PhantomData;

use druid_shell::kurbo::{Affine, Point};

use crate::app::tree::pointer_handler::HitTestEntries;
use crate::app::tree::NodeRef;
use crate::prelude::Widget;
use crate::render::*;

pub struct HitTestCx<W> {
    pub(crate) inner: HitTestCxOS,
    _p: PhantomData<W>,
}

impl<W: Widget> RenderExt<W> for HitTestCx<W> {
    fn node(&self) -> &NodeRef {
        &self.inner.node
    }
}

impl<W> HitTestCx<W> {
    pub(crate) fn new(cx: HitTestCxOS) -> HitTestCx<W> {
        Self {
            inner: cx,
            _p: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct HitTestCxOS {
    pub(crate) node: NodeRef,
    /// All affine transformations applied to point at this depth.
    pub(crate) affine: Affine,
    /// All widgets that got hit and registered for pointer events.
    pub(crate) hit_entries: HitTestEntries,
}

impl HitTestCxOS {
    pub(crate) fn new(node: &NodeRef, hit_entries: HitTestEntries, affine: Affine) -> HitTestCxOS {
        Self {
            node: node.clone(),
            hit_entries,
            affine,
        }
    }

    pub fn child(&self, index: usize) -> Option<HitTestCxOS> {
        Some(HitTestCxOS {
            node: self.node.child(index)?,
            hit_entries: self.hit_entries.clone(),
            affine: self.affine,
        })
    }

    pub fn children<'a>(&'a mut self) -> ChildrenIter<'a> {
        self.node.children().into_iter().map(|child| HitTestCxOS {
            node: child,
            ..self.clone()
        })
    }

    /// Add comment.
    pub fn hit_test(&mut self, point: Point) -> bool {
        self.node.widget().hit_test_os(self.clone(), point)
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
        let mut cx = self.clone();

        let point_after = transform * point;
        cx.affine = transform * self.affine;

        self.node.widget().hit_test_os(cx, point_after)
    }

    pub fn layout_box(&self) -> Size {
        self.node.borrow().render_data.size
    }
}

type ChildrenIter<'a> =
    impl Iterator<Item = HitTestCxOS> + 'a + DoubleEndedIterator + ExactSizeIterator;

impl<W> std::ops::Deref for HitTestCx<W> {
    type Target = HitTestCxOS;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<W> std::ops::DerefMut for HitTestCx<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
