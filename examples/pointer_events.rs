//! This example shows how to use [`PointerListener`] and [`PointerRegion`].

#![feature(type_alias_impl_trait)]

use frui::prelude::*;
use frui::render::*;

#[derive(ViewWidget)]
struct App;

impl WidgetState for App {
    type State = Stats;

    fn create_state(&self) -> Self::State {
        Stats::default()
    }
}

impl ViewWidget for App {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Stack::builder().children((
            // Stats:
            Positioned::builder()
                .left(30.)
                .top(30.)
                .child(cx.state().clone()),
            Center::child(
                PointerListener::builder()
                    .on_pointer_down(|_| cx.state_mut().down_count += 1)
                    .on_pointer_up(|_| cx.state_mut().up_count += 1)
                    .on_pointer_scroll(|_| cx.state_mut().scroll_count += 1)
                    .child(
                        PointerRegion::builder()
                            .on_enter(|_| cx.state_mut().enter_count += 1)
                            .on_exit(|_| cx.state_mut().exit_count += 1)
                            .on_move(|e| cx.state_mut().pointer_pos = e.0.pos)
                            .child(
                                Container::builder()
                                    .width(100.)
                                    .height(100.)
                                    .color(Color::SILVER)
                                    .child(()),
                            ),
                    ),
            ),
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
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Text::new(format!("{:#?}", self))
    }
}

fn main() {
    run_app(App);
}
