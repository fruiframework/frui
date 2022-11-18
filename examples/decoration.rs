#![feature(type_alias_impl_trait)]
use frui::prelude::*;

fn main() {
    run_app(ColoredBox {
        child: Row::builder()
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .cross_axis_size(CrossAxisSize::Max)
            .space_between(20.0)
            .main_axis_alignment(MainAxisAlignment::SpaceAround)
            .main_axis_size(MainAxisSize::Max)
            .children(vec!(
                SizedBox::from_size(
                    DecoratedBox::builder()
                        .child(Center::child(Text::new("Hello, world!")))
                        .decoration(
                            BoxDecoration::builder()
                                .color(Color::Rgba32(0x28C6A8FF))
                                .border_radius(BorderRadius::circular(10.0))
                                .border(BoxBorder::all(
                                    Color::Rgba32(0x000000FF),
                                    2.0,
                                    BorderStyle::Dash(vec![10.0, 5.0], 0.0),
                                ))
                                .box_shadow(vec![BoxShadow {
                                    color: Color::BLACK.with_alpha(0.3),
                                    offset: Offset::new(5.0, 2.0),
                                    blur_radius: 10.0,
                                    spread_radius: 0.0,
                                    blur_style: BlurStyle::Normal,
                                }]),
                        )
                        .position(DecorationPosition::Background),
                    Size::new(100.0, 100.0),
                ),
                SizedBox::from_size(
                    DecoratedBox::builder()
                        .child(Center::child(Text::new("+").size(60.0)))
                        .decoration(
                            BoxDecoration::builder()
                                .color(Color::Rgba32(0xFC6900FF))
                                .shape(BoxShape::Circle)
                        )
                        .position(DecorationPosition::Background),
                    Size::new(100.0, 100.0)
                ),
            )),
        color: Color::WHITE,
    });
}
