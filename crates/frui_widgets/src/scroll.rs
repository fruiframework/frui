//! This is a bad prototype.

use frui::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
    // Todo: All,
}

/// Todo: Finish implementation.
#[derive(RenderWidget, Builder)]
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

impl<W: Widget> RenderWidget for Scroll<W> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
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

        ctx.child(0).layout(child_constraints);

        constraints.biggest()
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        if let Err(e) = canvas.save() {
            log::error!("saving render context failed: {:?}", e);
            return;
        }

        let viewport = Rect::from_origin_size(offset, ctx.size());
        canvas.clip(viewport);
        canvas.transform(Affine::translate(-ctx.widget_state().scroll_offset));

        ctx.child(0).paint(canvas, offset);

        if let Err(e) = canvas.restore() {
            log::error!("restoring render context failed: {:?}", e);
        }

        // Todo: Draw scroll bar.
    }
}
