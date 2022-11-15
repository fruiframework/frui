use druid_shell::kurbo::Rect;

use frui::prelude::*;

use crate::{Decoration, DecorationPosition, TextDirection};



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
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let size = ctx.child(0).layout(Constraints {
            max_width: self.width.unwrap_or(constraints.max_width),
            max_height: self.height.unwrap_or(constraints.max_height),
            ..constraints
        });

        Size {
            width: self.width.unwrap_or(size.width),
            height: self.height.unwrap_or(size.height),
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        if let Some(color) = &self.color {
            let brush = &canvas.solid_brush(color.clone());

            PietRenderContext::fill(canvas, Rect::from_origin_size(offset, ctx.size()), brush);
        }

        ctx.child(0).paint(canvas, offset)
    }
}

#[derive(RenderWidget)]
pub struct DecoratedBox<W: Widget, D: Decoration> {
    pub child: W,
    pub decoration: D,
    pub position: DecorationPosition,
}

impl<W: Widget, D: Decoration> RenderWidget for DecoratedBox<W, D> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
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