#![allow(unused)]

use frui::prelude::*;

const SQUARE_RED: Big = Big::new(Color::rgb8(255, 0, 110));
const SQUARE_BLUE: Big = Big(50., 50., Color::rgb8(0, 186, 255));
const SQUARE_GREEN: Big = Big::new(Color::rgb8(13, 245, 152));

pub fn inflexible() -> impl WidgetList {
    (SQUARE_RED, SQUARE_BLUE, SQUARE_GREEN)
}

pub fn flexible() -> impl WidgetList {
    (
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(Color::RED)
                .child(Text::new("Tight,flex=1")),
        ),
        Flexible::new(
            Container::builder()
                .width(50.)
                .height(50.)
                .color(Color::RED)
                .child(Text::new("Loose,flex=1")),
        ),
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(Color::RED)
                .child(Text::new("Tight,flex=2")),
        )
        .flex(2),
    )
}

pub fn flexible_inflexible() -> impl WidgetList {
    (
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(Color::RED)
                .child(Text::new("Tight,flex=1")),
        ),
        SQUARE_RED,
        Flexible::new(
            Container::builder()
                .width(50.)
                .height(50.)
                .color(Color::RED)
                .child(Text::new("Loose,flex=1")),
        ),
        SQUARE_BLUE,
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(Color::RED)
                .child(Text::new("Tight,flex=2")),
        )
        .flex(2),
        SQUARE_GREEN,
    )
}

#[derive(ViewWidget)]
pub struct Big(pub f64, pub f64, pub Color);

impl Big {
    pub(crate) const fn new(color: Color) -> Self {
        Self(100., 100., color)
    }
}

impl ViewWidget for Big {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Container::builder()
            .color(self.2.clone())
            .width(self.0)
            .height(self.1)
    }
}
