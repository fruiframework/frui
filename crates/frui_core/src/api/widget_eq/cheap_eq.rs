use std::{
    cell::{Cell, RefCell, UnsafeCell},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

pub auto trait Immutable {}

impl<T> !Immutable for UnsafeCell<T> {}
impl<T: Immutable> Immutable for Rc<T> {}
impl<T: Immutable> Immutable for Arc<T> {}

/// This trait is automatically implemented by `#[derive(WidgetKind)]` macro
/// and shouldn't be implemented manually for widgets.
///
/// # About
///
/// This trait is used to compare widget configurations (structures) to
/// optimize widget tree rebuilds.
///
/// # Safety
///
/// To implement this trait, you must ensure that you do not perform any
/// mutable operations on `self` and `other` (in `cheap_eq` method) which could
/// cause UB due to lifetime mismatch (e.g. dangling-references).
///
/// Additionally, the framework makes assumptions about the way two structures
/// are tested for equality, which are automatically upheld when implementing
/// through macro.
///
/// *todo: those assumptions*
pub unsafe trait CheapEq {
    /// This associated type is used to avoid recursive comparisons of the whole
    /// widget subtree (if one of the fields is itself a widget).
    const CHEAP_TO_EQ: bool;

    fn cheap_eq(&self, other: &Self) -> bool;
}

unsafe impl<T: ?Sized> CheapEq for T {
    default const CHEAP_TO_EQ: bool = false;

    default fn cheap_eq(&self, _: &Self) -> bool {
        false
    }
}

unsafe impl<T: CheapEq + ?Sized> CheapEq for &T {
    default const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    default fn cheap_eq(&self, other: &Self) -> bool {
        // Only if pointers to T are equal, can compare the inner-contents.
        // Otherwise this could cause use-after-free after we deallocated
        // appropriate parent widget.
        if *self as *const _ == *other as *const _ {
            T::cheap_eq(self.deref(), other.deref()) // deref to T
        } else {
            false
        }
    }
}

unsafe impl<T: CheapEq + ?Sized> CheapEq for &mut T {
    default const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    default fn cheap_eq(&self, other: &Self) -> bool {
        // Only if pointers to T are equal, can compare the inner-contents.
        // Otherwise this could cause use-after-free after we deallocated
        // appropriate parent widget.
        if *self as *const _ == *other as *const _ {
            T::cheap_eq(self.deref(), other.deref()) // deref to T
        } else {
            false
        }
    }
}

unsafe impl<T: CheapEq + ?Sized> CheapEq for Box<T> {
    default const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    default fn cheap_eq(&self, other: &Self) -> bool {
        T::cheap_eq(self.deref(), other.deref()) // deref to T
    }
}

unsafe impl<T: CheapEq + ?Sized> CheapEq for Rc<T> {
    default const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    default fn cheap_eq(&self, other: &Self) -> bool {
        T::cheap_eq(self.deref(), other.deref()) // deref to T
    }
}

unsafe impl<T: CheapEq + ?Sized> CheapEq for Arc<T> {
    default const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    default fn cheap_eq(&self, other: &Self) -> bool {
        T::cheap_eq(self.deref(), other.deref()) // deref to T
    }
}

unsafe impl<T: CheapEq + Copy + ?Sized> CheapEq for Cell<T> {
    const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    fn cheap_eq(&self, other: &Self) -> bool {
        T::cheap_eq(&self.get(), &other.get())
    }
}

unsafe impl<T: CheapEq + ?Sized> CheapEq for RefCell<T> {
    const CHEAP_TO_EQ: bool = T::CHEAP_TO_EQ;

    fn cheap_eq(&self, other: &Self) -> bool {
        T::cheap_eq(self.borrow().deref(), other.borrow().deref()) // deref to T
    }
}

// If &T / Arc / Rc are immutable, we just need to compare pointers to test the
// equality.

unsafe impl<T: Immutable + CheapEq + ?Sized> CheapEq for &T {
    default const CHEAP_TO_EQ: bool = true;

    default fn cheap_eq(&self, other: &Self) -> bool {
        // Because contents of T are immutable, we only need to compare
        // pointers to check the equality.
        *self as *const _ == *other as *const _
    }
}

unsafe impl<T: Immutable + CheapEq + ?Sized> CheapEq for Rc<T> {
    const CHEAP_TO_EQ: bool = true;

    fn cheap_eq(&self, other: &Self) -> bool {
        // Because contents of T are immutable, we only need to compare
        // pointers to check the equality.
        Rc::as_ptr(self) == Rc::as_ptr(other)
    }
}

unsafe impl<T: Immutable + CheapEq + ?Sized> CheapEq for Arc<T> {
    const CHEAP_TO_EQ: bool = true;

    fn cheap_eq(&self, other: &Self) -> bool {
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

unsafe impl CheapEq for str {
    const CHEAP_TO_EQ: bool = true;

    fn cheap_eq(&self, other: &Self) -> bool {
        let c1 = self as *const _ == other as *const _;
        let c2 = (*self).len() == (*other).len();

        c1 && c2
    }
}

unsafe impl<T: Immutable> CheapEq for [T] {
    const CHEAP_TO_EQ: bool = true;

    fn cheap_eq(&self, other: &Self) -> bool {
        let c1 = self as *const _ == other as *const _;
        let c2 = (*self).len() == (*other).len();

        c1 && c2
    }
}

macro_rules! impl_cheap_eq_for_primitives {
    ($($t:tt)*) => ($(
        unsafe impl CheapEq for $t {
            const CHEAP_TO_EQ: bool = true;

            fn cheap_eq(&self, other: &Self) -> bool {
                self == other
            }
        }
    )*)
}

impl_cheap_eq_for_primitives! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64 char bool ()
}
