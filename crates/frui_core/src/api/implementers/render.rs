use crate::{
    api::{contexts::build_cx::_BuildCx, IntoWidgetPtr, WidgetPtr},
    macro_exports::RawBuildCx,
    prelude::BuildCx,
    render::*,
};

use super::{RenderWidgetOS, WidgetDerive};

pub trait RenderWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>>;

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size;

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset);
}

impl<T: RenderWidget> RenderWidgetOS for T {
    fn build<'w>(&'w self, cx: &'w RawBuildCx) -> Vec<WidgetPtr<'w>> {
        let cx = unsafe { std::mem::transmute::<&RawBuildCx, &_BuildCx<T>>(cx) };

        T::build(&self, cx)
            .into_iter()
            .map(|w| w.into_widget_ptr())
            .collect()
    }

    fn layout(&self, cx: LayoutCxOS, constraints: Constraints) -> Size {
        let cx = &<LayoutCx<T>>::new(cx);

        T::layout(&self, cx, constraints)
    }

    fn paint(&self, cx: PaintCxOS, canvas: &mut Canvas, offset: &Offset) {
        let cx = &mut <PaintCx<T>>::new(cx);

        T::paint(self, cx, canvas, offset);
    }
}
