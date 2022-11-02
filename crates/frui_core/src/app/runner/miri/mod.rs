use druid_shell::{
    kurbo::{Rect, Size},
    KeyEvent,
    MouseEvent,
    Region,
};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use super::{window_handler::WindowHandler, FruiWindowHandler};
use crate::{app::listeners::keyboard::KEYBOARD_EVENT_LISTENERS, prelude::Widget};

mod substitutes;
pub use substitutes::*;

pub struct MiriAppRunner {
    handler: WindowHandler,
}

impl MiriAppRunner {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        // Enable debug logging:
        TermLogger::init(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::AlwaysAnsi,
        )
        .unwrap();

        let mut window_handler = WindowHandler::new(widget);

        // Execute application startup sequence:

        window_handler.connect(&WindowHandle {});
        window_handler.size(Size::new(500., 400.));
        window_handler.prepare_paint();
        window_handler.paint(&mut PaintContext::default(), &default_region());

        let mut this = MiriAppRunner {
            handler: window_handler,
        };

        // First update.
        this.update();

        this
    }

    pub fn update(&mut self) {
        self.handler.prepare_paint();
        self.handler
            .paint(&mut PaintContext::default(), &default_region());
    }

    pub fn mouse_down(&mut self, event: &MouseEvent) {
        self.handler.mouse_down(&event);
    }

    pub fn send_keyboard_event(&mut self, event: KeyEvent) {
        KEYBOARD_EVENT_LISTENERS.with(|listeners| {
            for listener in listeners.borrow_mut().iter() {
                listener(event.clone());
            }
        });
    }
}

fn default_region() -> Region {
    let mut region = Region::EMPTY;
    region.add_rect(Rect::new(0., 0., 500., 400.));
    region
}
