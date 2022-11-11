use crate::{
    api::{
        contexts::{
            build_ctx::_BuildContext,
            render_ctx::{
                AnyRenderContext, _RenderContext,
                paint_ctx::{PaintContext, PaintContextOS},
            },
            Context,
        },
        IntoWidgetPtr, WidgetPtr,
    },
    prelude::{BuildContext, Canvas, Constraints, Offset, RenderContext, Size},
};

use super::{RenderWidgetOS, WidgetDerive};

pub trait RenderWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>>;

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size;

    fn paint(&self, ctx: &mut PaintContext<Self>, canvas: &mut Canvas, offset: &Offset);
}

impl<T: RenderWidget> RenderWidgetOS for T {
    fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>> {
        let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

        T::build(&self, ctx)
            .into_iter()
            .map(|w| w.into_widget_ptr())
            .collect()
    }

    fn layout(&self, ctx: &AnyRenderContext, constraints: Constraints) -> Size {
        let ctx = &<_RenderContext<T>>::new(ctx);

        T::layout(&self, ctx, constraints)
    }

    fn paint(&self, ctx: PaintContextOS, canvas: &mut Canvas, offset: &Offset) {
        let ctx = &mut <PaintContext<T>>::new(ctx);

        T::paint(self, ctx, canvas, offset);
    }
}
