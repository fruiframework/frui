//! This is an obligatory example of a counter app.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::Button;

#[derive(ViewWidget)]
struct Counter;

impl WidgetState for Counter {
    type State = isize;

    fn create_state(&self) -> Self::State {
        0
    }
}

impl ViewWidget for Counter {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        // Size of the child is basically size of the window.
        let child_width = 500.;
        let child_height = 400.;

        Transform(
            // Affine::default(),
            Affine::translate((child_width / 2., child_height / 2.))
                * Affine::rotate(std::f64::consts::FRAC_PI_4)
                * Affine::translate((-child_width / 2., -child_height / 2.)),
            Column::builder()
                .space_between(60.0)
                .main_axis_size(MainAxisSize::Max)
                .cross_axis_size(CrossAxisSize::Max)
                .main_axis_alignment(MainAxisAlignment::Center)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .children((
                    Text::new(ctx.state().to_string())
                        .size(150.0)
                        .weight(FontWeight::BOLD),
                    Row::builder()
                        .space_between(10.0) //
                        .children((
                            Button {
                                label: Text::new("+").size(30.),
                                on_click: || *ctx.state_mut() += 1,
                            },
                            Button {
                                label: Text::new("-").size(30.),
                                on_click: || *ctx.state_mut() -= 1,
                            },
                        )),
                )),
        )
    }
}

fn main() {
    run_app(Counter);
}

#[cfg(test)]
#[cfg(feature = "miri")]
mod test {
    use super::*;
    use frui::{
        app::runner::miri::MiriAppRunner,
        druid_shell::{Modifiers, MouseButtons, MouseEvent},
    };

    fn mouse_event_default(pos: Point) -> MouseEvent {
        MouseEvent {
            pos,
            buttons: MouseButtons::new(),
            mods: Modifiers::empty(),
            count: 1,
            focus: false,
            button: MouseButton::Left,
            wheel_delta: Vec2::default(),
        }
    }

    #[test]
    pub fn run_example_under_miri() {
        let mut runner = MiriAppRunner::new(Counter);

        for _ in 0..4 {
            runner.mouse_down(&mouse_event_default(Point::new(0., 0.)));
            runner.update();
        }
    }
}
