use std::any::Any;

use frui_macros::sealed;

pub trait RenderState {
    type State: 'static;

    fn create_state(&self) -> Self::State;
}

#[sealed(crate)]
pub trait RenderStateOS {
    fn create_render_state(&self) -> Box<dyn Any>;
}

impl<T> RenderStateOS for T {
    default fn create_render_state(&self) -> Box<dyn Any> {
        Box::new(())
    }
}

impl<T: RenderState> RenderStateOS for T {
    fn create_render_state(&self) -> Box<dyn Any> {
        Box::new(T::create_state(self))
    }
}
