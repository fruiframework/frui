#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        UnconstrainedBox {
            child: Flex {
                children: (
                    Flexible {
                        child: SizedBox::from_size(
                            ColoredBox {
                                child: Text::new("Hello world!"),
                                color: Color::RED,
                            }, Size::new(100.0, 100.0)
                        ),
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
                    SizedBox::from_size(
                        ColoredBox {
                            child: Text::new("Hello world!"),
                            color: Color::BLUE,
                        }, Size::new(100.0, 100.0))
                ),
                direction: Axis::Vertical,
                main_axis_size: MainAxisSize::Max,
                main_axis_alignment: MainAxisAlignment::SpaceAround,
                cross_axis_alignment: CrossAxisAlignment::Center,
                text_direction: TextDirection::Ltr,
                vertical_direction: VerticalDirection::Down,
                space_between: 20f64,
                cross_axis_size: CrossAxisSize::Min,
            }
        }
    }
}

fn main() {
    run_app(App);
}
