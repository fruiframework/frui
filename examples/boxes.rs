#![feature(type_alias_impl_trait)]

use frui::prelude::*;
use frui::render::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        Stack::builder().children((
            Align::builder()
                .alignment(AlignmentDirectional::TOP_START)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("TOP_START"),
                        color: Color::OLIVE,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::TOP_CENTER)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("TOP_CENTER"),
                        color: Color::RED,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::TOP_END)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("TOP_END"),
                        color: Color::PURPLE,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::CENTER_START)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("CENTER_START"),
                        color: Color::BLUE,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::CENTER)
                .child(LimitedBox {
                    child: ColoredBox {
                        child: Padding::builder()
                            .padding(EdgeInsets::all(10.0))
                            .child(Text::new("CENTER").color(Color::BLACK)),
                        color: Color::YELLOW,
                    },
                    max_width: 100.0,
                    max_height: 100.0,
                }),
            Align::builder()
                .alignment(AlignmentDirectional::CENTER_END)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("CENTER_END"),
                        color: Color::GREEN,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::BOTTOM_START)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("BOTTOM_START"),
                        color: Color::FUCHSIA,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::BOTTOM_CENTER)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("BOTTOM_CENTER"),
                        color: Color::TEAL,
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::BOTTOM_END)
                .child(SizedBox::from_size(
                    ColoredBox {
                        child: Text::new("BOTTOM_END"),
                        color: Color::AQUA,
                    },
                    Size::new(100.0, 100.0),
                )),
        ))
    }
}

fn main() {
    run_app(Directionality {
        direction: TextDirection::Rtl,
        child: App,
    });
}
