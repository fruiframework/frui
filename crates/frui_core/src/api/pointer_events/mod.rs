use druid_shell::kurbo::Point;
use frui_macros::sealed;

use self::context::HitTestCtxOS;

pub mod context;
pub mod events;
pub mod pointer_listener;
pub mod pointer_region;

pub use context::HitTestCtx;
pub use events::PointerEvent;
pub use pointer_listener::PointerListener;
pub use pointer_region::PointerRegion;

pub trait HitTest: Sized {
    fn hit_test<'a>(&'a self, ctx: &'a mut HitTestCtx<Self>, point: Point) -> bool {
        if ctx.layout_box().contains(point) {
            for mut child in ctx.children() {
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
    fn handle_event(&self, ctx: &mut HitTestCtx<Self>, event: &PointerEvent) {}
}

#[sealed(crate)]
pub trait HitTestOS {
    fn hit_test_os(&self, ctx: HitTestCtxOS, point: Point) -> bool;
    fn handle_event_os(&self, ctx: HitTestCtxOS, event: &PointerEvent);
}

impl<T> HitTestOS for T {
    default fn hit_test_os(&self, mut ctx: HitTestCtxOS, point: Point) -> bool {
        if ctx.layout_box().contains(point) {
            for mut child in ctx.children() {
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

    default fn handle_event_os(&self, _: HitTestCtxOS, _: &PointerEvent) {}
}

impl<T: HitTest> HitTestOS for T {
    fn hit_test_os(&self, ctx: HitTestCtxOS, point: Point) -> bool {
        let ctx = &mut <HitTestCtx<T>>::new(ctx);

        if T::hit_test(&self, ctx, point) {
            ctx.inner
                .hit_entries
                .borrow_mut()
                .insert(ctx.inner.node.clone(), ctx.inner.affine);

            true
        } else {
            false
        }
    }

    fn handle_event_os(&self, ctx: HitTestCtxOS, event: &PointerEvent) {
        let ctx = &mut HitTestCtx::new(ctx);

        T::handle_event(&self, ctx, event)
    }
}
