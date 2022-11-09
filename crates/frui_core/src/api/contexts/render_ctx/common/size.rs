use std::ops::{Add, AddAssign, Sub, SubAssign};

use druid_shell::kurbo::Point;

#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub const ZERO: Size = Size {
        width: 0.0,
        height: 0.0,
    };

    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0. && point.y >= 0. && point.x <= self.width && point.y <= self.height
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, rhs: Self) {
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        None
    }

    fn lt(&self, other: &Self) -> bool {
        self.width < other.width || self.height < other.height
    }

    fn le(&self, other: &Self) -> bool {
        self.width <= other.width || self.height <= other.height
    }

    fn gt(&self, other: &Self) -> bool {
        self.width > other.width || self.height > other.height
    }

    fn ge(&self, other: &Self) -> bool {
        self.width >= other.width || self.height >= other.height
    }
}

impl From<Size> for druid_shell::kurbo::Size {
    fn from(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

impl From<druid_shell::kurbo::Size> for Size {
    fn from(size: druid_shell::kurbo::Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}
