use frui::render::{Radius, RRect, Rect};

use crate::{Directional, TextDirection};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BorderRadius {
    pub top_left: Radius,
    pub top_right: Radius,
    pub bottom_left: Radius,
    pub bottom_right: Radius,
}

impl BorderRadius {
    pub const ZERO: BorderRadius = BorderRadius::all(Radius::ZERO);

    pub const fn all(radius: Radius) -> BorderRadius {
        BorderRadius {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    pub fn circular(radius: f64) -> BorderRadius {
        BorderRadius::all(Radius::circular(radius))
    }

    pub fn only(top_left: Radius, top_right: Radius, bottom_left: Radius, bottom_right: Radius) -> BorderRadius {
        BorderRadius {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn vertical(top: Radius, bottom: Radius) -> BorderRadius {
        BorderRadius {
            top_left: top,
            top_right: top,
            bottom_left: bottom,
            bottom_right: bottom,
        }
    }

    pub fn horizontal(left: Radius, right: Radius) -> BorderRadius {
        BorderRadius {
            top_left: left,
            top_right: right,
            bottom_left: left,
            bottom_right: right,
        }
    }

    pub fn is_uniform(&self) -> bool {
        self.top_left == self.top_right && self.top_left == self.bottom_left && self.top_left == self.bottom_right
    }

    pub fn to_rrect(&self, rect: &Rect) -> RRect {
        RRect::from_rect_and_corners(
            rect.clone(),
            self.top_left,
            self.top_right,
            self.bottom_right,
            self.bottom_left,
        )
    }
}

impl Directional for BorderRadius {
    type Output = BorderRadius;

    fn resolve(&self, _text_direction: &TextDirection) -> BorderRadius {
        *self
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BorderRadiusDirectional {
    pub top_start: Radius,
    pub top_end: Radius,
    pub bottom_start: Radius,
    pub bottom_end: Radius,
}

impl Directional for BorderRadiusDirectional {
    type Output = BorderRadius;

    fn resolve(&self, text_direction: &TextDirection) -> BorderRadius {
        match text_direction {
            TextDirection::Ltr => BorderRadius {
                top_left: self.top_start,
                top_right: self.top_end,
                bottom_left: self.bottom_start,
                bottom_right: self.bottom_end,
            },
            TextDirection::Rtl => BorderRadius {
                top_left: self.top_end,
                top_right: self.top_start,
                bottom_left: self.bottom_end,
                bottom_right: self.bottom_start,
            },
        }
    }
}


