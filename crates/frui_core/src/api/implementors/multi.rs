use crate::{
    api::contexts::build_ctx::BuildContext,
    prelude::{Constraints, Offset, PaintContext, RenderContext, Size},
};

pub trait MultiChildWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>>;

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size;

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset);
}

pub(crate) use sealed::MultiChildWidgetOS;

use super::WidgetDerive;

mod sealed {
    use crate::{
        api::{
            contexts::{
                build_ctx::{WidgetStateOS, _BuildContext},
                render_ctx::{AnyRenderContext, RenderStateOS, _RenderContext},
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
    pub trait MultiChildWidgetOS:
        WidgetStateOS
        + RenderStateOS
        + StructuralEqOS
        + WidgetLocalKey
        + WidgetUniqueType
        + WidgetDebug
        + WidgetEventOS
        + AnyExt
    {
        fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>>;

        fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size;

        fn paint<'w>(
            &self,
            ctx: &'w mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        );
    }

    impl<T: super::MultiChildWidget> MultiChildWidgetOS for T {
        fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>> {
            let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

            T::build(&self, ctx)
                .into_iter()
                .map(|w| w.into_widget_ptr())
                .collect()
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
