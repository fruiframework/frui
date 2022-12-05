use crate::prelude::*;

use super::{
    events::{PointerDown, PointerEvent, PointerScroll, PointerUp},
    HitTest, HitTestCx,
};

#[derive(ViewWidget)]
pub struct PointerListener<PU, PD, PS, CHILD>
where
    PU: FnPointerUp,
    PD: FnPointerDown,
    PS: FnPointerScroll,
    CHILD: Widget,
{
    // Todo: Consider implementing `on_pointer_move` which would be triggered on
    // `PointerMove` events while the pointer is down. That event would be
    // dispatched outside of widget layout boundary, as opposed to `PointerRegion`.
    on_pointer_up: PU,
    on_pointer_down: PD,
    on_pointer_scroll: PS,
    child: CHILD,
}

impl<PU, PD, PS, CHILD> ViewWidget for PointerListener<PU, PD, PS, CHILD>
where
    PU: FnPointerUp,
    PD: FnPointerDown,
    PS: FnPointerScroll,
    CHILD: Widget,
{
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }
}

impl<PU, PD, PS, CHILD> HitTest for PointerListener<PU, PD, PS, CHILD>
where
    PU: FnPointerUp,
    PD: FnPointerDown,
    PS: FnPointerScroll,
    CHILD: Widget,
{
    fn hit_test<'a>(
        &'a self,
        cx: &'a mut HitTestCx<Self>,
        point: druid_shell::kurbo::Point,
    ) -> bool {
        if cx.layout_box().contains(point) {
            for mut child in cx.children() {
                child.hit_test(point);
            }

            return true;
        }

        false
    }

    fn handle_event(&self, _: &mut HitTestCx<Self>, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(e) => self.on_pointer_down.call(e),
            PointerEvent::PointerUp(e) => self.on_pointer_up.call(e),
            PointerEvent::PointerScroll(e) => self.on_pointer_scroll.call(e),
            _ => {}
        }
    }
}

//
// Builder methods:

/// No-op function
#[doc(hidden)]
pub struct NOP;

macro_rules! impl_fn {
    (Fn($($arg:tt)*) for $target:ident with $temp_trait:ident) => {
        pub trait $temp_trait {
            fn call(&self, _: $($arg)*) {}
        }

        impl $temp_trait for $target {
            fn call(&self, _: $($arg)*) {}
        }

        impl<F: Fn($($arg)*)> $temp_trait for F {
            fn call(&self, v: $($arg)*) {
                self(v)
            }
        }
    };
}

impl_fn!(Fn(&PointerUp) for NOP with FnPointerUp);
impl_fn!(Fn(&PointerDown) for NOP with FnPointerDown);
impl_fn!(Fn(&PointerScroll) for NOP with FnPointerScroll);

impl PointerListener<NOP, NOP, NOP, ()> {
    pub fn builder() -> Self {
        Self {
            on_pointer_up: NOP,
            on_pointer_down: NOP,
            on_pointer_scroll: NOP,
            child: (),
        }
    }
}

impl<PU, PD, PS, CHILD> PointerListener<PU, PD, PS, CHILD>
where
    PU: FnPointerUp,
    PD: FnPointerDown,
    PS: FnPointerScroll,
    CHILD: Widget,
{
    pub fn on_pointer_up(
        self,
        f: impl Fn(&PointerUp),
    ) -> PointerListener<impl Fn(&PointerUp), PD, PS, CHILD> {
        PointerListener {
            on_pointer_up: f,
            on_pointer_down: self.on_pointer_down,
            on_pointer_scroll: self.on_pointer_scroll,
            child: self.child,
        }
    }

    pub fn on_pointer_down(
        self,
        f: impl Fn(&PointerDown),
    ) -> PointerListener<PU, impl Fn(&PointerDown), PS, CHILD> {
        PointerListener {
            on_pointer_up: self.on_pointer_up,
            on_pointer_down: f,
            on_pointer_scroll: self.on_pointer_scroll,
            child: self.child,
        }
    }

    pub fn on_pointer_scroll(
        self,
        f: impl Fn(&PointerScroll),
    ) -> PointerListener<PU, PD, impl Fn(&PointerScroll), CHILD> {
        PointerListener {
            on_pointer_up: self.on_pointer_up,
            on_pointer_down: self.on_pointer_down,
            on_pointer_scroll: f,
            child: self.child,
        }
    }

    pub fn child(self, child: impl Widget) -> PointerListener<PU, PD, PS, impl Widget> {
        PointerListener {
            on_pointer_up: self.on_pointer_up,
            on_pointer_down: self.on_pointer_down,
            on_pointer_scroll: self.on_pointer_scroll,
            child,
        }
    }
}
