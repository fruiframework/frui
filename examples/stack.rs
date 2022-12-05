#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Stack::builder()
            .alignment(AlignmentDirectional::CENTER_END)
            .children((
                Positioned::builder()
                    .top(100.0)
                    .left(100.0)
                    .bottom(100.0)
                    .right(100.0)
                    .child(Container::builder().color(Color::AQUA)),
                Text::new("🦀") //
                    .size(100.0)
                    .weight(FontWeight::BOLD),
                Positioned::builder() //
                    .right(10.0)
                    .bottom(10.0)
                    .child(
                        Container::builder()
                            .color(Color::GREEN)
                            .width(50.0)
                            .height(50.0),
                    ),
                Center::child(
                    Text::new("Centered") //
                        .size(50.0)
                        .weight(FontWeight::BOLD),
                ),
            ))
    }
}

fn main() {
    run_app(App);
}
