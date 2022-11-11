use frui::prelude::*;

use crate::BoxLayoutData;

#[derive(RenderWidget, Default, Builder)]
pub struct ConstrainedBox<T: Widget> {
    pub child: T,
    pub constraints: Constraints,
}

impl<T: Widget> ParentData for ConstrainedBox<T> {
    type Data = BoxLayoutData;

    fn create_data(&self) -> Self::Data {
        BoxLayoutData::default()
    }
}

impl<T: Widget> RenderWidget for ConstrainedBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let constraints = self.constraints.enforce(constraints);
        let child_size = ctx.child(0).layout(constraints);

        if child_size != Size::ZERO {
            child_size
        } else {
            self.constraints.enforce(constraints).constrain(Size::ZERO)
        }
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}

#[derive(RenderWidget, Builder)]
pub struct UnconstrainedBox<T: Widget> {
    pub child: T,
}

impl<T: Widget> ParentData for UnconstrainedBox<T> {
    type Data = BoxLayoutData;

    fn create_data(&self) -> Self::Data {
        BoxLayoutData::default()
    }
}

impl<T: Widget> RenderWidget for UnconstrainedBox<T> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints.loosen());
        if child_size != Size::ZERO {
            child_size
        } else {
            constraints.biggest()
        }
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}
pub struct SizedBox;

impl SizedBox {
    pub fn from_size<T: Widget>(child: T, size: Size) -> impl Widget {
        ConstrainedBox {
            child,
            constraints: Constraints::new_tight_for(Some(size.width), Some(size.height)),
        }
    }

    pub fn new<T: Widget>(child: T, width: Option<f64>, height: Option<f64>) -> impl Widget {
        ConstrainedBox {
            child,
            constraints: Constraints::new_tight_for(width, height),
        }
    }

    pub fn square<T: Widget>(child: T, size: f64) -> impl Widget {
        Self::from_size(child, Size::new(size, size))
    }

    pub fn shrink<T: Widget>(child: T) -> impl Widget {
        ConstrainedBox {
            child,
            constraints: Constraints::new_tight_for(None, None),
        }
    }
}
