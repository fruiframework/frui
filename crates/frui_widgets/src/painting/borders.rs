use std::ops::{Mul, Add};

use druid_shell::{piet::StrokeStyle, kurbo::BezPath};
use frui::prelude::*;
use frui::render::Rect;

use crate::{EdgeInsets};

#[derive(Debug, Clone, PartialEq)]
pub enum BorderStyle {
    None,
    Solid,
    /// Dashed line with a gap at the start and end of the line.
    /// Follow the standard of PostScript
    /// 
    /// Example:
    /// A dash line with 5px solid and 5px gap, and 2px solid and 5px gap, start offset 0px
    /// ```rust
    /// BorderStyle::Dashed(vec![5.0, 5.0, 2.0, 5.0], 0.0)
    /// ```
    /// You can use offset to change the start position of the line and make an animation.
    Dash(Vec<f64>, f64),
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
                style: a.style.clone(),
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
            style: style.unwrap_or(self.style.clone()),
        }
    }

    pub fn to_stroke_style(&self) -> Option<StrokeStyle> {
        if let BorderStyle::Dash(ref dash, offset) = self.style {
            Some(StrokeStyle::new().dash(dash.clone(), offset))
        } else {
            None
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

    fn stroke_path(&self, rect: Rect) -> BezPath;

    fn shape_path(&self, rect: Rect) -> BezPath;
}