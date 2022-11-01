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
