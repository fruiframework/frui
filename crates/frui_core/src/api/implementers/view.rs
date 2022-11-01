use crate::{
    api::{
        contexts::{
            build_ctx::{BuildContext, _BuildContext},
            render_ctx::AnyRenderContext,
            Context,
        },
        IntoWidgetPtr, WidgetPtr,
    },
    prelude::{Constraints, Offset, PaintContext, Size},
};

use super::{ViewWidgetOS, WidgetDerive};

pub trait ViewWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w>;
}

impl<T: ViewWidget> ViewWidgetOS for T {
    fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>> {
        let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

        vec![T::build(&self, ctx).into_widget_ptr()]
    }

    fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint<'w>(
        &'w self,
        ctx: &'w mut AnyRenderContext,
        canvas: &mut PaintContext,
        offset: &Offset,
    ) {
        ctx.child(0).paint(canvas, offset)
    }
}
