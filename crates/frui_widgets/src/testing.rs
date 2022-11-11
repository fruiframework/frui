use frui::prelude::{LayoutCtx, *};

use druid_shell::piet::{
    kurbo::Rect, Color, LineCap, RenderContext as PietRenderContext, StrokeStyle,
};

#[derive(RenderWidget)]
pub struct DebugContainer<W: Widget> {
    pub child: W,
}

impl<W: Widget> DebugContainer<W> {
    pub fn child(child: W) -> Self {
        Self { child }
    }
}

impl<W: Widget> RenderWidget for DebugContainer<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset);

        let rect = Rect::from_origin_size(*offset, ctx.child(0).size());
        let brush = &canvas.solid_brush(Color::GREEN);

        canvas.stroke_styled(rect, brush, 2., &StrokeStyle::new().line_cap(LineCap::Butt));
    }
}
