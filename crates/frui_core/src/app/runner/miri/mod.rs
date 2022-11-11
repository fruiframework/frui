use druid_shell::{
    kurbo::{Rect, Size},
    KeyEvent, MouseEvent, Region,
};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use super::{window_handler::WindowHandler, FruiWindowHandler};
use crate::prelude::Widget;

mod substitutes;
pub use substitutes::*;

pub struct MiriRunner {
    last_size: Size,
    handler: WindowHandler,
}

impl MiriRunner {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        // Enable debug logging:
        let _ = TermLogger::init(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::AlwaysAnsi,
        );

        let mut window_handler = WindowHandler::new(widget);

        // Execute application startup sequence:

        window_handler.connect(&WindowHandle {});
        window_handler.size(Size::new(500., 400.));
        window_handler.prepare_paint();
        window_handler.paint(
            &mut Canvas::default(),
            &default_region(Size::new(500., 400.)),
        );

        let mut this = MiriRunner {
            last_size: Size::new(500., 400.),
            handler: window_handler,
        };

        // First update.
        this.update(true);

        this
    }

    pub fn update(&mut self, force_repaint: bool) {
        for token in SCHEDULE_IDLE.lock().unwrap().drain(..) {
            self.handler.idle(token);
        }

        if *REQUEST_ANIM_FRAME.lock().unwrap() || force_repaint {
            self.handler.prepare_paint();
            self.handler
                .paint(&mut Canvas::default(), &default_region(self.last_size));

            *REQUEST_ANIM_FRAME.lock().unwrap() = false;
        }
    }

    //
    // Pass window events:

    pub fn mouse_down(&mut self, event: &MouseEvent) {
        self.handler.mouse_down(&event);
    }

    pub fn mouse_move(&mut self, event: &MouseEvent) {
        self.handler.mouse_move(&event);
    }

    pub fn mouse_up(&mut self, event: &MouseEvent) {
        self.handler.mouse_up(&event);
    }

    pub fn key_down(&mut self, event: KeyEvent) {
        self.handler.key_down(event);
    }

    pub fn size(&mut self, size: druid_shell::kurbo::Size) {
        self.handler.size(size);
        self.update(true);
    }
}

fn default_region(window_size: Size) -> Region {
    let mut region = Region::EMPTY;
    region.add_rect(Rect::new(0., 0., window_size.width, window_size.height));
    region
}
