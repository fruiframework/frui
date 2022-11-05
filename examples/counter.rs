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
                * Affine::rotate(std::f64::consts::FRAC_PI_8)
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
                    PointerListener::builder()
                        .on_pointer_down(|e| println!("{}", e.0.pos))
                        .on_pointer_up(|e| println!("{}", e.0.pos))
                        .child(
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
                        ),
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
        app::runner::miri::MiriRunner,
        druid_shell::{Modifiers, MouseButtons, MouseEvent},
    };

    static COUNT: std::sync::Mutex<isize> = std::sync::Mutex::new(0);

    #[derive(ViewWidget)]
    struct OnlyButtons;

    impl WidgetState for OnlyButtons {
        type State = isize;

        fn create_state(&self) -> Self::State {
            0
        }
    }

    impl ViewWidget for OnlyButtons {
        fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
            *COUNT.lock().unwrap() = *ctx.state();

            Row::builder().space_between(10.0).children((
                Button {
                    label: (),
                    on_click: || *ctx.state_mut() += 1,
                },
                Button {
                    label: (),
                    on_click: || *ctx.state_mut() -= 1,
                },
            ))
        }
    }

    #[test]
    pub fn run_example_under_miri() {
        let mut runner = MiriRunner::new(OnlyButtons);

        runner.size(frui::druid_shell::kurbo::Size {
            width: 500.,
            height: 400.,
        });

        click_plus(&mut runner);
        fake_click_plus(&mut runner);

        assert_eq!(*COUNT.lock().unwrap(), 1);

        click_minus(&mut runner);
        fake_click_minus(&mut runner);

        assert_eq!(*COUNT.lock().unwrap(), 0);
    }

    fn click_plus(runner: &mut MiriRunner) {
        let (x, y) = (20.0, 20.0);

        runner.mouse_move(&mdef(Point::new(x, y)));
        runner.mouse_down(&mdef(Point::new(x, y)));
        runner.mouse_up(&mdef(Point::new(x, y)));
        runner.update(false);
    }

    fn click_minus(runner: &mut MiriRunner) {
        let (x, y) = (80.0, 40.0);

        runner.mouse_move(&mdef(Point::new(x, y)));
        runner.mouse_down(&mdef(Point::new(x, y)));
        runner.mouse_up(&mdef(Point::new(x, y)));
        runner.update(false);
    }

    fn fake_click_plus(runner: &mut MiriRunner) {
        let (x, y) = (20.0, 20.0);
        let (x2, y2) = (20.0, 100.0);

        runner.mouse_move(&mdef(Point::new(x, y)));
        runner.mouse_down(&mdef(Point::new(x, y)));
        runner.mouse_move(&mdef(Point::new(x2, y2)));
        runner.mouse_up(&mdef(Point::new(x2, y2)));
        runner.update(false);
    }

    fn fake_click_minus(runner: &mut MiriRunner) {
        let (x, y) = (80.0, 40.0);
        let (x2, y2) = (80.0, 100.0);

        runner.mouse_move(&mdef(Point::new(x, y)));
        runner.mouse_down(&mdef(Point::new(x, y)));
        runner.mouse_move(&mdef(Point::new(x2, y2)));
        runner.mouse_up(&mdef(Point::new(x2, y2)));
        runner.update(false);
    }

    /// Default mouse event.
    fn mdef(pos: Point) -> MouseEvent {
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
}
