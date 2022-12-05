use std::any::TypeId;

use crate::{
    api::{IntoWidgetPtr, WidgetPtr},
    render::*,
};

use super::{InheritedWidgetOS, WidgetDerive};

pub trait InheritedWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self) -> Self::Widget<'w>;
}

impl<T: InheritedWidget> InheritedWidgetOS for T {
    fn build<'w>(&'w self, _: &'w crate::api::contexts::RawBuildCx) -> Vec<WidgetPtr<'w>> {
        vec![T::build(self).into_widget_ptr()]
    }

    fn layout<'w>(&'w self, cx: LayoutCxOS, constraints: Constraints) -> Size {
        cx.child(0).layout(constraints)
    }

    fn paint<'w>(&'w self, mut cx: PaintCxOS, canvas: &mut Canvas, offset: &Offset) {
        cx.child(0).paint(canvas, offset)
    }

    fn inherited_key(&self) -> Option<TypeId> {
        Some(TypeId::of::<T::UniqueTypeId>())
    }
}
