use frui_macros::sealed;

use super::any_ext::AnyExt;

mod structural_eq;

pub use structural_eq::{StructuralEq, StructuralEqImpl};

#[sealed(crate)]
pub trait StructuralEqOS {
    fn eq(&self, other: &dyn AnyExt) -> bool;
}

impl<T: StructuralEq> StructuralEqOS for T {
    fn eq(&self, other: &dyn AnyExt) -> bool {
        // Safety:
        //
        // `StructuralEq` is implemented by `#[derive(WidgetKind)]` macro, which doesn't
        // mutate any data of a widget through interior mutability thus it can't cause
        // dangling pointers.
        //
        // Additionally, the procedural macro correctly compares every field of a widget
        // (and that includes comparing fields containing references) which is important
        // because, if a structure contains any references, incorrectly assuming that
        // two widgets are equal could result in dangling references (after preserving
        // old widget configuration).
        unsafe {
            match other.downcast_ref::<T>() {
                Some(other) => <T as StructuralEq>::eq(self, other),
                None => {
                    eprintln!(
                        "WidgetEqOS: can't compare widgets of different types. This is a bug."
                    );
                    false
                }
            }
        }
    }
}
