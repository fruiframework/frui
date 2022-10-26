use crate::{
    api::contexts::build_ctx::BuildContext,
    prelude::{Constraints, Offset, PaintContext, RenderContext, Size},
};

pub trait SingleChildWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w>;

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        ctx.child().layout(constraints)
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child().paint(canvas, offset)
    }
}

pub(crate) use sealed::SingleChildWidgetOS;

use super::WidgetDerive;

mod sealed {
    use crate::{
        api::{
            contexts::{
                build_ctx::{WidgetStateOS, _BuildContext},
                render_ctx::{AnyRenderContext, ParentDataOS, RenderStateOS, _RenderContext},
                Context,
            },
            events::WidgetEventOS,
            local_key::WidgetLocalKey,
            structural_eq::StructuralEqOS,
            AnyExt, IntoWidgetPtr, WidgetDebug, WidgetPtr, WidgetUniqueType,
        },
        prelude::{Constraints, Offset, PaintContext, Size},
    };

    /// `OS` stands for "object safe".
    pub trait SingleChildWidgetOS:
        WidgetStateOS
        + ParentDataOS
        + RenderStateOS
        + StructuralEqOS
        + WidgetLocalKey
        + WidgetUniqueType
        + WidgetDebug
        + WidgetEventOS
        + AnyExt
    {
        fn build<'w>(&'w self, ctx: &'w Context) -> WidgetPtr<'w>;

        fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size;

        fn paint<'w>(
            &self,
            ctx: &'w mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        );
    }

    impl<T: super::SingleChildWidget> SingleChildWidgetOS for T {
        fn build<'w>(&'w self, ctx: &'w Context) -> WidgetPtr<'w> {
            let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

            T::build(&self, ctx).into_widget_ptr()
        }

        fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size {
            let ctx = &mut <_RenderContext<T>>::new(ctx);

            T::layout(&self, ctx, constraints)
        }

        fn paint<'w>(
            &self,
            ctx: &'w mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        ) {
            let ctx = &mut <_RenderContext<T>>::new(ctx);

            T::paint(&self, ctx, canvas, offset);
        }
    }
}
