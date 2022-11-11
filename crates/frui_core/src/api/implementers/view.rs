use crate::{
    api::{
        contexts::{
            build_ctx::{BuildCtx, _BuildCtx},
            Context,
        },
        IntoWidgetPtr, WidgetPtr,
    },
    render::*,
};

use super::{ViewWidgetOS, WidgetDerive};

pub trait ViewWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildCtx<'w, Self>) -> Self::Widget<'w>;
}

impl<T: ViewWidget> ViewWidgetOS for T {
    fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>> {
        let ctx = unsafe { std::mem::transmute::<&Context, &_BuildCtx<T>>(ctx) };

        vec![T::build(&self, ctx).into_widget_ptr()]
    }

    fn layout<'w>(&self, ctx: LayoutCtxOS, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint<'w>(&'w self, mut ctx: PaintCtxOS, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}
