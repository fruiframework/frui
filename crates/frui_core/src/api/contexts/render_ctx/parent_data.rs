use std::any::Any;

use frui_macros::sealed;

pub trait ParentData {
    type Data: 'static;

    fn create_data(&self) -> Self::Data;
}
#[sealed(crate)]
pub trait ParentDataOS {
    fn create_parent_data(&self) -> Box<dyn Any>;
}

impl<T> ParentDataOS for T {
    default fn create_parent_data(&self) -> Box<dyn Any> {
        Box::new(())
    }
}

impl<T: ParentData> ParentDataOS for T {
    fn create_parent_data(&self) -> Box<dyn Any> {
        Box::new(<T as ParentData>::create_data(&self))
    }
}
