use std::{borrow::Cow, marker::PhantomData, sync::Mutex};

use druid_shell::{
    piet::{CoreGraphicsImage, CoreGraphicsText, CoreGraphicsTextLayout, IntoBrush, PietText},
    Cursor, IdleToken,
};

pub static REQUEST_ANIM_FRAME: Mutex<bool> = Mutex::new(false);

pub static SCHEDULE_IDLE: Mutex<Vec<IdleToken>> = Mutex::new(Vec::new());

/// Placeholder for [`IdleHandle`](druid_shell::IdleHandle) that allows us to test Frui in Miri.
pub struct IdleHandle {}

impl IdleHandle {
    pub fn schedule_idle(&self, token: IdleToken) {
        SCHEDULE_IDLE.lock().unwrap().push(token)
    }
}

/// Placeholder for [`Application`](druid_shell::Application) that allows us to test Frui in Miri.
pub struct Application {}

impl Application {
    pub fn global() -> Self {
        Self {}
    }

    pub fn quit(&self) {}
}

/// Placeholder for [`WindowHandle`](druid_shell::WindowHandle) that allows us to test Frui in Miri.
#[derive(Default, Clone)]
pub struct WindowHandle {}

impl WindowHandle {
    pub fn request_anim_frame(&mut self) {
        *REQUEST_ANIM_FRAME.lock().unwrap() = true;
    }

    pub fn get_idle_handle(&self) -> Option<IdleHandle> {
        Some(IdleHandle {})
    }

    pub fn set_cursor(&self, _: &Cursor) {}

    pub fn invalidate(&self) {}

    pub fn close(&self) {}

    #[track_caller]
    pub fn text(&self) -> PietText {
        todo!()
    }
}

/// Placeholder for [`Piet`](druid_shell::piet::Piet) that allows us to test Frui in Miri.
#[derive(Default)]
pub struct Canvas<'a> {
    _p: PhantomData<&'a ()>,
}

#[allow(unused)]
impl druid_shell::piet::RenderContext for Canvas<'_> {
    type Brush = Brush;

    type Text = CoreGraphicsText;

    type TextLayout = CoreGraphicsTextLayout;

    type Image = CoreGraphicsImage;

    fn status(&mut self) -> Result<(), druid_shell::piet::Error> {
        todo!()
    }

    fn solid_brush(&mut self, color: druid_shell::piet::Color) -> Self::Brush {
        Brush
    }

    fn gradient(
        &mut self,
        gradient: impl Into<druid_shell::piet::FixedGradient>,
    ) -> Result<Self::Brush, druid_shell::piet::Error> {
        todo!()
    }

    fn clear(&mut self, color: druid_shell::piet::Color) {
        todo!()
    }

    fn stroke(
        &mut self,
        shape: impl druid_shell::kurbo::Shape,
        brush: &impl druid_shell::piet::IntoBrush<Self>,
        width: f64,
    ) {
        todo!()
    }

    fn stroke_styled(
        &mut self,
        shape: impl druid_shell::kurbo::Shape,
        brush: &impl druid_shell::piet::IntoBrush<Self>,
        width: f64,
        style: &druid_shell::piet::StrokeStyle,
    ) {
    }

    fn fill(
        &mut self,
        shape: impl druid_shell::kurbo::Shape,
        brush: &impl druid_shell::piet::IntoBrush<Self>,
    ) {
    }

    fn fill_even_odd(
        &mut self,
        shape: impl druid_shell::kurbo::Shape,
        brush: &impl druid_shell::piet::IntoBrush<Self>,
    ) {
        todo!()
    }

    fn clip(&mut self, shape: impl druid_shell::kurbo::Shape) {}

    fn text(&mut self) -> &mut Self::Text {
        todo!()
    }

    fn draw_text(&mut self, layout: &Self::TextLayout, pos: impl Into<druid_shell::kurbo::Point>) {
        todo!()
    }

    fn save(&mut self) -> Result<(), druid_shell::piet::Error> {
        // Todo: This is incorrect, we should create a stack for this.
        Ok(())
    }

    fn restore(&mut self) -> Result<(), druid_shell::piet::Error> {
        // Todo: This is incorrect, we should create a stack for this.
        Ok(())
    }

    fn finish(&mut self) -> Result<(), druid_shell::piet::Error> {
        todo!()
    }

    fn transform(&mut self, transform: druid_shell::kurbo::Affine) {}

    fn make_image(
        &mut self,
        width: usize,
        height: usize,
        buf: &[u8],
        format: druid_shell::piet::ImageFormat,
    ) -> Result<Self::Image, druid_shell::piet::Error> {
        todo!()
    }

    fn draw_image(
        &mut self,
        image: &Self::Image,
        dst_rect: impl Into<druid_shell::kurbo::Rect>,
        interp: druid_shell::piet::InterpolationMode,
    ) {
        todo!()
    }

    fn draw_image_area(
        &mut self,
        image: &Self::Image,
        src_rect: impl Into<druid_shell::kurbo::Rect>,
        dst_rect: impl Into<druid_shell::kurbo::Rect>,
        interp: druid_shell::piet::InterpolationMode,
    ) {
        todo!()
    }

    fn blurred_rect(
        &mut self,
        rect: druid_shell::kurbo::Rect,
        blur_radius: f64,
        brush: &impl druid_shell::piet::IntoBrush<Self>,
    ) {
        todo!()
    }

    fn current_transform(&self) -> druid_shell::kurbo::Affine {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Brush;

impl IntoBrush<Canvas<'_>> for Brush {
    fn make_brush<'a>(
        &'a self,
        _piet: &mut Canvas<'_>,
        _bbox: impl FnOnce() -> druid_shell::kurbo::Rect,
    ) -> Cow<'a, <Canvas<'_> as druid_shell::piet::RenderContext>::Brush> {
        Cow::Owned(Brush)
    }
}
