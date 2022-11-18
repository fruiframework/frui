#![allow(unused)]

use frui::prelude::*;

const RED: Color = Color::rgb8(255, 0, 110);
const BLUE: Color = Color::rgb8(0, 186, 255);
const GREEN: Color = Color::rgb8(13, 245, 152);

const SQUARE_RED: Big = Big::new(RED);
const SQUARE_BLUE: Big = Big(50., 50., BLUE);
const SQUARE_GREEN: Big = Big::new(GREEN);

pub fn inflexible() -> impl WidgetList {
    (SQUARE_RED, SQUARE_BLUE, SQUARE_GREEN)
}

pub fn flexible() -> impl WidgetList {
    (
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(RED)
                .child(Text::new("Tight,flex=1")),
        ),
        Flexible::new(
            Container::builder()
                .width(50.)
                .height(50.)
                .color(BLUE)
                .child(Text::new("Loose,flex=1")),
        ),
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(GREEN)
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
                .color(RED)
                .child(Text::new("Tight,flex=1")),
        ),
        SQUARE_RED,
        Flexible::new(
            Container::builder()
                .width(50.)
                .height(50.)
                .color(BLUE)
                .child(Text::new("Loose,flex=1")),
        ),
        SQUARE_BLUE,
        Expanded::new(
            Container::builder()
                .width(100.)
                .height(100.)
                .color(GREEN)
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
