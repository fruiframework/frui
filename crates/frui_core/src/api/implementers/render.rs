use crate::{
    api::{
        contexts::{
            build_ctx::_BuildContext,
            render_ctx::{AnyRenderContext, _RenderContext},
            Context,
        },
        IntoWidgetPtr, WidgetPtr,
    },
    prelude::{BuildContext, Constraints, Offset, PaintContext, RenderContext, Size},
};

use super::{RenderWidgetOS, WidgetDerive};

pub trait RenderWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>>;

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size;

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset);

    fn get_constraints(&self) -> Constraints {
        Constraints::default()
    }

    fn get_min_intrinsic_width(&self, height: f64) -> f64 {
        // wrap compute result with cache, maybe `memorize`?
        self.compute_min_intrinsic_width(height)
    }

    fn get_min_intrinsic_height(&self, width: f64) -> f64 {
        self.compute_min_intrinsic_height(width)
    }

    fn get_max_intrinsic_width(&self, height: f64) -> f64 {
        self.compute_max_intrinsic_width(height)
    }

    fn get_max_intrinsic_height(&self, width: f64) -> f64 {
        self.compute_max_intrinsic_height(width)
    }

    fn compute_max_intrinsic_width(&self, _height: f64) -> f64 {
        0.0
    }
    fn compute_max_intrinsic_height(&self, _width: f64) -> f64 {
        0.0
    }
    fn compute_min_intrinsic_width(&self, _height: f64) -> f64 {
        0.0
    }
    fn compute_min_intrinsic_height(&self, _width: f64) -> f64 {
        0.0
    }

    fn compute_size_for_no_child(&self, constraints: Constraints) -> Size {
        constraints.smallest()
    }

    fn perform_layout(&self, constraints: Constraints) -> Size {
        self.compute_size_for_no_child(constraints)
    }
}

impl<T: RenderWidget> RenderWidgetOS for T {
    fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>> {
        let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

        T::build(&self, ctx)
            .into_iter()
            .map(|w| w.into_widget_ptr())
            .collect()
    }

    fn layout(&self, ctx: &mut AnyRenderContext, constraints: Constraints) -> Size {
        let ctx = &mut <_RenderContext<T>>::new(ctx);

        T::layout(&self, ctx, constraints)
    }

    fn paint(&self, ctx: &mut AnyRenderContext, canvas: &mut PaintContext, offset: &Offset) {
        let ctx = &mut <_RenderContext<T>>::new(ctx);

        T::paint(&self, ctx, canvas, offset);
    }
}
