//! This is a bad prototype.

use frui::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
    // Todo: All,
}

/// Todo: Finish implementation.
#[derive(SingleChildWidget, Builder)]
pub struct Scroll<W: Widget> {
    pub child: W,
    pub scroll_direction: ScrollDirection,
}

impl Scroll<()> {
    pub fn builder() -> Scroll<()> {
        Scroll {
            child: (),
            scroll_direction: ScrollDirection::Vertical,
        }
    }
}

#[doc(hidden)]
pub struct ScrollState {
    scroll_offset: Vec2,
}

impl<W: Widget> WidgetState for Scroll<W> {
    type State = ScrollState;

    fn create_state(&self) -> Self::State {
        ScrollState {
            scroll_offset: Vec2::new(0., 0.),
        }
    }
}

impl<W: Widget> SingleChildWidget for Scroll<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_constraints = match self.scroll_direction {
            ScrollDirection::Horizontal => Constraints {
                min_width: 0.,
                max_width: f64::INFINITY,
                ..constraints
            },
            ScrollDirection::Vertical => Constraints {
                min_height: 0.,
                max_height: f64::INFINITY,
                ..constraints
            },
        };

        ctx.child().layout(child_constraints);

        constraints.max()
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        if let Err(e) = canvas.save() {
            log::error!("saving render context failed: {:?}", e);
            return;
        }

        let viewport = Rect::from_origin_size(offset, ctx.size());
        canvas.clip(viewport);
        canvas.transform(Affine::translate(-ctx.wstate().scroll_offset));

        ctx.child().paint(canvas, offset);

        if let Err(e) = canvas.restore() {
            log::error!("restoring render context failed: {:?}", e);
        }

        // Todo: Draw scroll bar.
    }
}

impl<W: Widget> WidgetEvent for Scroll<W> {
    fn handle_event(&self, ctx: RenderContext<Self>, event: &Event) -> bool {
        // Todo: Transform event into child coordinates.

        let viewport = Rect::from_origin_size(ctx.offset(), ctx.size());

        let event = event
            .transform_scroll(ctx.wstate().scroll_offset, viewport)
            .unwrap();

        ctx.child().handle_event(&event);

        if let Event::MouseWheel(event) = event {
            let size = ctx.size();
            let child_size = ctx.child().size();

            scroll(
                &mut ctx.wstate_mut().scroll_offset,
                child_size,
                event.wheel_delta,
                size,
            );
        }

        true
    }
}

/// Update the scroll.
///
/// Returns `true` if the scroll has been updated.
pub fn scroll(scroll_offset: &mut Vec2, child_size: Size, delta: Vec2, size: Size) {
    let mut offset = *scroll_offset + delta;

    offset.x = offset.x.min(child_size.width - size.width).max(0.0);
    offset.y = offset.y.min(child_size.height - size.height).max(0.0);

    if (offset - *scroll_offset).hypot2() > 1e-12 {
        *scroll_offset = offset;
    }
}
