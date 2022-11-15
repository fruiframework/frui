use std::{ops::Add};

use frui::prelude::*;

use crate::{borders::BorderSide, Directional, ShapeBorder, TextDirection, EdgeInsets, EPSILON};

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

    fn stroke_path(&self, rect: Rect, text_direction: Option<TextDirection>) -> BezPath {
        Into::<druid_shell::piet::kurbo::Rect>::into(self.dimensions().deflate_rect(rect)).into_path(EPSILON)
    }

    fn shape_path(&self, rect: Rect, text_direction: Option<TextDirection>) -> BezPath {
        Into::<druid_shell::piet::kurbo::Rect>::into(rect.clone()).into_path(EPSILON)
    }

    fn paint(&self, canvas: &mut PaintContext, rect: Rect, text_direction: Option<TextDirection>) {
        todo!()
    }
}

impl BoxBorder {
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
