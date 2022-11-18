//! Adapted from Flutter.

use super::Size;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Constraints {
    pub const ZERO: Constraints = Constraints {
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
    };

    pub fn new(min_width: f64, max_width: f64, min_height: f64, max_height: f64) -> Self {
        Self {
            min_width,
            max_width,
            min_height,
            max_height,
        }
    }

    pub fn new_loose(size: Size) -> Self {
        Self {
            min_width: 0.0,
            max_width: size.width,
            min_height: 0.0,
            max_height: size.height,
        }
    }

    pub fn new_tight(size: Size) -> Self {
        Self {
            min_width: size.width,
            max_width: size.width,
            min_height: size.height,
            max_height: size.height,
        }
    }

    pub fn new_tight_for(width: Option<f64>, height: Option<f64>) -> Self {
        Self {
            min_width: width.unwrap_or(0.0),
            max_width: width.unwrap_or(f64::INFINITY),
            min_height: height.unwrap_or(0.0),
            max_height: height.unwrap_or(f64::INFINITY),
        }
    }

    /// Returns new constraints that remove the minimum width and height
    /// requirements.
    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            max_width: self.max_width,
            min_height: 0.0,
            max_height: self.max_height,
        }
    }

    /// Returns new tight constraints with width and/or height as close to the
    /// given width and height as possible while still respecting the original
    /// constraints.
    #[rustfmt::skip]
    pub fn tighten(&self, width: Option<f64>, height: Option<f64>) -> Self {
        Self {
            min_width: width.map_or(self.min_width, |v| v.clamp(self.min_width, self.max_width)),
            max_width: width.map_or(self.max_width, |v| v.clamp(self.min_width, self.max_width)),
            min_height: height.map_or(self.min_height, |v| v.clamp(self.min_height, self.max_height)),
            max_height: height.map_or(self.max_height, |v| v.clamp(self.min_height, self.max_height)),
        }
    }

    /// Returns new constraints that respect the given constraints while being
    /// as close as possible to the original constraints.
    #[rustfmt::skip]
    pub fn enforce(&self, constraints: Constraints) -> Self {
        Self {
            min_width: self.min_width.clamp(constraints.min_width, constraints.max_width),
            max_width: self.max_width.clamp(constraints.min_width, constraints.max_width),
            min_height: self.min_height.clamp(constraints.min_height, constraints.max_height),
            max_height: self.max_height.clamp(constraints.min_height, constraints.max_height),
        }
    }

    /// The smallest size that satisfies the constraints.
    pub fn smallest(&self) -> Size {
        Size::new(self.min_width, self.min_height)
    }

    /// The biggest size that satisfies the constraints.
    pub fn biggest(&self) -> Size {
        Size::new(self.max_width, self.max_height)
    }

    /// Returns the size that both satisfies the constraints and is as close as
    /// possible to the given size.
    pub fn constrain(&self, size: Size) -> Size {
        Size {
            width: self.constrain_width(size.width),
            height: self.constrain_height(size.height),
        }
    }

    /// Returns the width that both satisfies the constraints and is as close as
    /// possible to the given width.
    pub fn constrain_width(&self, width: f64) -> f64 {
        width.clamp(self.min_width, self.max_width)
    }

    /// Returns the height that both satisfies the constraints and is as close as
    /// possible to the given height.
    pub fn constrain_height(&self, height: f64) -> f64 {
        height.clamp(self.min_height, self.max_height)
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
