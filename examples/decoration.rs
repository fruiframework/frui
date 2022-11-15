#![feature(type_alias_impl_trait)]
use frui::{
    druid_shell::piet::{LineCap, LineJoin, StrokeStyle},
    prelude::*,
};

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
            let brush = &cv.solid_brush(Color::BLACK.with_alpha(0.3));
            let rect = Rect::from_origin_size(*offset, Size::new(100.0, 100.0));
            cv.save()?;
            let translate: Vec2 = (5f64, 5f64).into();
            cv.transform(Affine::translate(translate));
            cv.blurred_rect(rect.into(), self.blur_radius, brush);
            cv.restore()?;
            let line = Line::new(offset, &Offset::new(100.0, 100.0));
            let line_style = StrokeStyle::new()
                .dash(vec![5.0, 5.0], 0.0)
                .line_cap(LineCap::Round)
                .line_join(LineJoin::Round);
            let rounded_rect = RoundedRect::from_rect(rect.into(), 10.0);
            let line_brush = &cv.solid_brush(Color::Rgba32(0x000000FF));
            // cv.stroke_styled(rounded_rect, line_brush, 2.0, &line_style);
            cv.stroke(rounded_rect, line_brush, 4.0);
            let rect_brush = cv.solid_brush(Color::Rgba32(0x28C6A8FF));
            cv.fill(rounded_rect, &rect_brush);
            ctx.child(0).paint(cv, offset);

            Ok(())
        });

        r.unwrap();
    }
}

fn main() {
    run_app(ColoredBox {
        child: Center {
            child: DecoratedBox {
                child: SizedBox::from_size(
                    ColoredBox {
                        child: (),
                        color: Color::Rgba32(0x28C6A8FF),
                    },
                    Size::new(100.0, 100.0),
                ),
                decoration: BoxDecoration {
                    color: None,
                    box_shadow: vec![
                        BoxShadow {
                            color: Color::BLACK.with_alpha(0.3),
                            offset: Offset::new(5.0, 5.0),
                            blur_radius: 10.0,
                            spread_radius: 0.0,
                            blur_style: BlurStyle::Normal,
                        },
                    ],
                    border: Some(BoxBorder::all(
                        Color::Rgba32(0x000000FF),
                        4.0,
                        BorderStyle::Dash(vec![10.0, 5.0], 0.0),
                    )),
                    border_radius: Some(BorderRadius::circular(10.0)),
                    shape: BoxShape::Rectangle,
                    text_direction: TextDirection::Ltr,
                },
                position: DecorationPosition::Background,
            },
        },
        color: Color::WHITE,
    });
}
