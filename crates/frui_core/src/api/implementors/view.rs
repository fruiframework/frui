use crate::api::contexts::build_ctx::BuildContext;

pub trait ViewWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w>;
}

pub(crate) use sealed::ViewWidgetOS;

use super::WidgetDerive;

mod sealed {
    use crate::{
        api::{
            contexts::{
                build_ctx::{WidgetStateOS, _BuildContext},
                render_ctx::AnyRenderContext,
                Context,
            },
            events::WidgetEventOS,
            local_key::WidgetLocalKey,
            widget_eq::WidgetEqOS,
            AnyExt, IntoWidgetPtr, WidgetDebug, WidgetPtr, WidgetUniqueType,
        },
        prelude::{Constraints, Offset, PaintContext, Size},
    };

    /// `OS` stands for "object safe".
    pub trait ViewWidgetOS:
        WidgetStateOS
        + WidgetEqOS
        + WidgetLocalKey
        + WidgetUniqueType
        + WidgetDebug
        + WidgetEventOS
        + AnyExt
    {
        fn build<'w>(&'w self, ctx: &'w Context) -> WidgetPtr<'w>;

        fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size;

        fn paint<'w>(
            &'w self,
            ctx: &'w mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        );
    }

    impl<T: super::ViewWidget> ViewWidgetOS for T {
        fn build<'w>(&'w self, ctx: &'w Context) -> WidgetPtr<'w> {
            let ctx = unsafe { std::mem::transmute::<&Context, &_BuildContext<T>>(ctx) };

            T::build(&self, ctx).into_widget_ptr()
        }

        fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size {
            ctx.child().layout(constraints)
        }

        fn paint<'w>(
            &'w self,
            ctx: &'w mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        ) {
            ctx.child().paint(canvas, offset)
        }
    }
}
