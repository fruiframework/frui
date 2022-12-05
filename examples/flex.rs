#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        DebugContainer::new(
            Flex::builder()
                .space_between(10.0)
                .direction(Axis::Horizontal)
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
                        flex: 1,
                        fit: FlexFit::Tight,
                        child: SizedBox::from_size(
                            ColoredBox {
                                child: Text::new("Hello world!"),
                                color: Color::RED,
                            },
                            Size::new(100.0, 100.0),
                        ),
                    },
                    SizedBox::from_size(
                        ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::RED,
                        },
                        Size::new(100.0, 100.0),
                    ),
                    Flexible {
                        flex: 2,
                        fit: FlexFit::Tight,
                        child: ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::FUCHSIA,
                        },
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
    run_app(Directionality {
        direction: TextDirection::Rtl,
        child: App,
    });
}
