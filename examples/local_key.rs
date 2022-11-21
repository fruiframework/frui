//! This example shows how to annotate children of a [`RenderWidget`] in a way
//! that allows Frui to preserve the state of those widgets between rebuilds.
//!
//! [`RandomState`] is a widget that will generate and display a new number
//! every time its state has been reset. If you run this example you will be
//! able to notice that the state of [`RandomState`] widgets in each of the
//! [`Column`]s doesn't reset after you switch views (by clicking any key).
//!
//! This is because each of those widgets is annotated with [`LocalKey`] which
//! informs Frui that if it is possible it should try preserving the state of
//! that widget between rebuilds.

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
    fn build<'w>(&'w self, ctx: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        KeyboardEventDetector {
            on_event: |_| ctx.state_mut().switch(),
            child: if ctx.state().value() {
                Center::child(Column::builder().children((
                    LocalKey::new(1usize, RandomState),
                    Text::new("First Widget 🦀"),
                    LocalKey::new(2i32, RandomState),
                )))
                .boxed()
            } else {
                Center::child(Column::builder().children((
                    Text::new("First Widget 🦀"),
                    Text::new("Second Widget 🦀"),
                    LocalKey::new(1usize, RandomState),
                    LocalKey::new(2i32, RandomState),
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
