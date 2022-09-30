use frui::prelude::*;

#[derive(SingleChildWidget)]
pub struct Center<W: Widget> {
    pub child: W,
}

impl<W: Widget> Center<W> {
    pub fn child(child: W) -> Self {
        Center { child }
    }
}

impl<W: Widget> SingleChildWidget for Center<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child().layout(constraints.loosen());

        let mut size = constraints.max();

        if constraints.max_height == f64::INFINITY {
            size.height = child_size.height;
        } else if constraints.max_width == f64::INFINITY {
            size.width = child_size.width;
        }

        size
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let self_size = ctx.size();
        let child_size = ctx.child().size();

        let child_offset = Offset {
            x: offset.x + (self_size.width - child_size.width) / 2.,
            y: offset.y + (self_size.height - child_size.height) / 2.,
        };

        ctx.child().paint(canvas, &child_offset);
    }
}
