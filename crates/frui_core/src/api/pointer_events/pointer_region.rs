use crate::{api::contexts::render_ctx::ext::RenderExt, prelude::*};

use super::{events::*, HitTest, HitTestCtx};

#[derive(ViewWidget)]
pub struct PointerRegion<PEN, PMV, PEX, CHILD>
where
    PEN: FnPointerEnter,
    PMV: FnPointerMove,
    PEX: FnPointerExit,
    CHILD: Widget,
{
    on_enter: PEN,
    on_move: PMV,
    on_exit: PEX,
    child: CHILD,
}

impl<PEN, PMV, PEX, CHILD> ViewWidget for PointerRegion<PEN, PMV, PEX, CHILD>
where
    PEN: FnPointerEnter,
    PMV: FnPointerMove,
    PEX: FnPointerExit,
    CHILD: Widget,
{
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }
}

impl<PEN, PMV, PEX, CHILD> RenderState for PointerRegion<PEN, PMV, PEX, CHILD>
where
    PEN: FnPointerEnter,
    PMV: FnPointerMove,
    PEX: FnPointerExit,
    CHILD: Widget,
{
    // Is hovered.
    type State = bool;

    fn create_state(&self) -> Self::State {
        false
    }
}

impl<PEN, PMV, PEX, CHILD> HitTest for PointerRegion<PEN, PMV, PEX, CHILD>
where
    PEN: FnPointerEnter,
    PMV: FnPointerMove,
    PEX: FnPointerExit,
    CHILD: Widget,
{
    fn handle_event(&self, ctx: &mut HitTestCtx<Self>, event: &PointerEvent) {
        match event {
            PointerEvent::PointerMove(e) => {
                if *ctx.render_state() {
                    self.on_move.call(&PointerMove(e.0.clone()));
                } else {
                    // Pointer now hovers over this widget.
                    *ctx.render_state_mut() = true;

                    self.on_enter.call(&PointerEnter(e.0.clone()));
                }
            }
            PointerEvent::PointerExit(e) => {
                // Pointer no longer hovers over this widget.
                *ctx.render_state_mut() = false;

                self.on_exit.call(e);
            }
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

impl_fn!(Fn(&PointerEnter) for NOP with FnPointerEnter);
impl_fn!(Fn(&PointerMove) for NOP with FnPointerMove);
impl_fn!(Fn(&PointerExit) for NOP with FnPointerExit);

impl PointerRegion<NOP, NOP, NOP, ()> {
    pub fn builder() -> Self {
        Self {
            on_enter: NOP,
            on_move: NOP,
            on_exit: NOP,
            child: (),
        }
    }
}

impl<PEN, PMV, PEX, CHILD> PointerRegion<PEN, PMV, PEX, CHILD>
where
    PEN: FnPointerEnter,
    PMV: FnPointerMove,
    PEX: FnPointerExit,
    CHILD: Widget,
{
    pub fn on_enter(
        self,
        f: impl Fn(&PointerEnter),
    ) -> PointerRegion<impl Fn(&PointerEnter), PMV, PEX, CHILD> {
        PointerRegion {
            on_enter: f,
            on_move: self.on_move,
            on_exit: self.on_exit,
            child: self.child,
        }
    }

    pub fn on_move(
        self,
        f: impl Fn(&PointerMove),
    ) -> PointerRegion<PEN, impl Fn(&PointerMove), PEX, CHILD> {
        PointerRegion {
            on_enter: self.on_enter,
            on_move: f,
            on_exit: self.on_exit,
            child: self.child,
        }
    }

    pub fn on_exit(
        self,
        f: impl Fn(&PointerExit),
    ) -> PointerRegion<PEN, PMV, impl Fn(&PointerExit), CHILD> {
        PointerRegion {
            on_enter: self.on_enter,
            on_move: self.on_move,
            on_exit: f,
            child: self.child,
        }
    }

    pub fn child(self, child: impl Widget) -> PointerRegion<PEN, PMV, PEX, impl Widget> {
        PointerRegion {
            on_enter: self.on_enter,
            on_move: self.on_move,
            on_exit: self.on_exit,
            child,
        }
    }
}
