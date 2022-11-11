use crate::{
    api::{contexts::build_ctx::_BuildCtx, IntoWidgetPtr, WidgetPtr},
    macro_exports::RawBuildCtx,
    prelude::BuildCtx,
    render::*,
};

use super::{RenderWidgetOS, WidgetDerive};

pub trait RenderWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>>;

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size;

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset);
}

impl<T: RenderWidget> RenderWidgetOS for T {
    fn build<'w>(&'w self, ctx: &'w RawBuildCtx) -> Vec<WidgetPtr<'w>> {
        let ctx = unsafe { std::mem::transmute::<&RawBuildCtx, &_BuildCtx<T>>(ctx) };

        T::build(&self, ctx)
            .into_iter()
            .map(|w| w.into_widget_ptr())
            .collect()
    }

    fn layout(&self, ctx: LayoutCtxOS, constraints: Constraints) -> Size {
        let ctx = &<LayoutCtx<T>>::new(ctx);

        T::layout(&self, ctx, constraints)
    }

    fn paint(&self, ctx: PaintCtxOS, canvas: &mut Canvas, offset: &Offset) {
        let ctx = &mut <PaintCtx<T>>::new(ctx);

        T::paint(self, ctx, canvas, offset);
    }
}
