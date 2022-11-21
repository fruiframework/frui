use frui::{prelude::*, render::{Constraints, Size, Offset, Rect}};

use std::{
    fmt::{Display, Formatter},
    ops::{Add, Mul, Neg, Sub},
};

use crate::{Axis, Directional, TextDirection};

#[derive(Copy, Clone, Debug, Default, PartialEq, Builder)]
pub struct EdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl Directional for EdgeInsets {
    type Output = EdgeInsets;

    fn resolve(&self, _: &TextDirection) -> EdgeInsets {
        *self
    }
}

impl EdgeInsets {
    pub const ZERO: EdgeInsets = EdgeInsets {
        top: 0.0,
        left: 0.0,
        bottom: 0.0,
        right: 0.0,
    };

    pub const INFINITY: EdgeInsets = EdgeInsets {
        top: f64::INFINITY,
        left: f64::INFINITY,
        bottom: f64::INFINITY,
        right: f64::INFINITY,
    };

    pub fn deflate_constraints(&self, constraints: &Constraints) -> Constraints {
        let horizontal = self.horizontal();
        let vertical = self.vertical();
        let deflated_min_width = (constraints.min_width - horizontal).max(0.0);
        let deflated_min_height = (constraints.min_height - vertical).max(0.0);
        Constraints {
            min_width: deflated_min_width,
            max_width: deflated_min_width.max(constraints.max_width - horizontal),
            min_height: deflated_min_height,
            max_height: deflated_min_height.max(constraints.max_height - vertical),
        }
    }

    pub fn from_ltrb(left: f64, top: f64, right: f64, bottom: f64) -> EdgeInsets {
        EdgeInsets {
            top,
            left,
            bottom,
            right,
        }
    }

    pub fn all(value: f64) -> EdgeInsets {
        EdgeInsets {
            top: value,
            left: value,
            bottom: value,
            right: value,
        }
    }

    pub fn symmetric(vertical: f64, horizontal: f64) -> EdgeInsets {
        EdgeInsets {
            top: vertical,
            left: horizontal,
            bottom: vertical,
            right: horizontal,
        }
    }

    pub fn is_non_negative(&self) -> bool {
        self.top >= 0.0 && self.left >= 0.0 && self.bottom >= 0.0 && self.right >= 0.0
    }

    /// The total offset in the horizontal direction.
    pub fn horizontal(&self) -> f64 {
        self.left + self.right
    }

    /// The total offset in the vertical direction.
    pub fn vertical(&self) -> f64 {
        self.top + self.bottom
    }

    /// The total offset in the given direction.
    pub fn along(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.horizontal(),
            Axis::Vertical => self.vertical(),
        }
    }

    /// The size that this [EdgeInsets] would occupy with an empty interior.
    pub fn collapsed_size(&self) -> Size {
        Size::new(self.horizontal(), self.vertical())
    }

    pub fn flipped(&self) -> EdgeInsets {
        EdgeInsets {
            top: self.bottom,
            left: self.right,
            bottom: self.top,
            right: self.left,
        }
    }

    pub fn inflate_size(&self, size: Size) -> Size {
        Size::new(
            size.width + self.horizontal(),
            size.height + self.vertical(),
        )
    }

    pub fn deflate_size(&self, size: Size) -> Size {
        Size::new(
            size.width - self.horizontal(),
            size.height - self.vertical(),
        )
    }

    pub fn deflate_rect(&self, rect: Rect) -> Rect {
        Rect::from_ltrb(
            rect.left + self.left,
            rect.top + self.top,
            rect.right - self.right,
            rect.bottom - self.bottom,
        )
    }

    pub fn clamp(&self, min: EdgeInsets, max: EdgeInsets) -> EdgeInsets {
        EdgeInsets {
            top: self.top.clamp(min.top, max.top),
            left: self.left.clamp(min.left, max.left),
            bottom: self.bottom.clamp(min.bottom, max.bottom),
            right: self.right.clamp(min.right, max.right),
        }
    }

    pub fn lerp(&self, other: EdgeInsets, t: f64) -> EdgeInsets {
        self.clone() + (other - self.clone()) * t
    }

    /// An Offset describing the vector from the top left of a rectangle to the
    /// top left of that rectangle inset by this object.
    pub fn top_left(&self) -> Offset {
        Offset::new(self.left, self.top)
    }

    /// An Offset describing the vector from the top right of a rectangle to the
    /// top right of that rectangle inset by this object.
    pub fn top_right(&self) -> Offset {
        Offset::new(-self.right, self.top)
    }

    /// An Offset describing the vector from the bottom left of a rectangle to
    /// the bottom left of that rectangle inset by this object.
    pub fn bottom_left(&self) -> Offset {
        Offset::new(self.left, -self.bottom)
    }

    /// An Offset describing the vector from the bottom right of a rectangle to
    /// the bottom right of that rectangle inset by this object.
    pub fn bottom_right(&self) -> Offset {
        Offset::new(-self.right, -self.bottom)
    }
}

impl Into<Size> for EdgeInsets {
    fn into(self) -> Size {
        self.collapsed_size()
    }
}

impl Neg for EdgeInsets {
    type Output = EdgeInsets;

    fn neg(self) -> EdgeInsets {
        EdgeInsets {
            top: -self.top,
            left: -self.left,
            bottom: -self.bottom,
            right: -self.right,
        }
    }
}

impl Sub for EdgeInsets {
    type Output = EdgeInsets;

    fn sub(self, rhs: EdgeInsets) -> EdgeInsets {
        EdgeInsets {
            top: self.top - rhs.top,
            left: self.left - rhs.left,
            bottom: self.bottom - rhs.bottom,
            right: self.right - rhs.right,
        }
    }
}

impl Add for EdgeInsets {
    type Output = EdgeInsets;

    fn add(self, rhs: EdgeInsets) -> EdgeInsets {
        EdgeInsets {
            top: self.top + rhs.top,
            left: self.left + rhs.left,
            bottom: self.bottom + rhs.bottom,
            right: self.right + rhs.right,
        }
    }
}

impl Mul<f64> for EdgeInsets {
    type Output = EdgeInsets;

    fn mul(self, rhs: f64) -> EdgeInsets {
        EdgeInsets {
            top: self.top * rhs,
            left: self.left * rhs,
            bottom: self.bottom * rhs,
            right: self.right * rhs,
        }
    }
}

impl Display for EdgeInsets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EdgeInsets({}, {}, {}, {})",
            self.top, self.left, self.bottom, self.right
        )
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EdgeInsetsDirectional {
    pub start: f64,
    pub top: f64,
    pub end: f64,
    pub bottom: f64,
}

impl EdgeInsetsDirectional {
    pub fn from_steb(start: f64, top: f64, end: f64, bottom: f64) -> EdgeInsetsDirectional {
        EdgeInsetsDirectional {
            start,
            top,
            end,
            bottom,
        }
    }
}

impl Directional for EdgeInsetsDirectional {
    type Output = EdgeInsets;

    fn resolve(&self, text_direction: &TextDirection) -> EdgeInsets {
        match text_direction {
            TextDirection::Ltr => EdgeInsets {
                left: self.start,
                top: self.top,
                right: self.end,
                bottom: self.bottom,
            },
            TextDirection::Rtl => EdgeInsets {
                left: self.end,
                top: self.top,
                right: self.start,
                bottom: self.bottom,
            },
        }
    }
}
