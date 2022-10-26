use crate::prelude::{Constraints, Offset, PaintContext, RenderContext, Size};

pub trait LeafWidget: WidgetDerive + Sized {
    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size;

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset);
}

pub(crate) use sealed::LeafWidgetOS;

use super::WidgetDerive;

mod sealed {
    use crate::{
        api::{
            contexts::{
                build_ctx::WidgetStateOS,
                render_ctx::{AnyRenderContext, ParentDataOS, RenderStateOS, _RenderContext},
            },
            events::WidgetEventOS,
            local_key::WidgetLocalKey,
            structural_eq::StructuralEqOS,
            AnyExt, WidgetDebug, WidgetUniqueType,
        },
        prelude::{Constraints, Offset, PaintContext, Size},
    };

    /// `OS` stands for "object safe".
    pub trait LeafWidgetOS:
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
        fn layout<'a>(&self, ctx: &'a mut AnyRenderContext, constraints: Constraints) -> Size;

        fn paint<'a>(
            &self,
            ctx: &'a mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        );
    }

    impl<T: super::LeafWidget> LeafWidgetOS for T {
        fn layout<'a>(&self, ctx: &'a mut AnyRenderContext, constraints: Constraints) -> Size {
            let ctx = &mut <_RenderContext<T>>::new(ctx);

            T::layout(&self, ctx, constraints)
        }

        fn paint<'a>(
            &self,
            ctx: &'a mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        ) {
            let ctx = &mut <_RenderContext<T>>::new(ctx);

            T::paint(&self, ctx, canvas, offset)
        }
    }
}
