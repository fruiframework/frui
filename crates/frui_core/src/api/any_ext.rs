use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

/// This trait allows us to acquire `TypeId` of any `T` (not just `T: 'static`),
/// which is used to downcast trait objects containing non-static fields to a
/// concrete type.
pub trait AnyExt: AsAny {
    fn type_id(&self) -> TypeId;

    fn type_name(&self) -> &'static str;

    /// Helper function.
    fn as_any_ext(&self) -> &dyn AnyExt;
}

impl<T> AnyExt for T {
    fn type_id(&self) -> TypeId {
        get_type_id::<T>()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn as_any_ext(&self) -> &dyn AnyExt {
        self
    }
}

impl<'a> dyn AnyExt + 'a {
    /// Downcasts reference `self` to `T` or returns `None`.
    ///
    /// # Safety
    ///
    /// Downcasted `&T` may contain references of lifetimes that are
    /// different between two structures even if `TypeId`s match.
    ///
    /// One must ensure that this cannot cause UB.
    ///
    /// # Example
    ///
    /// Using internal mutabilty one can swap `'a` and `'static` references
    /// causing dangling references and use-after-free.
    ///
    /// ```
    /// struct Test<'a> {
    ///     r: RefCell<&'a str>,
    /// }
    ///
    /// impl<'a> Test<'a> {
    ///     fn swap(&'a self, other: &'a Test<'a>) {
    ///         *self.r.borrow_mut() = *other.r.borrow();
    ///     }
    /// }
    ///
    /// let string = String::from("non 'static");
    ///
    /// let static_ = Test {
    ///     r: RefCell::new("'static str"),
    /// };
    /// let non_static = Test {
    ///     r: RefCell::new(&string),
    /// };
    ///
    /// let static_any: &dyn AnyExt = &static_;
    /// let non_static_any: &dyn AnyExt = &non_static;
    ///
    /// fn uh_oh(static_: &dyn AnyExt, non_static: &dyn AnyExt) {
    ///     unsafe {
    ///         let static_ = static_.downcast_ref::<Test>().unwrap();
    ///         let non_static = non_static.downcast_ref::<Test>().unwrap();
    ///
    ///         static_.swap(non_static);
    ///     }
    /// }
    ///
    /// uh_oh(static_any, non_static_any);
    ///
    /// drop(string);
    /// println!("{}", static_.r.borrow()); // uh-oh
    /// ```
    pub unsafe fn downcast_ref<T>(&self) -> Option<&T> {
        match AnyExt::type_id(self) == get_type_id::<T>() {
            true => Some(&*(self as *const _ as *const T)),
            false => None,
        }
    }

    /// # Safety
    ///
    /// See `downcast_ref`.
    pub unsafe fn downcast_mut<T>(&mut self) -> Option<&mut T> {
        match AnyExt::type_id(self) == get_type_id::<T>() {
            true => Some(&mut *(self as *mut _ as *mut T)),
            false => None,
        }
    }
}

struct TypeIdKey<T>(PhantomData<T>);

impl<T> TypeIdKey<T> {
    fn new() -> Self {
        TypeIdKey(PhantomData)
    }
}

fn get_type_id<T>() -> TypeId {
    unsafe {
        let key = <TypeIdKey<T>>::new();

        // Safety: We cast &key to 'static to be able to cast it to `Any` to acquire TypeId.
        // This is because `TypeId::of::<TypeIdKey<T>>()` won't work since T isn't 'static.
        //
        // That `&'static key` reference is not used any longer than it would normally be.
        let any = std::mem::transmute::<&dyn AsAny, &'static dyn AsAny>(&key);
        let any = any.as_any();
        Any::type_id(any)
    }
}

/// Helper trait used in [`get_type_id`] above.
pub trait AsAny {
    fn as_any(&'static self) -> &dyn Any;
}

impl<T> AsAny for T {
    fn as_any(&'static self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_downcast() {
        unsafe {
            assert!((&16usize as &dyn AnyExt).downcast_ref::<usize>().is_some());
            assert!((&String::new() as &dyn AnyExt)
                .downcast_ref::<String>()
                .is_some());
            assert!((&std::sync::Mutex::new(2u8) as &dyn AnyExt)
                .downcast_ref::<std::sync::Mutex<u8>>()
                .is_some());
        }
    }

    #[test]
    fn should_not_downcast() {
        unsafe {
            assert!((&16usize as &dyn AnyExt).downcast_ref::<u8>().is_none());
            assert!((&std::sync::Mutex::new(2u8) as &dyn AnyExt)
                .downcast_ref::<std::sync::Mutex<usize>>()
                .is_none());
        }
    }
}
