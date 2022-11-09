use std::ops::{Add, Sub};

use druid_shell::kurbo::Point;

use super::Size;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl Add for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Offset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Size> for Offset {
    fn from(size: Size) -> Self {
        Self {
            x: size.width,
            y: size.height,
        }
    }
}

impl From<Offset> for Point {
    fn from(offset: Offset) -> Self {
        Point {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl From<&Offset> for Point {
    fn from(offset: &Offset) -> Self {
        Point {
            x: offset.x,
            y: offset.y,
        }
    }
}
