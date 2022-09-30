pub mod cheap_eq;

pub(crate) use sealed::WidgetEqOS;

mod sealed {
    use crate::{api::AnyExt, macro_exports::CheapEq};

    /// `OS` stands for "object safe".
    pub trait WidgetEqOS {
        /// Checks if two widget configurations are equal.
        fn eq(&self, other: &dyn AnyExt) -> bool;
    }

    impl<T: CheapEq> WidgetEqOS for T {
        fn eq(&self, other: &dyn AnyExt) -> bool {
            // Safety:
            //
            // `CheapEq` is implemented by `#[derive(WidgetKind)]` macro, which doesn't
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
                    Some(other) => <T as CheapEq>::cheap_eq(self, other),
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
}
