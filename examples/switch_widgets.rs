//! This example shows how to construct a simple widget which will switch
//! between two views.
//!
//! [`Switch`] is a simple structure holding [`bool`] which provides
//! convenience method `switch()` which toggles between two states of a
//! [`bool`].
//!
//! Each of the views needs to be boxed, because of the way Rust treats return
//! types. To achieve this `boxed()` helper method is used which wraps given
//! widget with a Box and type erases it.
//!
//! [`KeyboardEventDetector`] is a widget which allows its consumers to react
//! to keyboard events through a callback provided in `on_event`.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::Switch;

#[derive(ViewWidget)]
struct App;

impl WidgetState for App {
    type State = Switch;

    fn create_state(&self) -> Self::State {
        Switch::default()
    }
}

impl ViewWidget for App {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w> {
        KeyboardEventDetector {
            on_event: |_| cx.state_mut().switch(),
            child: match cx.state().value() {
                true => Text::new("Top Left").boxed(),
                false => Center::child(Text::new("Centered")).boxed(),
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
        app::runner::miri::MiriRunner,
        druid_shell::{keyboard_types::Key, KeyEvent, Modifiers},
    };

    #[test]
    pub fn run_example_under_miri() {
        let mut runner = MiriRunner::new(App);

        for _ in 0..4 {
            runner.key_down(KeyEvent::for_test(
                Modifiers::default(),
                Key::Character(" ".into()),
            ));
            runner.update(true);
        }
    }
}
