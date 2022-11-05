use std::ops::{AddAssign, Sub, SubAssign};

use druid_shell::kurbo::Point;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl From<Offset> for Size {
    fn from(value: Offset) -> Self {
        Size {
            width: value.x,
            height: value.y,
        }
    }
}

impl Size {
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

impl From<druid_shell::kurbo::Size> for Size {
    fn from(size: druid_shell::kurbo::Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
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

impl Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Constraints {
    pub fn min(&self) -> Size {
        Size {
            width: self.min_width,
            height: self.min_height,
        }
    }

    pub fn max(&self) -> Size {
        Size {
            width: self.max_width,
            height: self.max_height,
        }
    }

    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            max_width: self.max_width,
            min_height: 0.0,
            max_height: self.max_height,
        }
    }

    pub fn tighten(&self, width: Option<f64>, height: Option<f64>) -> Self {
        Self {
            min_width: width.map_or(self.min_width, |w| w.clamp(self.min_width, self.max_width)),
            max_width: width.map_or(self.max_width, |w| w.clamp(self.min_width, self.max_width)),
            min_height: height.map_or(self.min_height, |h| {
                h.clamp(self.min_height, self.max_height)
            }),
            max_height: height.map_or(self.max_height, |h| {
                h.clamp(self.min_height, self.max_height)
            }),
        }
    }

    pub fn tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    pub fn loose(size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.width,
            min_height: 0.0,
            max_height: size.height,
        }
    }

    pub fn constrain_width(&self, width: Option<f64>) -> f64 {
        width
            .unwrap_or(f64::INFINITY)
            .clamp(self.min_width, self.max_width)
    }

    pub fn constrain_height(&self, height: Option<f64>) -> f64 {
        height
            .unwrap_or(f64::INFINITY)
            .clamp(self.min_height, self.max_height)
    }

    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            self.constrain_width(Some(size.width)),
            self.constrain_height(Some(size.height)),
        )
    }

    pub fn constrain_dimensions(&self, width: f64, height: f64) -> Size {
        Size::new(
            self.constrain_width(Some(width)),
            self.constrain_height(Some(height)),
        )
    }

    pub fn biggest(&self) -> Size {
        Size::new(self.constrain_width(None), self.constrain_height(None))
    }

    pub fn smallest(&self) -> Size {
        self.constrain_dimensions(0.0, 0.0)
    }

    pub fn has_tight_width(&self) -> bool {
        self.min_width >= self.max_width
    }

    pub fn has_tight_height(&self) -> bool {
        self.min_height >= self.max_height
    }

    pub fn is_tight(&self) -> bool {
        self.has_tight_width() && self.has_tight_height()
    }

    pub fn constrains_size_with_aspect_ratio(&self, size: Size) -> Size {
        if self.is_tight() {
            self.smallest()
        } else {
            let aspect_ratio = size.aspect_ratio();
            let mut width = size.width;
            let mut height = size.height;

            if width > self.max_width {
                width = self.max_width;
                height = width / aspect_ratio;
            }

            if height > self.max_height {
                height = self.max_height;
                width = height * aspect_ratio;
            }

            if width < self.min_width {
                width = self.min_width;
                height = width / aspect_ratio;
            }

            if height < self.min_height {
                height = self.min_height;
                width = height * aspect_ratio;
            }

            self.constrain_dimensions(width, height)
        }
    }

    pub fn has_bounded_width(&self) -> bool {
        self.max_width < f64::INFINITY
    }

    pub fn has_bounded_height(&self) -> bool {
        self.max_height < f64::INFINITY
    }

    pub fn has_infinite_width(&self) -> bool {
        self.min_width >= f64::INFINITY
    }

    pub fn has_infinite_height(&self) -> bool {
        self.min_height >= f64::INFINITY
    }

    pub fn is_satisfied_by(&self, size: Size) -> bool {
        (self.min_width..=self.max_width).contains(&size.width)
            && (self.min_height..=self.max_height).contains(&size.height)
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Self {
            min_width: 0.0,
            max_width: f64::INFINITY,
            min_height: 0.0,
            max_height: f64::INFINITY,
        }
    }
}
