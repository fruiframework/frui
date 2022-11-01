//! This example shows how the order of children of a [`RenderWidget`]
//! affects whether the state of those children widgets will be preserved.
//!
//! [`RandomState`] is a widget that will generate and display a new number
//! every time its state has been reset. If you run this example you will be
//! able to notice that the state of [`RandomState`] widgets in each of the
//! [`Column`]s doesn't reset after you switch views (by clicking any key).
//!
//! This is because that [`RandomState`] in both of the views is a 3rd child.
//! Changing the order of that widget between children will ultimately cause
//! [`RandomState`] to lose its state.
//!
//! To avoid this, see `local_key` example which shows how you can annotate
//! stateful widgets in a way that will preserve their state even if their
//! order in children list changes.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::{RandomState, Switch};

#[derive(ViewWidget)]
struct App;

impl WidgetState for App {
    type State = Switch;

    fn create_state(&self) -> Self::State {
        Switch::default()
    }
}

impl ViewWidget for App {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        KeyboardEventDetector {
            on_event: |_| ctx.state_mut().switch(),
            child: if ctx.state().value() {
                Center::child(Column::builder().children((
                    Text::new("First child 🦀"),
                    Text::new("Second child 🦀"),
                    RandomState, // <-- 3rd child
                )))
                .boxed()
            } else {
                Center::child(Column::builder().children((
                    Text::new("First child 🦀"),
                    // Following widget makes `RandomState` a third child of the Column.
                    // If you delete it, when you run this example you will be able to
                    // see that its state is not preserved (and number changes).
                    (),
                    RandomState, // <-- 3rd child
                )))
                .boxed()
            },
        }
    }
}

fn main() {
    run_app(App);
}

#[cfg(all(test, feature = "miri"))]
mod test {
    use super::*;
    use frui::{
        app::runner::miri::MiriAppRunner,
        druid_shell::{keyboard_types::Key, Modifiers},
    };

    #[test]
    pub fn run_app_under_miri() {
        let mut runner = MiriAppRunner::new(App);

        for _ in 0..4 {
            runner.send_keyboard_event(KeyEvent::for_test(
                Modifiers::default(),
                Key::Character(" ".into()),
            ));
            runner.update();
        }
    }
}
