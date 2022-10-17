pub trait InheritedWidget: WidgetDerive + Sized {
    fn child<'w>(&'w self) -> &'w Self::Widget<'w>;
}

pub(crate) use sealed::InheritedWidgetOS;

use super::WidgetDerive;

mod sealed {
    use std::any::TypeId;

    use crate::{
        api::{
            contexts::{
                build_ctx::WidgetStateOS,
                render_ctx::{AnyRenderContext, RenderStateOS},
            },
            events::WidgetEventOS,
            local_key::WidgetLocalKey,
            structural_eq::StructuralEqOS,
            AnyExt, IntoWidgetPtr, WidgetDebug, WidgetPtr, WidgetUniqueType,
        },
        prelude::{Constraints, Offset, PaintContext, Size},
    };

    use super::InheritedWidget;

    /// `OS` stands for "object safe".
    pub trait InheritedWidgetOS:
        WidgetStateOS
        + RenderStateOS
        + StructuralEqOS
        + WidgetLocalKey
        + WidgetUniqueType
        + WidgetDebug
        + WidgetEventOS
        + AnyExt
    {
        fn inherited_key(&self) -> TypeId;

        fn child<'w>(&'w self) -> WidgetPtr<'w>;

        fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size;

        fn paint<'w>(
            &'w self,
            ctx: &'w mut AnyRenderContext,
            canvas: &mut PaintContext,
            offset: &Offset,
        );
    }

    impl<T: InheritedWidget> InheritedWidgetOS for T {
        fn inherited_key(&self) -> TypeId {
            TypeId::of::<T::UniqueTypeId>()
        }

        fn child<'w>(&'w self) -> WidgetPtr<'w> {
            T::child(&self).into_widget_ptr()
        }

        fn layout<'w>(&'w self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size {
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
