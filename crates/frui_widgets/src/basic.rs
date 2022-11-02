use frui::prelude::*;

use crate::ChildWidget;

#[derive(RenderWidget, Builder)]
pub struct ColoredBox<T: RenderWidget + Widget> {
    pub child: T,
    pub color: Color,
}

impl<T: RenderWidget + Widget> ChildWidget for ColoredBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: RenderWidget + Widget> RenderWidget for ColoredBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints);
        if child_size != Size::ZERO {
            child_size
        } else {
            constraints.smallest()
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let rect = Rect::from_origin_size(*offset, ctx.size());
        let brush = &canvas.solid_brush(self.color.clone());
        canvas.fill(rect, brush);
        ctx.child(0).paint(canvas, offset)
    }
    fn get_constraints(&self) -> Constraints {
        Constraints::default()
    }
}