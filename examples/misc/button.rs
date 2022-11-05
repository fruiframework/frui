use frui::prelude::*;

const WIDTH: f64 = 60.0;
const HEIGHT: f64 = 60.0;

const COLOR: Color = Color::rgb8(255, 144, 54);

#[derive(ViewWidget)]
pub struct Button<L: Widget, F: Fn()> {
    pub label: L,
    pub on_click: F,
}

pub struct ButtonState {
    is_pressed: bool,
    is_hovered: bool,
}

impl<L: Widget, F: Fn()> WidgetState for Button<L, F> {
    type State = ButtonState;

    fn create_state(&self) -> Self::State {
        ButtonState {
            is_pressed: false,
            is_hovered: false,
        }
    }
}

impl<L: Widget, F: Fn()> ViewWidget for Button<L, F> {
    fn build<'w>(&'w self, ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        let (is_hovered, is_pressed) = (ctx.state().is_hovered, ctx.state().is_pressed);

        let color = if is_pressed {
            COLOR.darken().darken()
        } else if is_hovered {
            COLOR.darken()
        } else {
            COLOR
        };

        PointerListener::builder()
            .on_pointer_down(|_| ctx.state_mut().is_pressed = true)
            .on_pointer_up(|_| {
                ctx.state_mut().is_pressed = false;

                if ctx.state().is_hovered {
                    (self.on_click)();
                }
            })
            .child(
                PointerRegion::builder()
                    .on_enter(|_| ctx.state_mut().is_hovered = true)
                    .on_exit(|_| ctx.state_mut().is_hovered = false)
                    .child(
                        Container::builder()
                            .width(WIDTH)
                            .height(HEIGHT)
                            .color(color)
                            .child(Center::child(&self.label)),
                    ),
            )
    }
}

trait ColorExt: Sized {
    fn darken(&self) -> Color;
}

impl ColorExt for Color {
    fn darken(&self) -> Color {
        let darken_amount = 0.2;

        let (mut r, mut g, mut b, a) = self.as_rgba8();

        // User-chosen color darkened.
        r = (r as f32 * (1. - darken_amount)) as u8;
        g = (g as f32 * (1. - darken_amount)) as u8;
        b = (b as f32 * (1. - darken_amount)) as u8;

        Color::rgba8(r, g, b, a)
    }
}
