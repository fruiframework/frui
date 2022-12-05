use frui::prelude::*;

use rand::Rng;

/// This widget will display a different number every time its state is lost.
#[derive(ViewWidget)]
pub struct RandomState;

impl WidgetState for RandomState {
    type State = usize;

    fn create_state<'a>(&'a self) -> Self::State {
        rand::thread_rng().gen()
    }
}

impl ViewWidget for RandomState {
    fn build<'w>(&'w self, cx: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Text::new(cx.state().to_string())
    }
}

/// This widget will display a different number every time it gets rebuilt.
#[derive(ViewWidget)]
pub struct ChangesOnRebuild;

impl ViewWidget for ChangesOnRebuild {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        Text::new(rand::thread_rng().gen::<usize>().to_string())
    }
}
