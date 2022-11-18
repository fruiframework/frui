use frui::prelude::*;
use frui::render::*;

use druid_shell::piet::{kurbo::Rect, Color, LineCap, RenderContext, StrokeStyle};

pub trait DebugContainerExt: Widget + Sized {
    fn debug_container(self) -> DebugContainer<Self> {
        DebugContainer {
            child: self,
            print_size: "",
        }
    }
}

impl<T: Widget> DebugContainerExt for T {}

#[derive(RenderWidget, Builder)]
pub struct DebugContainer<W: Widget> {
    pub child: W,
    /// Print to the console size of child widget computed during layout. It
    /// will not print to the console if str == "".
    pub print_size: &'static str,
}

impl<W: Widget> DebugContainer<W> {
    pub fn new(child: W) -> Self {
        Self {
            child,
            print_size: "",
        }
    }
}

impl<W: Widget> RenderWidget for DebugContainer<W> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let size = ctx.child(0).layout(constraints);

        if self.print_size != "" {
            println!("{} = {:?}", self.print_size, size);
        }

        size
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        ctx.child(0).paint(canvas, offset);

        let rect = Rect::from_origin_size(*offset, ctx.child(0).size());
        let brush = &canvas.solid_brush(Color::GREEN);

        canvas.stroke_styled(rect, brush, 2., &StrokeStyle::new().line_cap(LineCap::Butt));
    }
}
