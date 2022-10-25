use druid_shell::{
    kurbo::Size, piet::Piet, Application, IdleToken, KeyEvent, MouseEvent, Region, WinHandler,
    WindowBuilder, WindowHandle,
};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::prelude::Widget;

use super::{handler::WindowHandler, FruiWindowHandler};

// Currently there is `'static` lifetime requirement for the root widget
// because of the requirements of `WinHandle` from the druid_shell.
//
// In the future this requirement may be lifted.
pub fn run_app<'a>(widget: impl Widget + 'static) {
    if cfg!(feature = "miri") {
        panic!(concat!(
            "feature `miri` is enabled which is not supported for `run_app`. ",
            "Disable feature `miri` to use `run_app`. To test application using ",
            "Miri use `MiriRunner` instead."
        ));
    }

    // Enable debug logging:
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::AlwaysAnsi,
    )
    .unwrap();

    //
    // Run app:

    let app = Application::new().unwrap();

    let mut window = WindowBuilder::new(app.clone());
    window.set_handler(Box::new(WindowHandler::new(widget)));
    window.set_title("Frui App");

    let window = window.build().unwrap();

    window.show();
    app.run(None);

    drop(window);
}

impl druid_shell::AppHandler for WindowHandler {
    fn command(&mut self, id: u32) {
        println!("handle system command of id {id}")
    }
}

#[allow(unused)]
impl WinHandler for WindowHandler {
    fn connect(&mut self, handle: &WindowHandle) {
        #[cfg(feature = "miri")]
        unreachable!();
        #[cfg(not(feature = "miri"))]
        FruiWindowHandler::connect(self, handle);
    }

    fn prepare_paint(&mut self) {
        FruiWindowHandler::prepare_paint(self);
    }

    #[allow(unused)]
    fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        #[cfg(feature = "miri")]
        unreachable!();
        #[cfg(not(feature = "miri"))]
        FruiWindowHandler::paint(self, piet, invalid)
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        FruiWindowHandler::as_any(self)
    }

    fn size(&mut self, size: Size) {
        FruiWindowHandler::size(self, size)
    }

    fn idle(&mut self, token: IdleToken) {
        FruiWindowHandler::idle(self, token)
    }

    fn destroy(&mut self) {
        FruiWindowHandler::destroy(self)
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        FruiWindowHandler::mouse_down(self, event)
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        FruiWindowHandler::mouse_move(self, event)
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        FruiWindowHandler::mouse_up(self, event)
    }

    fn wheel(&mut self, event: &MouseEvent) {
        FruiWindowHandler::wheel(self, event)
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        FruiWindowHandler::key_down(self, event)
    }

    fn request_close(&mut self) {
        FruiWindowHandler::request_close(self)
    }
}
