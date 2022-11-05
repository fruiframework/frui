//! This example shows how to use [`PointerListener`] and [`PointerRegion`].

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

#[derive(ViewWidget)]
struct App;

impl WidgetState for App {
    type State = Stats;

    fn create_state(&self) -> Self::State {
        Stats::default()
    }
}

impl ViewWidget for App {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Stack::builder().children((
            Center::child(
                PointerListener::builder()
                    .on_pointer_down(|_| ctx.state_mut().down_count += 1)
                    .on_pointer_up(|_| ctx.state_mut().up_count += 1)
                    .on_pointer_scroll(|_| ctx.state_mut().scroll_count += 1)
                    .child(
                        PointerRegion::builder()
                            .on_enter(|_| ctx.state_mut().enter_count += 1)
                            .on_exit(|_| ctx.state_mut().exit_count += 1)
                            .on_move(|e| ctx.state_mut().pointer_pos = e.0.pos)
                            .child(
                                Container::builder()
                                    .width(100.)
                                    .height(100.)
                                    .color(Color::SILVER)
                                    .child(()),
                            ),
                    ),
            ),
            // Stats:
            ctx.state().clone(),
        ))
    }
}

#[derive(ViewWidget, Debug, Default, Clone)]
struct Stats {
    pointer_pos: Point,
    up_count: usize,
    down_count: usize,
    scroll_count: usize,
    enter_count: usize,
    exit_count: usize,
}

impl ViewWidget for Stats {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        Text::new(format!("{:#?}", self))
    }
}

fn main() {
    run_app(App);
}
