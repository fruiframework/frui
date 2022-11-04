#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Flex {
            children: (
                UnconstrainedBox {
                    child: SizedBox::new(
                        ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::RED,
                        },
                        Some(100.0),
                        Some(100.0),
                    ),
                },
                UnconstrainedBox {
                    child: ColoredBox {
                        child: Text::new("Hello world!"),
                        color: Color::FUCHSIA,
                    },
                },
                Expanded::new(ColoredBox {
                    child: Text::new("Hello world!"),
                    color: Color::GREEN,
                }),
            ),
            direction: Axis::Horizontal,
            main_axis_size: MainAxisSize::Max,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Stretch,
            text_direction: TextDirection::Ltr,
            vertical_direction: VerticalDirection::Down,
        }
    }
}

fn main() {
    run_app(App);
}
