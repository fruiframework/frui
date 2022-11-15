use std::ops::{Mul, Add};

use frui::prelude::*;

use crate::{EdgeInsets, TextDirection};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    None,
    Solid,
}

/// A side of a border of a box.
#[derive(Debug, Clone, PartialEq)]
pub struct BorderSide {
    pub color: Color,
    pub width: f64,
    pub style: BorderStyle,
}

impl BorderSide {
    pub const NONE: BorderSide = BorderSide {
        color: Color::BLACK,
        width: 0.0,
        style: BorderStyle::None,
    };

    pub fn merge(a: &BorderSide, b: &BorderSide) -> Self {
        assert!(BorderSide::can_merge(a, b));
        let a_is_none = a.style == BorderStyle::None && a.width == 0.0;
        let b_is_none = b.style == BorderStyle::None && b.width == 0.0;
        if a_is_none && b_is_none {
            BorderSide::NONE
        } else if a_is_none {
            b.clone()
        } else if b_is_none {
            a.clone()
        } else {
            BorderSide {
                color: a.color.clone(),
                width: a.width + b.width,
                style: BorderStyle::Solid,
            }
        }
    }

    pub fn can_merge(a: &BorderSide, b: &BorderSide) -> bool {
        if (a.style == BorderStyle::None && a.width == 0.0)
            || (b.style == BorderStyle::None && b.width == 0.0)
        {
            true
        } else {
            a.style == b.style && a.color == b.color
        }
    }

    pub fn copy_with(
        &self,
        color: Option<Color>,
        width: Option<f64>,
        style: Option<BorderStyle>,
    ) -> Self {
        BorderSide {
            color: color.unwrap_or(self.color.clone()),
            width: width.unwrap_or(self.width),
            style: style.unwrap_or(self.style),
        }
    }
}

impl Mul<f64> for BorderSide {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        BorderSide {
            color: self.color,
            width: (self.width * rhs).max(0.0),
            style: if rhs <= 0.0 {
                BorderStyle::None
            } else {
                self.style
            },
        }
    }
}

pub trait ShapeBorder : Add + Sized {
    fn dimensions(&self) -> EdgeInsets;

    fn stroke_path(&self, rect: Rect, text_direction: Option<TextDirection>) -> BezPath;

    fn shape_path(&self, rect: Rect, text_direction: Option<TextDirection>) -> BezPath;

    fn paint(&self, canvas: &mut PaintContext, rect: Rect, text_direction: Option<TextDirection>);
}

// pub struct ShapeBorder<E, S>
// where
//     E: Directional<Output = EdgeInsets>,
//     S: Shape,
// {
//     pub dimensions: E,
//     pub shape: S,
// }

// impl<E, S> ShapeBorder<E, S>
// where
//     E: Directional<Output = EdgeInsets>,
//     S: Shape,
// {
//     fn paint(&self, canvas: &mut PaintContext, rect: Rect, text_direction: TextDirection) {}
// }