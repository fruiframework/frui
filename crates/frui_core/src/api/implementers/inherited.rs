use std::any::TypeId;

use crate::{
    api::{
        contexts::render_ctx::{paint_ctx::PaintContextOS, LayoutCtxOS},
        IntoWidgetPtr, WidgetPtr,
    },
    prelude::{Canvas, Constraints, Offset, Size},
};

use super::{InheritedWidgetOS, WidgetDerive};

pub trait InheritedWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self) -> Self::Widget<'w>;
}

impl<T: InheritedWidget> InheritedWidgetOS for T {
    fn build<'w>(&'w self, _: &'w crate::api::contexts::Context) -> Vec<WidgetPtr<'w>> {
        vec![T::build(self).into_widget_ptr()]
    }

    fn layout<'w>(&'w self, ctx: LayoutCtxOS, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint<'w>(&'w self, mut ctx: PaintContextOS, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }

    fn inherited_key(&self) -> Option<TypeId> {
        Some(TypeId::of::<T::UniqueTypeId>())
    }
}
