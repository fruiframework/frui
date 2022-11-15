use std::ops::Mul;

use druid_shell::piet::PaintBrush;
use frui::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlurStyle {
    Normal,
    Solid,
    Outer,
    Inner,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Offset,
    pub blur_radius: f64,
    pub spread_radius: f64,
    pub blur_style: BlurStyle,
}

impl Mul<f64> for BoxShadow {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        BoxShadow {
            color: self.color,
            offset: self.offset * rhs,
            blur_radius: self.blur_radius * rhs,
            spread_radius: self.spread_radius * rhs,
            blur_style: self.blur_style,
        }
    }
}