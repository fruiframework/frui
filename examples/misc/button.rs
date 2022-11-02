use std::cell::Cell;

use frui::prelude::*;

const WIDTH: f64 = 60.0;
const HEIGHT: f64 = 60.0;

const COLOR: Color = Color::rgb8(255, 144, 54);

#[derive(ViewWidget)]
pub struct Button<L: Widget, F: Fn()> {
    pub label: L,
    pub on_click: F,
}

impl<L: Widget, F: Fn()> ViewWidget for Button<L, F> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        PointerRegion::builder()
            .on_enter(|e| log::info!("enter {}", e.0.pos))
            .on_move(|e| log::info!("move {}", e.0.pos))
            .on_exit(|e| log::info!("exit {}", e.0.pos))
            .child(
                Container::builder()
                    .width(WIDTH)
                    .height(HEIGHT)
                    .color(COLOR)
                    .child(Center::child(&self.label)),
            )
    }
}

pub struct ButtonRenderState {
    pub(crate) is_pressed: Cell<bool>,
    pub(crate) is_hovered: Cell<bool>,
}

impl<L: Widget, F: Fn()> RenderState for Button<L, F> {
    type State = ButtonRenderState;

    fn create_state(&self) -> Self::State {
        ButtonRenderState {
            is_pressed: Cell::new(false),
            is_hovered: Cell::new(false),
        }
    }
}

impl<L: Widget, F: Fn()> WidgetEvent for Button<L, F> {
    fn handle_event(&self, ctx: RenderContext<Self>, event: &Event) -> bool {
        match event {
            Event::MouseDown(e) => {
                if let MouseButton::Left = e.button {
                    if ctx.point_in_layout_bounds(e.pos) {
                        ctx.rstate().is_pressed.set(true);
                    }
                }
            }
            Event::MouseUp(e) => {
                if let MouseButton::Left = e.button {
                    if ctx.rstate().is_pressed.replace(false) {
                        // Call user-defined callback.
                        (self.on_click)();
                    }
                }
            }
            Event::MouseMove(e) => {
                if ctx.point_in_layout_bounds(e.pos) {
                    if !ctx.rstate().is_hovered.replace(true) {
                        // Repaint only if the hover state changed.
                        ctx.schedule_layout();
                    }
                } else {
                    if ctx.rstate().is_hovered.replace(false) {
                        // Repaint only if the hover state changed.
                        ctx.schedule_layout();
                    }
                }
            }
            _ => {}
        };

        if let Event::MouseDown(_) | Event::MouseUp(_) | Event::MouseMove(_) = event {
            true
        } else {
            false
        }
    }
}
