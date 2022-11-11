use druid_shell::{IdleToken, KeyEvent, MouseEvent};

pub mod window_handler;

#[cfg(feature = "miri")]
pub mod miri;
pub mod native;

#[cfg(feature = "miri")]
pub type IdleHandle = miri::IdleHandle;
#[cfg(not(feature = "miri"))]
pub type IdleHandle = druid_shell::IdleHandle;

#[cfg(feature = "miri")]
pub type Application = miri::Application;
#[cfg(not(feature = "miri"))]
pub type Application = druid_shell::Application;

#[cfg(feature = "miri")]
pub type WindowHandle = miri::WindowHandle;
#[cfg(not(feature = "miri"))]
pub type WindowHandle = druid_shell::WindowHandle;

#[cfg(feature = "miri")]
pub type PaintCtx<'a> = miri::PaintCtx<'a>;
#[cfg(not(feature = "miri"))]
pub type Canvas<'a> = druid_shell::piet::Piet<'a>;

/// Wrapper around [`druid_shell::WinHandler`] that allows us to run tests in Miri.
/// This implementation can be called by both [`MiriRunner`] and [`druid_shell::WinHandler`].
pub trait FruiWindowHandler {
    fn connect(&mut self, handle: &WindowHandle);

    fn prepare_paint(&mut self);

    fn paint(&mut self, piet: &mut Canvas, invalid: &druid_shell::Region);

    fn size(&mut self, size: druid_shell::kurbo::Size);

    fn idle(&mut self, token: IdleToken);

    fn destroy(&mut self);

    fn as_any(&mut self) -> &mut dyn std::any::Any;

    // Events:

    fn mouse_down(&mut self, event: &MouseEvent);

    fn mouse_up(&mut self, event: &MouseEvent);

    fn mouse_move(&mut self, event: &MouseEvent);

    fn wheel(&mut self, event: &MouseEvent);

    fn key_down(&mut self, event: KeyEvent) -> bool;

    fn request_close(&mut self);
}
