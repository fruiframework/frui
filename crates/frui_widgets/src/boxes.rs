use frui::prelude::*;
use frui::render::*;

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
    fn build<'w>(&'w self, _ctx: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
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
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
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


#[derive(RenderWidget, Builder)]
pub struct ColoredBox<T: Widget> {
    pub child: T,
    pub color: Color,
}

impl<T: Widget> RenderWidget for ColoredBox<T> {
    fn build<'w>(&'w self, _ctx: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints);
        if child_size != Size::ZERO {
            child_size
        } else {
            constraints.smallest()
        }
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let rect = Rect::from_origin_size(*offset, ctx.size());
        let brush = &canvas.solid_brush(self.color.clone());
        canvas.fill(druid_shell::piet::kurbo::Rect::from(rect), brush);
        ctx.child(0).paint(canvas, offset)
    }
}

#[derive(RenderWidget, Builder)]
pub struct LimitedBox<T: Widget> {
    pub child: T,
    pub max_width: f64,
    pub max_height: f64,
}

impl LimitedBox<()> {
    pub fn builder() -> Self {
        Self {
            child: (),
            max_width: f64::INFINITY,
            max_height: f64::INFINITY,
        }
    }
}

impl<T: Widget> LimitedBox<T> {
    fn limit_constraints(&self, constraints: &Constraints) -> Constraints {
        Constraints {
            min_width: constraints.min_width,
            max_width: if constraints.has_bounded_width() {
                constraints.max_width
            } else {
                constraints.constrain_width(self.max_width)
            },
            min_height: constraints.min_height,
            max_height: if constraints.has_bounded_height() {
                constraints.max_height
            } else {
                constraints.constrain_height(self.max_height)
            },
        }
    }
}

impl<T: Widget> RenderWidget for LimitedBox<T> {
    fn build<'w>(&'w self, _ctx: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let limited_constraints = self.limit_constraints(&constraints);
        constraints.constrain(ctx.child(0).layout(limited_constraints))
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}