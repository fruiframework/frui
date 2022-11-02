use druid_shell::{kurbo::Point, MouseEvent};

#[derive(Debug, Clone)]
pub enum PointerEvent {
    PointerUp(PointerUp),
    PointerMove(PointerMove),
    PointerDown(PointerDown),
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
        }
    }

    pub(crate) fn clone_at(&self, pos: Point) -> Self {
        let mut r = self.clone();

        match &mut r {
            PointerEvent::PointerUp(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerMove(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerDown(e) => {
                e.0.pos = pos;
            }
            PointerEvent::PointerScroll(e) => {
                e.0.pos = pos;
            }
        }

        r
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
