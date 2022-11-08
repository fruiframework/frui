//! This example shows usage of the [`Column`] widget and its different options.
//!
//! [`DebugContainer`] is used to visualize layout bounds of the [`Column`]
//! widget.
//!
//! Feel free to modify each of the properties of the [`Column`] to see how it
//! affects the way its children are laid out.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::children_combinations::Big;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        DebugContainer::child(
            Column::builder()
                .cross_axis_size(CrossAxisSize::Max)
                .main_axis_size(MainAxisSize::Max)
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .space_between(20.0)
                .children((
                    Big(Color::rgb8(13, 245, 152)),
                    Big(Color::rgb8(255, 0, 110)),
                    Big(Color::rgb8(0, 186, 255)),
                )),
        )
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
    pub fn run_example_under_miri() {
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
