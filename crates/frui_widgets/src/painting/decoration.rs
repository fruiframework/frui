use druid_shell::piet::kurbo::Shape;
use frui::prelude::*;

use crate::{border_radius::BorderRadius, Directional, EdgeInsets, TextDirection, box_border::BoxShape};

pub trait BoxPainter {
    fn paint(&self, canvas: &mut PaintContext, offset: Offset);
}

pub trait Decoration {
    fn padding(&self) -> EdgeInsets;

    fn get_clip_path(&self, rect: Rect, text_direction: &TextDirection) -> BezPath;
}



pub enum DecorationPosition {
    Background,
    Foreground,
}

pub struct BoxDecoration<B, BR, G>
where
    B: Directional<Output = EdgeInsets>,
    BR: Directional<Output = BorderRadius>,
    G: Into<druid_shell::piet::Brush>,
{
    pub color: Option<Color>,
    // TODO: image background
    // pub image: Option<Image>,
    pub border: Option<B>,
    pub border_radius: Option<BR>,
    pub gradient: Option<G>,
    pub shape: BoxShape,
    pub text_direction: TextDirection,
}

impl<B, BR, G> Decoration for BoxDecoration<B, BR, G>
where
    B: Directional<Output = EdgeInsets>,
    BR: Directional<Output = BorderRadius>,
    G: Into<druid_shell::piet::Brush>,
{
    fn padding(&self) -> EdgeInsets {
        self.border.as_ref().map(|b| b.resolve(&self.text_direction)).unwrap_or(EdgeInsets::ZERO)
    }

    fn get_clip_path(&self, rect: Rect, text_direction: &TextDirection) -> BezPath {
        match self.shape {
            BoxShape::Circle => {
                Circle::new(rect.center(), rect.width().min(rect.height()) / 2.0).to_path(0.1)
            }
            BoxShape::Rectangle => {
                if let Some(border_radius) = &self.border_radius {
                    let border_radius = border_radius.resolve(text_direction);
                    assert!(border_radius.is_uniform());
                    let rrect = border_radius.to_rrect(&rect);
                    RoundedRect::try_from(rrect).unwrap().to_path(0.1)
                } else {
                    druid_shell::piet::kurbo::Rect::from(rect).to_path(0.1)
                }
            }
        }
    }
}
