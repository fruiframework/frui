#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        Stack::builder()
            .alignment(AlignmentDirectional::CENTER_END)
            .children((
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
                Positioned::builder()
                    .right(10.0)
                    .bottom(10.0)
                    .left(10.0)
                    .top(50.0)
                    .child(
                        Container::builder()
                            .color(Color::AQUA)
                            .width(50.0)
                            .height(50.0),
                    ),
                Center::child(
                    Text::new("🦀") //
                        .size(100.0)
                        .weight(FontWeight::BOLD),
                ),
            ))
    }
}

fn main() {
    run_app(App);
}
