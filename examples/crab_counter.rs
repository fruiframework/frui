//! Crab Counter is an app that allows you to keep track of the number of crabs
//! you have! 🦀 🦀 🦀

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod button;

use button::Button;

#[derive(ViewWidget)]
struct CrabCounter;

impl WidgetState for CrabCounter {
    type State = isize;

    fn create_state(&self) -> Self::State {
        0
    }
}

impl ViewWidget for CrabCounter {
    fn build<'w>(&'w self, ctx: BuildCtx<'w, Self>) -> Self::Widget<'w> {
        Column::builder()
            .space_between(60.0)
            .main_axis_size(MainAxisSize::Max)
            .cross_axis_size(CrossAxisSize::Max)
            .main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .children((
                Text::new(format!("{} 🦀", *ctx.state()))
                    .size(100.0)
                    .weight(FontWeight::BOLD),
                // Todo: Make better construction methods:
                //
                // SizedBox::from_size((), Size::new(0.0, 60.0)),
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
            ))
    }
}

fn main() {
    run_app(CrabCounter);
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
        let mut runner = MiriRunner::new(CrabCounter);

        for _ in 0..4 {
            runner.key_down(KeyEvent::for_test(
                Modifiers::default(),
                Key::Character(" ".into()),
            ));
            runner.update(true);
        }
    }
}
