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
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        DebugContainer::child(
            Column::builder()
                .space_between(10.0)
                .main_axis_size(MainAxisSize::Max)
                .cross_axis_size(CrossAxisSize::Max)
                .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .cross_axis_alignment(CrossAxisAlignment::Center)
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
        app::runner::miri::MiriRunner,
        druid_shell::{keyboard_types::Key, Modifiers},
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
