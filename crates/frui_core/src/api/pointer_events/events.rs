use druid_shell::{
    kurbo::{Affine, Point},
    MouseEvent,
};

#[derive(Debug, Clone)]
pub enum PointerEvent {
    PointerUp(PointerUp),
    PointerDown(PointerDown),
    PointerMove(PointerMove),
    PointerExit(PointerExit),
    PointerScroll(PointerScroll),
}

impl PointerEvent {
    pub(crate) fn new(e: &MouseEvent, arg: &str) -> PointerEvent {
        match arg {
            "up" => Self::PointerUp(PointerUp(e.clone())),
            "move" => Self::PointerMove(PointerMove(e.clone())),
            "down" => Self::PointerDown(PointerDown(e.clone())),
            "wheel" => Self::PointerScroll(PointerScroll(e.clone())),
            _ => unreachable!(),
        }
    }

    pub fn pos(&self) -> Point {
        match self {
            PointerEvent::PointerDown(e) => e.0.pos,
            PointerEvent::PointerUp(e) => e.0.pos,
            PointerEvent::PointerScroll(e) => e.0.pos,
            PointerEvent::PointerMove(e) => e.0.pos,
            PointerEvent::PointerExit(e) => e.0.pos,
        }
    }

    pub fn transform(&self, affine: &Affine) -> Self {
        let pos = *affine * self.pos();
        self.clone_at(pos)
    }

    pub fn clone_at(&self, pos: Point) -> Self {
        let mut r = self.clone();

        match &mut r {
            PointerEvent::PointerUp(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerDown(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerScroll(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerMove(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerExit(e) => {
                e.0.pos = pos;
            }
        }

        r
    }

    pub(crate) fn raw(&self) -> MouseEvent {
        match self.clone() {
            PointerEvent::PointerUp(e) => e.0,
            PointerEvent::PointerDown(e) => e.0,
            PointerEvent::PointerMove(e) => e.0,
            PointerEvent::PointerExit(e) => e.0,
            PointerEvent::PointerScroll(e) => e.0,
        }
    }
}

// Todo: Refactor following ...

#[derive(Debug, Clone)]
pub struct PointerUp(pub MouseEvent);

#[derive(Debug, Clone)]
pub struct PointerDown(pub MouseEvent);

#[derive(Debug, Clone)]
pub struct PointerScroll(pub MouseEvent);

#[derive(Debug, Clone)]
pub struct PointerEnter(pub MouseEvent);

#[derive(Debug, Clone)]
pub struct PointerMove(pub MouseEvent);

#[derive(Debug, Clone)]
pub struct PointerExit(pub MouseEvent);
