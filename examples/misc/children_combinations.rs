#![allow(unused)]

use frui::prelude::*;

pub fn inflexible() -> impl WidgetList {
    (
        Big::new(Color::rgb8(13, 245, 152)),
        Big::new(Color::rgb8(255, 0, 110)),
        Big::new(Color::rgb8(0, 186, 255)),
    )
}

pub fn flexible_inflexible() -> impl WidgetList {
    (
        Expanded::new(
            Container::builder()
                .width(100.)
                .color(Color::RED)
                .child(Text::new("Tight,flex=1")),
        ),
        Big::new(Color::rgb8(13, 245, 152)),
        Flexible::new(
            Container::builder()
                .width(100.)
                .color(Color::RED)
                .child(Text::new("Loose,flex=1")),
        ),
        Big::new(Color::rgb8(255, 0, 110)),
        Expanded::new(
            Container::builder()
                .width(100.)
                .color(Color::RED)
                .child(Text::new("Tight,flex=2")),
        )
        .flex(2),
        Big::new(Color::rgb8(0, 186, 255)),
    )
}

#[derive(ViewWidget)]
pub struct Big(pub f64, pub f64, pub Color);

impl Big {
    pub(crate) fn new(color: Color) -> Self {
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
