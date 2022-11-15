use druid_shell::piet::kurbo::Shape;
use frui::prelude::*;

use crate::{
    border_radius::BorderRadius, box_border::BoxShape, BoxBorder, BoxShadow, Directional,
    EdgeInsets, ShapeBorder, TextDirection, EPSILON,
};

pub trait BoxPainter {
    fn paint(&self, canvas: &mut PaintContext, offset: Offset);
}

pub trait Decoration {
    fn padding(&self) -> EdgeInsets;

    fn get_clip_path(&self, rect: Rect, text_direction: &TextDirection) -> BezPath;

    fn paint(&self, canvas: &mut PaintContext, rect: Rect, offset: &Offset);
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
    // pub gradient: Option<G>,
    pub shape: BoxShape,
    pub text_direction: TextDirection,
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

    fn paint(&self, canvas: &mut PaintContext, rect: Rect, offset: &Offset) {
        let path = self.get_clip_path(rect, &self.text_direction);

        // draw shadows
        if self.shape != BoxShape::Rectangle || self.border_radius.is_some() {
            log::warn!("Box shadows are not supported for non-rectangular shapes");
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
