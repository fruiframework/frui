use frui::prelude::{Offset, Size};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::TextDirection;

pub enum AlignmentGeometry {
    Alignment(Alignment),
    Directional(AlignmentDirectional),
}

impl AlignmentGeometry {
    pub fn resolve(&self, text_direction: &TextDirection) -> Alignment {
        match self {
            AlignmentGeometry::Alignment(a) => *a,
            AlignmentGeometry::Directional(a) => match text_direction {
                TextDirection::Ltr => Alignment { x: a.start, y: a.y },
                TextDirection::Rtl => Alignment {
                    x: -a.start,
                    y: a.y,
                },
            },
        }
    }
}
#[derive(Copy, Clone, Debug, Default)]
pub struct Alignment {
    x: f64,
    y: f64,
}

impl Alignment {
    pub fn along<T: Into<Size>>(&self, other: T) -> Offset {
        let size: Size = other.into();
        let center_x = size.width / 2.0;
        let center_y = size.height / 2.0;
        Offset {
            x: center_x + self.x * center_x,
            y: center_y + self.y * center_y,
        }
    }

    pub const TOP_LEFT: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: -1.0, y: -1.0 });
    pub const TOP_CENTER: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: 0.0, y: -1.0 });
    pub const TOP_RIGHT: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: 1.0, y: -1.0 });
    pub const CENTER_LEFT: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: -1.0, y: 0.0 });
    pub const CENTER: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: 0.0, y: 0.0 });
    pub const CENTER_RIGHT: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: 1.0, y: 0.0 });
    pub const BOTTOM_LEFT: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: -1.0, y: 1.0 });
    pub const BOTTOM_CENTER: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: 0.0, y: 1.0 });
    pub const BOTTOM_RIGHT: AlignmentGeometry =
        AlignmentGeometry::Alignment(Alignment { x: 1.0, y: 1.0 });

    const PRELUDES: [(&AlignmentGeometry, &'static str); 9] = [
        (&Alignment::TOP_LEFT, "Alignment::TOP_LEFT"),
        (&Alignment::TOP_CENTER, "Alignment::TOP_CENTER"),
        (&Alignment::TOP_RIGHT, "Alignment::TOP_RIGHT"),
        (&Alignment::CENTER_LEFT, "Alignment::CENTER_LEFT"),
        (&Alignment::CENTER, "Alignment::CENTER"),
        (&Alignment::CENTER_RIGHT, "Alignment::CENTER_RIGHT"),
        (&Alignment::BOTTOM_LEFT, "Alignment::BOTTOM_LEFT"),
        (&Alignment::BOTTOM_CENTER, "Alignment::BOTTOM_CENTER"),
        (&Alignment::BOTTOM_RIGHT, "Alignment::BOTTOM_RIGHT"),
    ];
}

impl Add for Alignment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Alignment {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Alignment {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Alignment {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Alignment {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Neg for Alignment {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Alignment {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for prelude in Alignment::PRELUDES {
            if let (AlignmentGeometry::Alignment(alignment), name) = prelude {
                if alignment == self {
                    return write!(f, "{}", name);
                }
            }
        }
        write!(f, "Alignment({}, {})", &self.x, &self.y)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AlignmentDirectional {
    start: f64,
    y: f64,
}

impl PartialEq for AlignmentDirectional {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.y == other.y
    }
}

impl AlignmentDirectional {
    const fn new(start: f64, y: f64) -> Self {
        Self { start, y }
    }

    pub const TOP_START: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(-1., -1.));
    pub const TOP_CENTER: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(0., -1.));
    pub const TOP_END: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(1., -1.));
    pub const CENTER_START: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(-1., 0.));
    pub const CENTER: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(0., 0.));
    pub const CENTER_END: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(1., 0.));
    pub const BOTTOM_START: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(-1., 1.));
    pub const BOTTOM_CENTER: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(0., 1.));
    pub const BOTTOM_END: AlignmentGeometry = AlignmentGeometry::Directional(Self::new(1., 1.));

    const PRELUDES: [(&AlignmentGeometry, &'static str); 9] = [
        (
            &AlignmentDirectional::TOP_START,
            "AlignmentDirectional::TOP_START",
        ),
        (
            &AlignmentDirectional::TOP_CENTER,
            "AlignmentDirectional::TOP_CENTER",
        ),
        (
            &AlignmentDirectional::TOP_END,
            "AlignmentDirectional::TOP_END",
        ),
        (
            &AlignmentDirectional::CENTER_START,
            "AlignmentDirectional::CENTER_START",
        ),
        (
            &AlignmentDirectional::CENTER,
            "AlignmentDirectional::CENTER",
        ),
        (
            &AlignmentDirectional::CENTER_END,
            "AlignmentDirectional::CENTER_END",
        ),
        (
            &AlignmentDirectional::BOTTOM_START,
            "AlignmentDirectional::BOTTOM_START",
        ),
        (
            &AlignmentDirectional::BOTTOM_CENTER,
            "AlignmentDirectional::BOTTOM_CENTER",
        ),
        (
            &AlignmentDirectional::BOTTOM_END,
            "AlignmentDirectional::BOTTOM_END",
        ),
    ];
}

impl Add for AlignmentDirectional {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AlignmentDirectional {
            start: self.start + rhs.start,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for AlignmentDirectional {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        AlignmentDirectional {
            start: self.start - rhs.start,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for AlignmentDirectional {
    type Output = Self;

    fn neg(self) -> Self::Output {
        AlignmentDirectional {
            start: -self.start,
            y: -self.y,
        }
    }
}

impl Mul<f64> for AlignmentDirectional {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        AlignmentDirectional::new(self.start * rhs, self.y * rhs)
    }
}

impl Div<f64> for AlignmentDirectional {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        AlignmentDirectional::new(self.start / rhs, self.y / rhs)
    }
}

impl Display for AlignmentDirectional {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for prelude in AlignmentDirectional::PRELUDES {
            if let (AlignmentGeometry::Directional(alignment), name) = prelude {
                if alignment == self {
                    return write!(f, "{}", name);
                }
            }
        }
        write!(f, "AlignmentDirectional({}, {})", &self.start, &self.y)
    }
}
