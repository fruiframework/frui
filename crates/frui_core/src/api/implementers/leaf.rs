use crate::{
    api::contexts::render_ctx::{AnyRenderContext, _RenderContext},
    prelude::{Constraints, Offset, PaintContext, RenderContext, Size},
};

use super::{RenderWidgetOS, WidgetDerive};

pub trait RenderWidget: WidgetDerive + Sized {
    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size;

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset);
}

impl<T: RenderWidget> RenderWidgetOS for T {
    fn build<'w>(&'w self, _: &'w crate::api::contexts::Context) -> Vec<crate::api::WidgetPtr<'w>> {
        vec![]
    }

    fn layout<'a>(&self, ctx: &'a mut AnyRenderContext, constraints: Constraints) -> Size {
        let ctx = &mut <_RenderContext<T>>::new(ctx);

        T::layout(&self, ctx, constraints)
    }

    fn paint<'a>(&self, ctx: &'a mut AnyRenderContext, canvas: &mut PaintContext, offset: &Offset) {
        let ctx = &mut <_RenderContext<T>>::new(ctx);

        T::paint(&self, ctx, canvas, offset)
    }
}
