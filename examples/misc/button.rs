use std::cell::Cell;

use frui::prelude::*;

const WIDTH: f64 = 60.0;
const HEIGHT: f64 = 60.0;

const COLOR: Color = Color::rgb8(255, 144, 54);

#[derive(RenderWidget)]
pub struct Button<L: Widget, F: Fn()> {
    pub label: L,
    pub on_click: F,
}

impl<L: Widget, F: Fn()> RenderWidget for Button<L, F> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![Center { child: &self.label }]
    }

    fn layout(&self, ctx: RenderContext<Self>, _: Constraints) -> Size {
        ctx.child(0).layout(Constraints {
            min_width: 0.,
            max_width: WIDTH,
            min_height: 0.,
            max_height: HEIGHT,
        });

        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let color;
        let hover_shade = 0.2;

        if ctx.rstate().is_hovered.get() {
            let (mut r, mut g, mut b, a) = COLOR.as_rgba8();

            // User-chosen color darkened.
            r = (r as f32 * (1. - hover_shade)) as u8;
            g = (g as f32 * (1. - hover_shade)) as u8;
            b = (b as f32 * (1. - hover_shade)) as u8;

            color = Color::rgba8(r, g, b, a);
        } else {
            color = COLOR.clone();
        }

        let brush = &canvas.solid_brush(color);

        PietRenderContext::fill(
            canvas,
            RoundedRect::new(offset.x, offset.y, offset.x + WIDTH, offset.y + HEIGHT, 15.),
            brush,
        );

        ctx.child(0).paint(canvas, offset)
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
