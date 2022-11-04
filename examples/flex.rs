#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Flex {
            children: (
                Flexible {
                    child: ColoredBox {
                        child: Text::new("Hello world!"),
                        color: Color::RED,
                    },
                    fit: FlexFit::Tight,
                    flex: 1,
                    
                },
                Flexible {
                    child: ColoredBox {
                        child: Text::new("Hello world!"),
                        color: Color::FUCHSIA,
                    },
                    fit: FlexFit::Tight,
                    flex: 2,
                    
                },
                // Equals with `Flexible { child: Text::new("Hello world!"), fit: FlexFit::Tight, flex: 1 }`
                Expanded::new(ColoredBox {
                    child: Text::new("Hello world!"),
                    color: Color::GREEN,
                }),
            ),
            direction: Axis::Vertical,
            main_axis_size: MainAxisSize::Max,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
            text_direction: TextDirection::Ltr,
            vertical_direction: VerticalDirection::Down,
        }
    }
}

fn main() {
    run_app(App);
}
