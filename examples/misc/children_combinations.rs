#![allow(unused)]

use frui::prelude::*;

pub fn big_big_big() -> impl WidgetList {
    (
        Big(Color::rgb8(13, 245, 152)),
        Big(Color::rgb8(255, 0, 110)),
        Big(Color::rgb8(0, 186, 255)),
    )
}

#[derive(ViewWidget)]
pub struct Big(pub Color);

impl ViewWidget for Big {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        Container::builder()
            .color(self.0.clone())
            .width(100.0)
            .height(100.0)
    }
}
