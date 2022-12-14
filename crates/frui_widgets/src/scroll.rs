//! This is a bad prototype.

use frui::prelude::*;
use frui::render::*;

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
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size {
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

        cx.child(0).layout(child_constraints);

        constraints.biggest()
    }

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset) {
        if let Err(e) = canvas.save() {
            log::error!("saving render context failed: {:?}", e);
            return;
        }

        let viewport = Rect::from_origin_size(*offset, cx.size());
        canvas.clip(druid_shell::piet::kurbo::Rect::from(viewport));
        canvas.transform(Affine::translate(-cx.widget_state().scroll_offset));

        cx.child(0).paint(canvas, offset);

        if let Err(e) = canvas.restore() {
            log::error!("restoring render context failed: {:?}", e);
        }

        // Todo: Draw scroll bar.
    }
}
