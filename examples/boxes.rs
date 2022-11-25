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
                        color: Color::OLIVE,
                        child: Text::new("TOP_START"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::TOP_CENTER)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::RED,
                        child: Text::new("TOP_CENTER"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::TOP_END)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::PURPLE,
                        child: Text::new("TOP_END"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::CENTER_START)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::BLUE,
                        child: Text::new("CENTER_START"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::CENTER)
                .child(LimitedBox {
                    max_width: 100.0,
                    max_height: 100.0,
                    child: ColoredBox {
                        color: Color::YELLOW,
                        child: Padding::builder()
                            .padding(EdgeInsets::all(20.0))
                            .child(Text::new("CENTER").color(Color::BLACK)),
                    },
                }),
            Align::builder()
                .alignment(AlignmentDirectional::CENTER_END)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::GREEN,
                        child: Text::new("CENTER_END"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::BOTTOM_START)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::FUCHSIA,
                        child: Text::new("BOTTOM_START"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::BOTTOM_CENTER)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::TEAL,
                        child: Text::new("BOTTOM_CENTER"),
                    },
                    Size::new(100.0, 100.0),
                )),
            Align::builder()
                .alignment(AlignmentDirectional::BOTTOM_END)
                .child(SizedBox::from_size(
                    ColoredBox {
                        color: Color::AQUA,
                        child: Text::new("BOTTOM_END"),
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
