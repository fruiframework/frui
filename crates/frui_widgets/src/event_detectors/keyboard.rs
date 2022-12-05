use druid_shell::KeyEvent;
use frui::{
    app::listeners::keyboard::{CallbackKey, KEYBOARD_EVENT_LISTENERS},
    prelude::*,
};

#[derive(ViewWidget)]
pub struct KeyboardEventDetector<W: Widget, F: Fn(KeyEvent)> {
    pub on_event: F,
    pub child: W,
}

impl<W: Widget, F: Fn(KeyEvent)> WidgetState for KeyboardEventDetector<W, F> {
    type State = Option<CallbackKey>;

    fn create_state<'a>(&'a self) -> Self::State {
        None
    }

    fn mount(&self, cx: BuildCx<Self>) {
        *cx.state_mut() = Some(
            KEYBOARD_EVENT_LISTENERS
                .with(|listeners| unsafe { listeners.borrow_mut().register(&self.on_event) }),
        );
    }

    fn unmount(&self, cx: BuildCx<Self>) {
        let mut key = cx.state_mut();
        KEYBOARD_EVENT_LISTENERS.with(|listeners| listeners.borrow_mut().unregister(&key.unwrap()));
        *key = None;
    }
}

impl<W: Widget, F: Fn(KeyEvent)> ViewWidget for KeyboardEventDetector<W, F> {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }
}
