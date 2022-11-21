#![feature(type_alias_impl_trait)]

use frui::prelude::*;
use frui::render::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        DebugContainer::new(
            Flex::builder()
                .space_between(10.0)
                .direction(Axis::Vertical)
                .text_direction(TextDirection::Rtl)
                .vertical_direction(VerticalDirection::Down)
                .main_axis_size(MainAxisSize::Max)
                .cross_axis_size(CrossAxisSize::Min)
                .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .children((
                    SizedBox::from_size(
                        ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::RED,
                        },
                        Size::new(100.0, 100.0),
                    ),
                    Flexible {
                        child: SizedBox::from_size(
                            ColoredBox {
                                child: Text::new("Hello world!"),
                                color: Color::RED,
                            },
                            Size::new(100.0, 100.0),
                        ),
                        fit: FlexFit::Tight,
                        flex: 1,
                    },
                    SizedBox::from_size(
                        ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::RED,
                        },
                        Size::new(100.0, 100.0),
                    ),
                    Flexible {
                        child: ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::FUCHSIA,
                        },
                        fit: FlexFit::Tight,
                        flex: 2,
                    },
                    Expanded::new(ColoredBox {
                        child: Text::new("Hello world!"),
                        color: Color::GREEN,
                    }),
                    SizedBox::from_size(
                        ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::BLUE,
                        },
                        Size::new(100.0, 100.0),
                    ),
                )),
        )
    }
}

fn main() {
    run_app(App);
}
