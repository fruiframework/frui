use std::{ops::Add};

use druid_shell::piet::StrokeStyle;
use frui::prelude::*;

use crate::{borders::BorderSide, Directional, ShapeBorder, TextDirection, EdgeInsets, EPSILON, BorderRadius, BorderStyle};

#[derive(Debug, Copy, Clone, PartialEq)]
    pub enum BoxShape {
    Circle,
    Rectangle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoxBorder {
    pub top: BorderSide,
    pub right: BorderSide,
    pub bottom: BorderSide,
    pub left: BorderSide,
}

impl Directional for BoxBorder {
    type Output = BoxBorder;

    fn resolve(&self, _direction: &TextDirection) -> Self {
        self.clone()
    }
}

impl Default for BoxBorder {
    fn default() -> Self {
        Self {
            top: BorderSide::NONE,
            right: BorderSide::NONE,
            bottom: BorderSide::NONE,
            left: BorderSide::NONE,
        }
    }
}

impl Add for BoxBorder {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if BorderSide::can_merge(&self.top, &rhs.top)
            && BorderSide::can_merge(&self.right, &rhs.right)
            && BorderSide::can_merge(&self.bottom, &rhs.bottom)
            && BorderSide::can_merge(&self.left, &rhs.left)
        {
            Some(BoxBorder::merge(&self, &rhs))
        } else {
            None
        }
    }
}

impl ShapeBorder for BoxBorder {
    fn dimensions(&self) -> EdgeInsets {
        EdgeInsets::from_ltrb(self.left.width, self.top.width, self.right.width, self.bottom.width)
    }

    fn stroke_path(&self, rect: Rect) -> BezPath {
        Into::<druid_shell::piet::kurbo::Rect>::into(self.dimensions().deflate_rect(rect)).into_path(EPSILON)
    }

    fn shape_path(&self, rect: Rect) -> BezPath {
        Into::<druid_shell::piet::kurbo::Rect>::into(rect.clone()).into_path(EPSILON)
    }
}

impl BoxBorder {
    pub fn all(color: Color, width: f64, stroke_style: BorderStyle) -> Self {
        let side = BorderSide {
            color,
            width,
            style: stroke_style,
        };
        Self::from_border_side(side)
    }

    pub fn from_border_side(side: BorderSide) -> Self {
        Self {
            top: side.clone(),
            right: side.clone(),
            bottom: side.clone(),
            left: side,
        }
    }

    pub fn merge(a: &BoxBorder, b: &BoxBorder) -> Self {
        Self {
            top: BorderSide::merge(&a.top, &b.top),
            right: BorderSide::merge(&a.right, &b.right),
            bottom: BorderSide::merge(&a.bottom, &b.bottom),
            left: BorderSide::merge(&a.left, &b.left),
        }
    }

    pub fn is_uniform(&self) -> bool {
        self.color_is_uniform() && self.width_is_uniform() && self.style_is_uniform()
    }

    fn color_is_uniform(&self) -> bool {
        self.top.color == self.right.color
            && self.right.color == self.bottom.color
            && self.bottom.color == self.left.color
    }

    fn width_is_uniform(&self) -> bool {
        self.top.width == self.right.width
            && self.right.width == self.bottom.width
            && self.bottom.width == self.left.width
    }

    fn style_is_uniform(&self) -> bool {
        self.top.style == self.right.style
            && self.right.style == self.bottom.style
            && self.bottom.style == self.left.style
    }

    pub fn paint(&self, canvas: &mut PaintContext, rect: Rect, shape: Option<BoxShape>, border_radius: BorderRadius) {
        assert!(self.is_uniform(), "BoxBorder::paint() can only paint uniform borders");
        if self.top.width == 0.0 {
            return;
        }

        let path = if let Some(shape) = shape {
            match shape {
                BoxShape::Circle => {
                    let center = rect.center();
                    let radius = rect.width().min(rect.height()) / 2.0;
                    Circle::new(center, radius).into_path(EPSILON)
                }
                BoxShape::Rectangle => {
                    RoundedRect::try_from(border_radius.to_rrect(&rect)).unwrap().into_path(EPSILON)
                }
            }
        } else {
            druid_shell::piet::kurbo::Rect::from(rect).into_path(EPSILON)
        };

        let brush = canvas.solid_brush(self.top.color.clone());
        if let Some(stroke_style) = self.top.to_stroke_style() {
            canvas.stroke_styled(path, &brush, self.top.width, &stroke_style);
        } else {
            canvas.stroke(path, &brush, self.top.width);
        }
    }
}

pub struct BoxBoxderDirectional {
    pub top: BorderSide,
    pub start: BorderSide,
    pub end: BorderSide,
    pub bottom: BorderSide,
    pub shape: BoxShape,
}

impl Directional for BoxBoxderDirectional {
    type Output = BoxBorder;

    fn resolve(&self, text_direction: &TextDirection) -> Self::Output {
        match text_direction {
            TextDirection::Ltr => BoxBorder {
                top: self.top.clone(),
                right: self.end.clone(),
                bottom: self.bottom.clone(),
                left: self.start.clone(),
            },
            TextDirection::Rtl => BoxBorder {
                top: self.top.clone(),
                right: self.start.clone(),
                bottom: self.bottom.clone(),
                left: self.end.clone(),
            },
        }
    }
}
