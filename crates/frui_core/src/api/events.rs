//! Following events implementation is a bad prototype.

use druid_shell::{
    kurbo::{Rect, Shape, Vec2},
    MouseEvent,
};
use frui_macros::sealed;

use crate::{macro_exports::AnyRenderContext, prelude::RenderContext};

use super::contexts::render_ctx::_RenderContext;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Event {
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    MouseWheel(MouseEvent),
}

impl Event {
    pub fn transform_scroll(&self, offset: Vec2, viewport: Rect) -> Option<Event> {
        match self {
            Event::MouseDown(mouse_event) => {
                if viewport.winding(mouse_event.pos) != 0 {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos += offset;
                    Some(Event::MouseDown(mouse_event))
                } else {
                    None
                }
            }
            Event::MouseUp(mouse_event) => {
                if viewport.winding(mouse_event.pos) != 0 {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos += offset;
                    Some(Event::MouseUp(mouse_event))
                } else {
                    None
                }
            }
            Event::MouseMove(mouse_event) => {
                if viewport.winding(mouse_event.pos) != 0 {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos += offset;
                    Some(Event::MouseMove(mouse_event))
                } else {
                    None
                }
            }
            Event::MouseWheel(mouse_event) => {
                if viewport.winding(mouse_event.pos) != 0 {
                    let mut mouse_event = mouse_event.clone();
                    mouse_event.pos += offset;
                    Some(Event::MouseWheel(mouse_event))
                } else {
                    None
                }
            } // _ => Some(self.clone()),
        }
    }
}

pub trait WidgetEvent: Sized {
    fn handle_event(&self, ctx: RenderContext<Self>, event: &Event) -> bool;
}

#[sealed(crate)]
pub trait WidgetEventOS {
    fn handle_event(&self, ctx: &mut AnyRenderContext, event: &Event) -> bool;
}

impl<T> WidgetEventOS for T {
    default fn handle_event(&self, _: &mut AnyRenderContext, _: &Event) -> bool {
        todo!("remove this");
        false
    }
}

impl<T: WidgetEvent> WidgetEventOS for T {
    fn handle_event(&self, ctx: &mut AnyRenderContext, event: &Event) -> bool {
        let ctx = &mut <_RenderContext<T>>::new(ctx);

        T::handle_event(self, ctx, event)
    }
}
