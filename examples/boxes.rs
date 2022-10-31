#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        UnconstrainedBox {
            child: ColoredBox {
                child: Text::new("Hello world!"),
                color: Color::RED,
            },
        }
    }
}

fn main() {
    run_app(App);
}
