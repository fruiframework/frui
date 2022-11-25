#![feature(type_alias_impl_trait)]

use frui::{
    prelude::*,
    render::{Offset, Size},
};

fn main() {
    run_app(ColoredBox {
        color: Color::WHITE,
        child: Row::builder()
            .main_axis_size(MainAxisSize::Max)
            .cross_axis_size(CrossAxisSize::Max)
            .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .children((
                SizedBox::from_size(
                    DecoratedBox::builder()
                        .position(DecorationPosition::Background)
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
                        .child(Center::child(Text::new("Hello, world!"))),
                    Size::new(100.0, 100.0),
                ),
                SizedBox::from_size(
                    DecoratedBox::builder()
                        .position(DecorationPosition::Background)
                        .decoration(
                            BoxDecoration::builder()
                                .color(Color::Rgba32(0xFC6900FF))
                                .shape(BoxShape::Circle),
                        )
                        .child(Center::child(Text::new("+").size(60.0))),
                    Size::new(100.0, 100.0),
                ),
            )),
    });
}
