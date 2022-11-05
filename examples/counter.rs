//! This is an obligatory example of a counter app.

#![allow(unused_attributes)]
#![feature(type_alias_impl_trait)]

use frui::prelude::*;

#[path = "button.rs"]
mod button;

use button::Button;

#[derive(ViewWidget)]
pub struct Counter;

impl WidgetState for Counter {
    type State = isize;

    fn create_state(&self) -> Self::State {
        0
    }
}

impl ViewWidget for Counter {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
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
            ))
    }
}

#[allow(unused)]
fn main() {
    run_app(Counter);
}
