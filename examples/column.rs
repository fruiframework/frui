//! This example shows usage of the [`Column`] widget and its different options.
//!
//! [`DebugContainer`] is used to visualize layout bounds of the [`Column`]
//! widget.
//!
//! Feel free to modify each of the properties of the [`Column`] to see how it
//! affects the way its children are laid out!

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::flex_children as list;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        DebugContainer::new(
            Column::builder()
                .space_between(20.0)
                .text_direction(TextDirection::Ltr)
                .vertical_direction(VerticalDirection::Up)
                .main_axis_size(MainAxisSize::Min)
                .cross_axis_size(CrossAxisSize::Min)
                .main_axis_alignment(MainAxisAlignment::Center)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .children(list::flexible()),
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
