#![feature(type_alias_impl_trait)]
use frui::{prelude::*, druid_shell::piet::{StrokeStyle, LineCap, LineJoin}};

#[derive(RenderWidget)]
pub struct BlurWidget<W: Widget> {
    child: W,
    blur_radius: f64,
}

impl<W: Widget> RenderWidget for BlurWidget<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        for child in ctx.children() {
            child.layout(constraints);
        }
        Size::new(100.0, 100.0)
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let r = canvas.with_save(|cv| {
            let brush = &cv.solid_brush(Color::RED);
            let rect = Rect::from_origin_size(offset, Size::new(100.0, 100.0));
            cv.blurred_rect(
                rect,
                self.blur_radius,
                brush,
            );
            let line = Line::new(offset, &Offset::new(100.0, 100.0));
            let line_style = StrokeStyle::new()
                .dash(vec![5.0, 5.0], 0.0)
                .line_cap(LineCap::Round)
                .line_join(LineJoin::Round);
            cv.stroke_styled(line, brush, 1.0, &line_style);
            let rect_brush = cv.solid_brush(Color::WHITE);
            cv.fill(rect, &rect_brush);
            ctx.child(0).paint(cv, offset);

            Ok(())
        });

        r.unwrap();
    }
}

fn main() {
    run_app(Center {
        child: BlurWidget {
            child: (),
            blur_radius: 10.0,
        }
    });
}
