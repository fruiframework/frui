use druid_shell::kurbo::Point;
use frui_macros::sealed;

use self::context::HitTestCxOS;

pub mod context;
pub mod events;
pub mod pointer_listener;
pub mod pointer_region;

pub use context::HitTestCx;
pub use events::PointerEvent;
pub use pointer_listener::PointerListener;
pub use pointer_region::PointerRegion;

pub trait HitTest: Sized {
    fn hit_test<'a>(&'a self, cx: &'a mut HitTestCx<Self>, point: Point) -> bool {
        if cx.layout_box().contains(point) {
            for mut child in cx.children() {
                if child.hit_test_with_paint_offset(point) {
                    // Don't hit test other children if one already handled that
                    // event.
                    return true;
                }
            }

            return true;
        }

        false
    }

    #[allow(unused_variables)]
    fn handle_event(&self, cx: &mut HitTestCx<Self>, event: &PointerEvent) {}
}

#[sealed(crate)]
pub trait HitTestOS {
    fn hit_test_os(&self, cx: HitTestCxOS, point: Point) -> bool;
    fn handle_event_os(&self, cx: HitTestCxOS, event: &PointerEvent);
}

impl<T> HitTestOS for T {
    default fn hit_test_os(&self, mut cx: HitTestCxOS, point: Point) -> bool {
        if cx.layout_box().contains(point) {
            for mut child in cx.children() {
                if child.hit_test_with_paint_offset(point) {
                    // Don't hit test other children if one already handled that
                    // event.
                    return true;
                }
            }

            return true;
        }

        false
    }

    default fn handle_event_os(&self, _: HitTestCxOS, _: &PointerEvent) {}
}

impl<T: HitTest> HitTestOS for T {
    fn hit_test_os(&self, cx: HitTestCxOS, point: Point) -> bool {
        let cx = &mut <HitTestCx<T>>::new(cx);

        if T::hit_test(&self, cx, point) {
            cx.inner
                .hit_entries
                .borrow_mut()
                .insert(cx.inner.node.clone(), cx.inner.affine);

            true
        } else {
            false
        }
    }

    fn handle_event_os(&self, cx: HitTestCxOS, event: &PointerEvent) {
        let cx = &mut HitTestCx::new(cx);

        T::handle_event(&self, cx, event)
    }
}
