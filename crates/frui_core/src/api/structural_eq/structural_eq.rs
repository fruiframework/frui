use std::{
    cell::{Cell, RefCell, UnsafeCell},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

/// ## Warning
///
/// This is internal trait and shouldn't be implemented manually.
///
/// ## About
///
/// This trait is implemented for every type and allows to test if widget
/// configuration can be reused to optimize widget tree rebuilds.
///
/// To implement it, use `#[derive(WidgetKind)]`.
pub unsafe trait StructuralEq {
    /// This constant allows to annotate whether given strcture can be compared.
    /// It's used to avoid recursive comparisons of the whole widget subtree (if
    /// one of the fields is itself a widget).
    const EQ_ENABLED: bool;

    fn eq(&self, other: &Self) -> bool;
}

unsafe impl<T> StructuralEq for T {
    default const EQ_ENABLED: bool = false;

    default fn eq(&self, _: &Self) -> bool {
        false
    }
}

/// ## Warning
///
/// This is internal trait and shouldn't be implemented manually.
///
/// ## About
///
/// This trait makes it possible to implement [`StructuralEq`] in the downstream
/// creates without requiring `min_specialization` feature.
///
/// It can be automatically implemented using `#[derive(WidgetKind)]` macro.
pub unsafe trait StructuralEqImpl {
    /// See [`StructuralEq::EQ_ENABLED`].
    const EQ_ENABLED: bool;

    fn eq(&self, other: &Self) -> bool;
}

unsafe impl<T: StructuralEqImpl> StructuralEq for T {
    const EQ_ENABLED: bool = <T as StructuralEqImpl>::EQ_ENABLED;

    fn eq(&self, other: &Self) -> bool {
        <T as StructuralEqImpl>::eq(self, other)
    }
}

pub auto trait Immutable {}

impl<T> !Immutable for UnsafeCell<T> {}
impl<T: Immutable> Immutable for Rc<T> {}
impl<T: Immutable> Immutable for Arc<T> {}

unsafe impl<T: StructuralEq + ?Sized> StructuralEqImpl for &T {
    default const EQ_ENABLED: bool = T::EQ_ENABLED;

    default fn eq(&self, other: &Self) -> bool {
        // Only if pointers to T are equal, we can compare the inner-contents.
        // Otherwise this could cause use-after-free after we deallocated one of
        // parent widgets.
        if *self as *const _ == *other as *const _ {
            T::eq(self.deref(), other.deref()) // deref to T
        } else {
            false
        }
    }
}

unsafe impl<T: StructuralEq + ?Sized> StructuralEqImpl for &mut T {
    default const EQ_ENABLED: bool = T::EQ_ENABLED;

    default fn eq(&self, other: &Self) -> bool {
        // Only if pointers to T are equal, we can compare the inner-contents.
        // Otherwise this could cause use-after-free after we deallocated one of
        // parent widgets.
        if *self as *const _ == *other as *const _ {
            T::eq(self.deref(), other.deref()) // deref to T
        } else {
            false
        }
    }
}

unsafe impl<T: StructuralEq + ?Sized> StructuralEqImpl for Box<T> {
    default const EQ_ENABLED: bool = T::EQ_ENABLED;

    default fn eq(&self, other: &Self) -> bool {
        T::eq(self.deref(), other.deref()) // deref to T
    }
}

unsafe impl<T: StructuralEq + ?Sized> StructuralEqImpl for Rc<T> {
    default const EQ_ENABLED: bool = T::EQ_ENABLED;

    default fn eq(&self, other: &Self) -> bool {
        T::eq(self.deref(), other.deref()) // deref to T
    }
}

unsafe impl<T: StructuralEq + ?Sized> StructuralEqImpl for Arc<T> {
    default const EQ_ENABLED: bool = T::EQ_ENABLED;

    default fn eq(&self, other: &Self) -> bool {
        T::eq(self.deref(), other.deref()) // deref to T
    }
}

unsafe impl<T: StructuralEq + Copy + ?Sized> StructuralEqImpl for Cell<T> {
    const EQ_ENABLED: bool = T::EQ_ENABLED;

    fn eq(&self, other: &Self) -> bool {
        T::eq(&self.get(), &other.get())
    }
}

unsafe impl<T: StructuralEq + ?Sized> StructuralEqImpl for RefCell<T> {
    const EQ_ENABLED: bool = T::EQ_ENABLED;

    fn eq(&self, other: &Self) -> bool {
        T::eq(self.borrow().deref(), other.borrow().deref()) // deref to T
    }
}

// // If &T / Arc / Rc are immutable, we just need to compare pointers to test the
// // equality.

unsafe impl<T: Immutable + StructuralEq + ?Sized> StructuralEqImpl for &T {
    default const EQ_ENABLED: bool = true;

    default fn eq(&self, other: &Self) -> bool {
        // Because contents of T are immutable, we only need to compare pointers
        // to check the equality.
        *self as *const _ == *other as *const _
    }
}

unsafe impl<T: Immutable + StructuralEq + ?Sized> StructuralEqImpl for Rc<T> {
    const EQ_ENABLED: bool = true;

    fn eq(&self, other: &Self) -> bool {
        // Because contents of T are immutable, we only need to compare pointers
        // to check the equality.
        Rc::as_ptr(self) == Rc::as_ptr(other)
    }
}

unsafe impl<T: Immutable + StructuralEq + ?Sized> StructuralEqImpl for Arc<T> {
    const EQ_ENABLED: bool = true;

    fn eq(&self, other: &Self) -> bool {
        // Because contents of T are immutable, we only need to compare
        // pointers to check the equality.
        Arc::as_ptr(self) == Arc::as_ptr(other)
    }
}

//
// Primitives and built-in types
//

// Following implementations may sometimes lead to false negatives
// where two strings/slices are equal but have different pointers.
//
// IMHO this is fine and better than not comparing them at all.

unsafe impl StructuralEqImpl for str {
    const EQ_ENABLED: bool = true;

    fn eq(&self, other: &Self) -> bool {
        let c1 = self as *const _ == other as *const _;
        let c2 = (*self).len() == (*other).len();

        c1 && c2
    }
}

unsafe impl<T: Immutable> StructuralEqImpl for [T] {
    const EQ_ENABLED: bool = true;

    fn eq(&self, other: &Self) -> bool {
        let c1 = self as *const _ == other as *const _;
        let c2 = (*self).len() == (*other).len();

        c1 && c2
    }
}

macro_rules! impl_eq_for_primitives {
    ($($t:tt)*) => ($(
         unsafe impl StructuralEqImpl for $t {
            const EQ_ENABLED: bool = true;

            fn eq(&self, other: &Self) -> bool {
                self == other
            }
        }
    )*)
}

impl_eq_for_primitives! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64 char bool ()
}
