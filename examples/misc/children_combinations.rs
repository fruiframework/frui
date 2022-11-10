#![allow(unused)]

use frui::prelude::*;

pub fn big_big_big() -> impl WidgetList {
    (
        Big(100., 100., Color::rgb8(13, 245, 152)),
        Big(100., 100., Color::rgb8(255, 0, 110)),
        Big(100., 100., Color::rgb8(0, 186, 255)),
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
