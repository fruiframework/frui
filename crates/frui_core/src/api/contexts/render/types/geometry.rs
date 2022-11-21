use std::ops::{Add, Div, Mul, Neg, Rem, Sub, BitAnd, BitOr};

use druid_shell::kurbo::Point;

use super::{Offset, Size};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl Rect {
    pub const ZERO: Self = Self::from_ltrb(0.0, 0.0, 0.0, 0.0);

    pub const fn from_ltrb(left: f64, top: f64, right: f64, bottom: f64) -> Rect {
        Rect {
            top,
            left,
            bottom,
            right,
        }
    }

    pub fn from_ltwh(left: f64, top: f64, width: f64, height: f64) -> Rect {
        Rect {
            top,
            left,
            bottom: top + height,
            right: left + width,
        }
    }

    pub fn from_points(p0: impl Into<Offset>, p1: impl Into<Offset>) -> Rect {
        let a: Offset = p0.into();
        let b: Offset = p1.into();
        Rect::from_ltrb(a.x.min(b.x), a.y.min(b.y), a.x.max(b.x), a.y.max(b.y))
    }

    pub fn from_origin_size(origin: impl Into<Point>, size: impl Into<Size>) -> Rect {
        let origin = origin.into();
        let size = size.into();
        Rect::from_ltwh(origin.x, origin.y, size.width, size.height)
    }

    pub fn from_circle(center: impl Into<Offset>, radius: f64) -> Rect {
        let center = center.into();
        Rect::from_ltwh(
            center.x - radius,
            center.y - radius,
            radius * 2.0,
            radius * 2.0,
        )
    }

    pub fn from_center(center: impl Into<Offset>, width: f64, height: f64) -> Rect {
        let center = center.into();
        Rect::from_ltwh(
            center.x - width / 2.0,
            center.y - height / 2.0,
            width,
            height,
        )
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    pub fn center(&self) -> Offset {
        Offset::new(
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }

    pub fn has_nan(&self) -> bool {
        self.top.is_nan() || self.left.is_nan() || self.bottom.is_nan() || self.right.is_nan()
    }

    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    pub fn is_finite(&self) -> bool {
        self.top.is_finite()
            && self.left.is_finite()
            && self.bottom.is_finite()
            && self.right.is_finite()
    }

    pub fn is_infinite(&self) -> bool {
        self.top.is_infinite()
            || self.left.is_infinite()
            || self.bottom.is_infinite()
            || self.right.is_infinite()
    }

    pub fn shortest_side(&self) -> f64 {
        self.width().min(self.height())
    }

    pub fn longest_side(&self) -> f64 {
        self.width().max(self.height())
    }

    pub fn top_left(&self) -> Offset {
        Offset::new(self.left, self.top)
    }

    pub fn top_right(&self) -> Offset {
        Offset::new(self.right, self.top)
    }

    pub fn bottom_left(&self) -> Offset {
        Offset::new(self.left, self.bottom)
    }

    pub fn bottom_right(&self) -> Offset {
        Offset::new(self.right, self.bottom)
    }

    pub fn center_left(&self) -> Offset {
        Offset::new(self.left, self.top + self.height() / 2.0)
    }

    pub fn center_right(&self) -> Offset {
        Offset::new(self.right, self.top + self.height() / 2.0)
    }

    pub fn top_center(&self) -> Offset {
        Offset::new(self.left + self.width() / 2.0, self.top)
    }

    pub fn bottom_center(&self) -> Offset {
        Offset::new(self.left + self.width() / 2.0, self.bottom)
    }

    pub fn inflate(&self, delta: f64) -> Rect {
        Rect::from_ltrb(
            self.left - delta,
            self.top - delta,
            self.right + delta,
            self.bottom + delta,
        )
    }

    pub fn deflate(&self, delta: f64) -> Rect {
        self.inflate(-delta)
    }

    pub fn contains(&self, other: impl AsRef<Offset>) -> bool {
        let other = other.as_ref();
        self.left <= other.x
            && self.right >= other.x
            && self.top <= other.y
            && self.bottom >= other.y
    }

    pub fn overlaps(&self, other: impl AsRef<Rect>) -> bool {
        let other = other.as_ref();
        self.left < other.right
            && self.right > other.left
            && self.top < other.bottom
            && self.bottom > other.top
    }
}

/// Shift the rect by an offset.
impl Add<Offset> for Rect {
    type Output = Rect;

    fn add(self, rhs: Offset) -> Self::Output {
        Rect::from_ltrb(
            self.left + rhs.x,
            self.top + rhs.y,
            self.right + rhs.x,
            self.bottom + rhs.y,
        )
    }
}

/// intersection of two rects
impl BitAnd<Rect> for Rect {
    type Output = Rect;

    fn bitand(self, rhs: Rect) -> Self::Output {
        Rect::from_ltrb(
            self.left.max(rhs.left),
            self.top.max(rhs.top),
            self.right.min(rhs.right),
            self.bottom.min(rhs.bottom),
        )
    }
}

/// union of two rects (expand to contain both)
impl BitOr<Rect> for Rect {
    type Output = Rect;

    fn bitor(self, rhs: Rect) -> Self::Output {
        Rect::from_ltrb(
            self.left.min(rhs.left),
            self.top.min(rhs.top),
            self.right.max(rhs.right),
            self.bottom.max(rhs.bottom),
        )
    }
}

impl From<druid_shell::piet::kurbo::Rect> for Rect {
    fn from(rect: druid_shell::piet::kurbo::Rect) -> Self {
        Self {
            top: rect.y0,
            left: rect.x0,
            bottom: rect.y1,
            right: rect.x1,
        }
    }
}

impl From<Rect> for druid_shell::piet::kurbo::Rect {
    fn from(rect: Rect) -> Self {
        Self::new(rect.left, rect.top, rect.right, rect.bottom)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Radius {
    pub x: f64,
    pub y: f64,
}

impl Radius {
    pub const ZERO: Radius = Radius { x: 0.0, y: 0.0 };

    pub fn circular(radius: f64) -> Radius {
        Radius {
            x: radius,
            y: radius,
        }
    }

    pub fn elliptical(x: f64, y: f64) -> Radius {
        Radius { x, y }
    }
}

impl Neg for Radius {
    type Output = Radius;

    fn neg(self) -> Radius {
        Radius {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Radius {
    type Output = Radius;

    fn add(self, rhs: Radius) -> Radius {
        Radius {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Radius {
    type Output = Radius;

    fn sub(self, rhs: Radius) -> Radius {
        Radius {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Radius {
    type Output = Radius;

    fn mul(self, rhs: f64) -> Radius {
        Radius {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Radius {
    type Output = Radius;

    fn div(self, rhs: f64) -> Radius {
        Radius {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Rem<f64> for Radius {
    type Output = Radius;

    fn rem(self, rhs: f64) -> Radius {
        Radius {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

pub struct RRect {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
    pub tl_radius_x: f64,
    pub tl_radius_y: f64,
    pub tr_radius_x: f64,
    pub tr_radius_y: f64,
    pub bl_radius_x: f64,
    pub bl_radius_y: f64,
    pub br_radius_x: f64,
    pub br_radius_y: f64,
}

impl From<druid_shell::piet::kurbo::RoundedRect> for RRect {
    fn from(rrect: druid_shell::piet::kurbo::RoundedRect) -> Self {
        Self::from_rect_and_radius(rrect.rect().into(), Radius::circular(rrect.radius()))
    }
}

impl TryFrom<RRect> for druid_shell::piet::kurbo::RoundedRect {
    type Error = ();

    fn try_from(rrect: RRect) -> Result<Self, Self::Error> {
        let rect = Rect {
            top: rrect.top,
            right: rrect.right,
            bottom: rrect.bottom,
            left: rrect.left,
        }
        .into();
        let radius = Radius {
            x: rrect.tl_radius_x,
            y: rrect.tl_radius_y,
        };
        if radius.x == rrect.tr_radius_x
            && radius.x == rrect.bl_radius_x
            && radius.x == rrect.br_radius_x
            && radius.y == rrect.tr_radius_y
            && radius.y == rrect.bl_radius_y
            && radius.y == rrect.br_radius_y
        {
            Ok(Self::from_rect(rect, radius.x))
        } else {
            Err(())
        }
    }
}

impl RRect {
    pub const fn from_ltrbxy(
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        radius_x: f64,
        radius_y: f64,
    ) -> Self {
        RRect {
            top,
            right,
            bottom,
            left,
            tl_radius_x: radius_x,
            tl_radius_y: radius_y,
            tr_radius_x: radius_x,
            tr_radius_y: radius_y,
            bl_radius_x: radius_x,
            bl_radius_y: radius_y,
            br_radius_x: radius_x,
            br_radius_y: radius_y,
        }
    }

    pub fn from_ltrbr(left: f64, top: f64, right: f64, bottom: f64, radius: Radius) -> Self {
        RRect {
            top,
            right,
            bottom,
            left,
            tl_radius_x: radius.x,
            tl_radius_y: radius.y,
            tr_radius_x: radius.x,
            tr_radius_y: radius.y,
            bl_radius_x: radius.x,
            bl_radius_y: radius.y,
            br_radius_x: radius.x,
            br_radius_y: radius.y,
        }
    }

    pub fn from_rect_xy(rect: Rect, radius_x: f64, radius_y: f64) -> Self {
        RRect {
            top: rect.top,
            left: rect.left,
            right: rect.right,
            bottom: rect.bottom,
            tl_radius_x: radius_x,
            tl_radius_y: radius_y,
            tr_radius_x: radius_x,
            tr_radius_y: radius_y,
            bl_radius_x: radius_x,
            bl_radius_y: radius_y,
            br_radius_x: radius_x,
            br_radius_y: radius_y,
        }
    }

    pub fn from_rect_and_radius(rect: Rect, radius: Radius) -> Self {
        RRect {
            top: rect.top,
            left: rect.left,
            right: rect.right,
            bottom: rect.bottom,
            tl_radius_x: radius.x,
            tl_radius_y: radius.y,
            tr_radius_x: radius.x,
            tr_radius_y: radius.y,
            bl_radius_x: radius.x,
            bl_radius_y: radius.y,
            br_radius_x: radius.x,
            br_radius_y: radius.y,
        }
    }

    pub fn from_rect_and_corners(
        rect: Rect,
        top_left: Radius,
        top_right: Radius,
        bottom_left: Radius,
        bottom_right: Radius,
    ) -> Self {
        RRect {
            top: rect.top,
            left: rect.left,
            right: rect.right,
            bottom: rect.bottom,
            tl_radius_x: top_left.x,
            tl_radius_y: top_left.y,
            tr_radius_x: top_right.x,
            tr_radius_y: top_right.y,
            bl_radius_x: bottom_left.x,
            bl_radius_y: bottom_left.y,
            br_radius_x: bottom_right.x,
            br_radius_y: bottom_right.y,
        }
    }

    pub fn from_ltrb_and_corners(
        left: f64,
        top: f64,
        right: f64,
        bottom: f64,
        top_left: Radius,
        top_right: Radius,
        bottom_left: Radius,
        bottom_right: Radius,
    ) -> Self {
        RRect {
            top,
            right,
            bottom,
            left,
            tl_radius_x: top_left.x,
            tl_radius_y: top_left.y,
            tr_radius_x: top_right.x,
            tr_radius_y: top_right.y,
            bl_radius_x: bottom_left.x,
            bl_radius_y: bottom_left.y,
            br_radius_x: bottom_right.x,
            br_radius_y: bottom_right.y,
        }
    }

    pub fn inflate(&self, delta: f64) -> Self {
        RRect {
            top: self.top - delta,
            right: self.right + delta,
            bottom: self.bottom + delta,
            left: self.left - delta,
            tl_radius_x: self.tl_radius_x + delta,
            tl_radius_y: self.tl_radius_y + delta,
            tr_radius_x: self.tr_radius_x + delta,
            tr_radius_y: self.tr_radius_y + delta,
            bl_radius_x: self.bl_radius_x + delta,
            bl_radius_y: self.bl_radius_y + delta,
            br_radius_x: self.br_radius_x + delta,
            br_radius_y: self.br_radius_y + delta,
        }
    }

    pub fn deflate(&self, delta: f64) -> Self {
        self.inflate(-delta)
    }

    pub fn top_left_radius(&self) -> Radius {
        Radius::elliptical(self.tl_radius_x, self.tl_radius_y)
    }

    pub fn top_right_radius(&self) -> Radius {
        Radius::elliptical(self.tr_radius_x, self.tr_radius_y)
    }

    pub fn bottom_left_radius(&self) -> Radius {
        Radius::elliptical(self.bl_radius_x, self.bl_radius_y)
    }

    pub fn bottom_right_radius(&self) -> Radius {
        Radius::elliptical(self.br_radius_x, self.br_radius_y)
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }

    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    pub fn outer_rect(&self) -> Rect {
        Rect::from_ltrb(self.left, self.top, self.right, self.bottom)
    }

    // TODO: inner_rect, middle_rect, wide_middle_rect, tall_middle_rect

    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    pub fn is_finite(&self) -> bool {
        self.left.is_finite()
            && self.top.is_finite()
            && self.right.is_finite()
            && self.bottom.is_finite()
    }

    pub fn is_rect(&self) -> bool {
        (self.tl_radius_x == 0.0 || self.tl_radius_y == 0.0)
            && (self.tr_radius_x == 0.0 || self.tr_radius_y == 0.0)
            && (self.bl_radius_x == 0.0 || self.bl_radius_y == 0.0)
            && (self.br_radius_x == 0.0 || self.br_radius_y == 0.0)
    }

    /// Whether this rounded rectangle has a side with no straight section.
    pub fn is_stadium(&self) -> bool {
        self.top_left_radius() == self.top_right_radius()
            && self.top_right_radius() == self.bottom_right_radius()
            && self.bottom_right_radius() == self.bottom_left_radius()
            && (self.width() <= 2.0 * self.tl_radius_x || self.height() <= self.tl_radius_y)
    }

    pub fn is_ellipse(&self) -> bool {
        self.top_left_radius() == self.top_right_radius()
            && self.top_right_radius() == self.bottom_right_radius()
            && self.bottom_right_radius() == self.bottom_left_radius()
            && self.width() <= 2.0 * self.tl_radius_x
            && self.height() <= 2.0 * self.tl_radius_y
    }

    pub fn is_circle(&self) -> bool {
        self.is_ellipse() && self.width() == self.height()
    }

    pub fn shortest_side(&self) -> f64 {
        self.width().abs().min(self.height().abs())
    }

    pub fn longest_side(&self) -> f64 {
        self.width().abs().max(self.height().abs())
    }

    pub fn has_nan(&self) -> bool {
        self.left.is_nan()
            || self.top.is_nan()
            || self.right.is_nan()
            || self.bottom.is_nan()
            || self.tl_radius_x.is_nan()
            || self.tl_radius_y.is_nan()
            || self.tr_radius_x.is_nan()
            || self.tr_radius_y.is_nan()
            || self.bl_radius_x.is_nan()
            || self.bl_radius_y.is_nan()
            || self.br_radius_x.is_nan()
            || self.br_radius_y.is_nan()
    }

    pub fn center(&self) -> Offset {
        Offset::new(
            (self.left + self.right) / 2.0,
            (self.top + self.bottom) / 2.0,
        )
    }

    pub fn contains(&self, point: Offset) -> bool {
        let x = point.x;
        let y = point.y;
        let left = self.left;
        let top = self.top;
        let right = self.right;
        let bottom = self.bottom;
        let tl_radius_x = self.tl_radius_x;
        let tl_radius_y = self.tl_radius_y;
        let tr_radius_x = self.tr_radius_x;
        let tr_radius_y = self.tr_radius_y;
        let bl_radius_x = self.bl_radius_x;
        let bl_radius_y = self.bl_radius_y;
        let br_radius_x = self.br_radius_x;
        let br_radius_y = self.br_radius_y;

        if x < left || x > right || y < top || y > bottom {
            return false;
        }

        if tl_radius_x > 0.0 || tl_radius_y > 0.0 {
            let dx = x - left;
            let dy = y - top;
            if dx * dx / (tl_radius_x * tl_radius_x) + dy * dy / (tl_radius_y * tl_radius_y) > 1.0 {
                return false;
            }
        }

        if tr_radius_x > 0.0 || tr_radius_y > 0.0 {
            let dx = x - right;
            let dy = y - top;
            if dx * dx / (tr_radius_x * tr_radius_x) + dy * dy / (tr_radius_y * tr_radius_y) > 1.0 {
                return false;
            }
        }

        if bl_radius_x > 0.0 || bl_radius_y > 0.0 {
            let dx = x - left;
            let dy = y - bottom;
            if dx * dx / (bl_radius_x * bl_radius_x) + dy * dy / (bl_radius_y * bl_radius_y) > 1.0 {
                return false;
            }
        }

        if br_radius_x > 0.0 || br_radius_y > 0.0 {
            let dx = x - right;
            let dy = y - bottom;
            if dx * dx / (br_radius_x * br_radius_x) + dy * dy / (br_radius_y * br_radius_y) > 1.0 {
                return false;
            }
        }

        true
    }
}

impl Add<Offset> for RRect {
    type Output = RRect;

    fn add(self, rhs: Offset) -> RRect {
        RRect {
            top: self.top + rhs.y,
            right: self.right + rhs.x,
            bottom: self.bottom + rhs.y,
            left: self.left + rhs.x,
            tl_radius_x: self.tl_radius_x,
            tl_radius_y: self.tl_radius_y,
            tr_radius_x: self.tr_radius_x,
            tr_radius_y: self.tr_radius_y,
            bl_radius_x: self.bl_radius_x,
            bl_radius_y: self.bl_radius_y,
            br_radius_x: self.br_radius_x,
            br_radius_y: self.br_radius_y,
        }
    }
}

impl Add<f64> for RRect {
    type Output = RRect;

    fn add(self, rhs: f64) -> RRect {
        RRect {
            top: self.top + rhs,
            right: self.right + rhs,
            bottom: self.bottom + rhs,
            left: self.left + rhs,
            tl_radius_x: self.tl_radius_x,
            tl_radius_y: self.tl_radius_y,
            tr_radius_x: self.tr_radius_x,
            tr_radius_y: self.tr_radius_y,
            bl_radius_x: self.bl_radius_x,
            bl_radius_y: self.bl_radius_y,
            br_radius_x: self.br_radius_x,
            br_radius_y: self.br_radius_y,
        }
    }
}

impl Sub<f64> for RRect {
    type Output = RRect;

    fn sub(self, rhs: f64) -> RRect {
        self.add(-rhs)
    }
}
