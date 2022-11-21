use frui::prelude::*;
use frui::render::*;

use crate::{Decoration, DecorationPosition, TextDirection, BoxDecoration, DefaultBoxDecoration};



#[derive(RenderWidget)]
pub struct Container<W: Widget> {
    child: W,
    width: Option<f64>,
    height: Option<f64>,
    color: Option<Color>,
}

impl Container<()> {
    pub fn builder() -> Container<()> {
        Container {
            child: (),
            width: None,
            height: None,
            color: None,
        }
    }
}

impl<W: Widget> Container<W> {
    pub fn child<C: Widget>(self, child: C) -> Container<C> {
        Container {
            child,
            width: self.width,
            height: self.height,
            color: self.color,
        }
    }

    #[track_caller]
    pub fn width(mut self, width: f64) -> Self {
        assert!(width >= 0.0, "width must be >= 0.0");
        self.width = Some(width);
        self
    }

    #[track_caller]
    pub fn height(mut self, height: f64) -> Self {
        assert!(height >= 0.0, "height must be >= 0.0");
        self.height = Some(height);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<W: Widget> RenderWidget for Container<W> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let constraints = Constraints::new_tight_for(self.width, self.height).enforce(constraints);

        ctx.child(0).layout(constraints)
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        if let Some(color) = &self.color {
            let brush = &canvas.solid_brush(color.clone());
            canvas.fill(DruidRect::from_origin_size(offset, ctx.size()), brush);
        }

        ctx.child(0).paint(canvas, offset)
    }
}

#[derive(RenderWidget, Builder)]
pub struct DecoratedBox<W: Widget, D: Decoration> {
    pub child: W,
    pub decoration: D,
    pub position: DecorationPosition,
}

impl DecoratedBox<(), DefaultBoxDecoration> {
    pub fn builder() -> Self {
        Self {
            child: (),
            decoration: BoxDecoration::builder(),
            position: DecorationPosition::Background,
        }
    }
}

impl<W: Widget, D: Decoration> RenderWidget for DecoratedBox<W, D> {
    fn build<'w>(&'w self, _ctx: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        self.decoration.padding().inflate_size(ctx.child(0).layout(constraints))
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let rect = Rect::from_origin_size(offset, ctx.size());
        let path = self.decoration.get_clip_path(rect.into(), &TextDirection::Ltr);
        if self.position == DecorationPosition::Background {
            self.decoration.paint(canvas, rect.into(), offset);
        }
        canvas.with_save(|c| {
            c.clip(path);
            ctx.child(0).paint(c, offset);
            Ok(())
        }).unwrap();
        if self.position == DecorationPosition::Foreground {
            self.decoration.paint(canvas, rect.into(), offset);
        }
    }
}