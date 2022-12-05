use crate::{
    api::{
        contexts::{
            build_cx::{BuildCx, _BuildCx},
            RawBuildCx,
        },
        IntoWidgetPtr, WidgetPtr,
    },
    render::*,
};

use super::{ViewWidgetOS, WidgetDerive};

pub trait ViewWidget: WidgetDerive + Sized {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w>;
}

impl<T: ViewWidget> ViewWidgetOS for T {
    fn build<'w>(&'w self, cx: &'w RawBuildCx) -> Vec<WidgetPtr<'w>> {
        let cx = unsafe { std::mem::transmute::<&RawBuildCx, &_BuildCx<T>>(cx) };

        vec![T::build(&self, cx).into_widget_ptr()]
    }

    fn layout<'w>(&self, cx: LayoutCxOS, constraints: Constraints) -> Size {
        cx.child(0).layout(constraints)
    }

    fn paint<'w>(&'w self, mut cx: PaintCxOS, canvas: &mut Canvas, offset: &Offset) {
        cx.child(0).paint(canvas, offset)
    }
}
