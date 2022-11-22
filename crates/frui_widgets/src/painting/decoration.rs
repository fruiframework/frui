use druid_shell::{
    kurbo::{BezPath, Circle, RoundedRect},
    piet::{kurbo::Shape, RenderContext},
};
use frui::{
    prelude::*,
    render::{Canvas, Offset, Rect},
};

use crate::{
    border_radius::BorderRadius, box_border::BoxShape, BoxBorder, BoxShadow, Directional,
    EdgeInsets, ShapeBorder, TextDirection, EPSILON,
};

pub trait BoxPainter {
    fn paint<T>(&self, canvas: &mut Canvas, offset: Offset);
}

pub trait Decoration {
    fn padding(&self) -> EdgeInsets;

    fn get_clip_path(&self, rect: Rect, text_direction: &TextDirection) -> BezPath;

    fn paint(&self, canvas: &mut Canvas, rect: Rect, offset: &Offset);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DecorationPosition {
    Background,
    Foreground,
}

pub struct BoxDecoration<B, BR>
where
    B: Directional<Output = BoxBorder>,
    BR: Directional<Output = BorderRadius>,
{
    pub color: Option<Color>,
    pub box_shadow: Vec<BoxShadow>,
    // TODO: image background
    // pub image: Option<Image>,
    pub border: Option<B>,
    pub border_radius: Option<BR>,
    // FIXME: implement gradient or merge it with color
    // pub gradient: Option<G>,
    pub shape: BoxShape,
    pub text_direction: TextDirection,
}

pub type DefaultBoxDecoration = BoxDecoration<BoxBorder, BorderRadius>;

impl BoxDecoration<BoxBorder, BorderRadius> {
    pub fn builder() -> DefaultBoxDecoration {
        Self {
            color: None,
            box_shadow: Vec::new(),
            border: None,
            border_radius: None,
            shape: BoxShape::Rectangle,
            text_direction: TextDirection::Ltr,
        }
    }
}

impl<B, BR> BoxDecoration<B, BR>
where
    B: Directional<Output = BoxBorder>,
    BR: Directional<Output = BorderRadius>,
{
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn box_shadow(mut self, box_shadow: Vec<BoxShadow>) -> Self {
        self.box_shadow.clear();
        self.box_shadow.extend(box_shadow);
        self
    }

    pub fn border<BORDER>(self, border: BORDER) -> BoxDecoration<BORDER, BR>
    where
        BORDER: Directional<Output = BoxBorder>,
    {
        BoxDecoration::<BORDER, BR> {
            color: self.color,
            box_shadow: self.box_shadow,
            border: Some(border),
            border_radius: self.border_radius,
            shape: self.shape,
            text_direction: self.text_direction,
        }
    }

    pub fn border_radius<RADIUS>(self, border_radius: RADIUS) -> BoxDecoration<B, RADIUS>
    where
        RADIUS: Directional<Output = BorderRadius>,
    {
        BoxDecoration::<B, RADIUS> {
            color: self.color,
            box_shadow: self.box_shadow,
            border: self.border,
            border_radius: Some(border_radius),
            shape: self.shape,
            text_direction: self.text_direction,
        }
    }

    pub fn shape(mut self, shape: BoxShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn text_direction(mut self, text_direction: TextDirection) -> Self {
        self.text_direction = text_direction;
        self
    }
}

impl<B, BR> Decoration for BoxDecoration<B, BR>
where
    B: Directional<Output = BoxBorder>,
    BR: Directional<Output = BorderRadius>,
{
    fn padding(&self) -> EdgeInsets {
        self.border.as_ref().map_or(EdgeInsets::ZERO, |b| {
            b.resolve(&self.text_direction).dimensions()
        })
    }

    fn get_clip_path(&self, rect: Rect, text_direction: &TextDirection) -> BezPath {
        match self.shape {
            BoxShape::Circle => {
                Circle::new(rect.center(), rect.width().min(rect.height()) / 2.0).to_path(EPSILON)
            }
            BoxShape::Rectangle => {
                if let Some(border_radius) = &self.border_radius {
                    let border_radius = border_radius.resolve(text_direction);
                    let rrect = border_radius.to_rrect(&rect);
                    RoundedRect::try_from(rrect).unwrap().to_path(EPSILON)
                } else {
                    druid_shell::piet::kurbo::Rect::from(rect).to_path(EPSILON)
                }
            }
        }
    }

    fn paint(&self, canvas: &mut Canvas, rect: Rect, offset: &Offset) {
        let path = self.get_clip_path(rect, &self.text_direction);

        // draw shadows
        if (self.shape != BoxShape::Rectangle || self.border_radius.is_some())
            && !self.box_shadow.is_empty()
        {
            log::warn!("Box shadows are not supported for non-rectangular shapes (yet)");
        }
        for shadow in &self.box_shadow {
            shadow.paint(canvas, rect, offset);
        }

        // draw background color
        if let Some(color) = &self.color {
            canvas.fill(path.clone(), color);
        }
        // or draw gradient
        // if let Some(gradient) = &self.gradient {
        //     canvas.fill(rect, gradient);
        // }
        // FIXME: draw background image

        // draw border
        if let Some(border) = &self.border {
            let border = border.resolve(&self.text_direction);
            let radius = self
                .border_radius
                .as_ref()
                .map(|r| r.resolve(&self.text_direction))
                .unwrap_or(BorderRadius::ZERO);
            border.paint(canvas, rect, Some(self.shape), radius)
        }
    }
}
