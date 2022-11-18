use std::ops::Mul;

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

impl BoxShadow {
    pub fn paint(&self, canvas: &mut PaintContext, rect: Rect, _offset: &Offset) {
        assert!(self.blur_style == BlurStyle::Normal, "Shadow now only supports BlurStyle::Normal blur style");
        let brush = canvas.solid_brush(self.color.clone());
        canvas.with_save(|c| {
            let translate: Vec2 = (self.offset.x, self.offset.y).into();
            c.transform(Affine::translate(translate));
            c.blurred_rect(rect.into(), self.blur_radius, &brush);
            Ok(())
        }).unwrap();
    }
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