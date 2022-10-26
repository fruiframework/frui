use std::any::Any;

pub(crate) use sealed::ParentDataOS;

pub trait ParentData {
    type Data: 'static;

    fn create_data(&self) -> Self::Data;
}

mod sealed {
    use super::*;

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
}
