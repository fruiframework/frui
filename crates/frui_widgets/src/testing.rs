use frui::prelude::{RenderContext, *};

use druid_shell::piet::{
    kurbo::Rect, Color, LineCap, RenderContext as PietRenderContext, StrokeStyle,
};

#[derive(SingleChildWidget)]
pub struct DebugContainer<W: Widget> {
    pub child: W,
}

impl<W: Widget> DebugContainer<W> {
    pub fn child(child: W) -> Self {
        Self { child }
    }
}

impl<W: Widget> SingleChildWidget for DebugContainer<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        ctx.child().layout(constraints)
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child().paint(canvas, offset);

        let rect = Rect::from_origin_size(*offset, ctx.child().size());
        let brush = &canvas.solid_brush(Color::GREEN);

        canvas.stroke_styled(rect, brush, 2., &StrokeStyle::new().line_cap(LineCap::Butt));
    }
}
