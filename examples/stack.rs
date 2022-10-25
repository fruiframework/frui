#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Stack::builder() //
            .alignment(Alignment::TOP_CENTER)
            .children((
                Text::new("🦀").size(100.0).weight(FontWeight::BOLD),
                Positioned {
                    child: Container::builder()
                        .color(Color::GREEN)
                        .width(50.0)
                        .height(50.0),
                    right: Some(10.0),
                    bottom: Some(10.0),
                    left: None,
                    top: None,
                    width: None,
                    height: None,
                },
                Positioned {
                    child: Container::builder()
                        .color(Color::GREEN)
                        .width(50.0)
                        .height(50.0),
                    right: Some(10.0),
                    bottom: Some(10.0),
                    left: Some(10.0),
                    top: Some(50.0),
                    width: None,
                    height: None,
                },
                Center {
                    child: Text::new("🦀").size(100.0).weight(FontWeight::BOLD),
                },
            ))
    }
}

fn main() {
    run_app(App);
}
