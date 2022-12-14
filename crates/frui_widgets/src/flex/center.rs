use frui::prelude::*;
use frui::render::*;

#[derive(RenderWidget)]
pub struct Center<W: Widget> {
    pub child: W,
}

impl<W: Widget> Center<W> {
    pub fn child(child: W) -> Self {
        Center { child }
    }
}

impl<W: Widget> RenderWidget for Center<W> {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size {
        let child_size = cx.child(0).layout(constraints.loosen());

        let mut size = constraints.biggest();

        if constraints.max_height == f64::INFINITY {
            size.height = child_size.height;
        } else if constraints.max_width == f64::INFINITY {
            size.width = child_size.width;
        }

        size
    }

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let self_size = cx.size();
        let child_size = cx.child(0).size();

        let child_offset = Offset {
            x: offset.x + (self_size.width - child_size.width) / 2.,
            y: offset.y + (self_size.height - child_size.height) / 2.,
        };

        cx.child(0).paint(canvas, &child_offset);
    }
}
