//! This example shows how to construct a simple [`Button`] widget.

#![allow(unused_attributes)]
#![feature(type_alias_impl_trait)]

use frui::prelude::*;

const WIDTH: f64 = 60.0;
const HEIGHT: f64 = 60.0;

const COLOR: Color = Color::rgb8(255, 144, 54);

#[derive(ViewWidget)]
pub struct Button<L: Widget, F: Fn()> {
    pub label: L,
    pub on_click: F,
}

pub struct ButtonState {
    is_pressed: bool,
    is_hovered: bool,
}

impl<L: Widget, F: Fn()> WidgetState for Button<L, F> {
    type State = ButtonState;

    fn create_state(&self) -> Self::State {
        ButtonState {
            is_pressed: false,
            is_hovered: false,
        }
    }
}

impl<L: Widget, F: Fn()> ViewWidget for Button<L, F> {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w> {
        let (is_hovered, is_pressed) = (cx.state().is_hovered, cx.state().is_pressed);

        let color = if is_pressed {
            COLOR.darken().darken()
        } else if is_hovered {
            COLOR.darken()
        } else {
            COLOR
        };

        PointerListener::builder()
            .on_pointer_down(|_| cx.state_mut().is_pressed = true)
            .on_pointer_up(|_| {
                cx.state_mut().is_pressed = false;

                if cx.state().is_hovered {
                    (self.on_click)();
                }
            })
            .child(
                PointerRegion::builder()
                    .on_enter(|_| cx.state_mut().is_hovered = true)
                    .on_exit(|_| cx.state_mut().is_hovered = false)
                    .child(SizedBox::from_size(
                        DecoratedBox::builder()
                            .position(DecorationPosition::Background)
                            .decoration(
                                BoxDecoration::builder()
                                    .color(color)
                                    .border_radius(BorderRadius::circular(15.0)),
                            )
                            .child(Center::child(&self.label)),
                        Size::new(WIDTH, HEIGHT),
                    )),
            )
    }
}

trait ColorExt: Sized {
    fn darken(&self) -> Color;
}

impl ColorExt for Color {
    fn darken(&self) -> Color {
        let darken_amount = 0.2;

        let (mut r, mut g, mut b, a) = self.as_rgba8();

        // User-chosen color darkened.
        r = (r as f32 * (1. - darken_amount)) as u8;
        g = (g as f32 * (1. - darken_amount)) as u8;
        b = (b as f32 * (1. - darken_amount)) as u8;

        Color::rgba8(r, g, b, a)
    }
}

#[allow(unused)]
fn main() {
    run_app(Button {
        label: Text::new(":)"),
        on_click: || println!("clicked!"),
    });
}

#[cfg(test)]
#[cfg(feature = "miri")]
mod test {
    use super::*;

    use frui::app::runner::miri::MiriRunner;
    use frui::druid_shell::{Modifiers, MouseButton, MouseButtons, MouseEvent};
    use frui::render::*;

    static COUNT: std::sync::Mutex<isize> = std::sync::Mutex::new(0);

    #[derive(ViewWidget)]
    pub struct OnlyButtons;

    impl WidgetState for OnlyButtons {
        type State = isize;

        fn create_state(&self) -> Self::State {
            0
        }
    }

    impl ViewWidget for OnlyButtons {
        fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w> {
            *COUNT.lock().unwrap() = *cx.state();

            UnconstrainedBox {
                child: Row::builder().space_between(10.0).children((
                    Button {
                        label: (),
                        on_click: || *cx.state_mut() += 1,
                    },
                    Button {
                        label: (),
                        on_click: || *cx.state_mut() -= 1,
                    },
                )),
            }
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
