use std::any::TypeId;

use self::implementors::{RawWidget, WidgetDerive};

pub(crate) mod any_ext;
pub(crate) mod contexts;
pub(crate) mod events;
pub(crate) mod implementors;
pub(crate) mod impls;
pub(crate) mod local_key;
pub(crate) mod structural_eq;
pub(crate) mod widget_ptr;

pub use widget_ptr::{IntoWidgetPtr, WidgetPtr};

pub trait Widget: RawWidget {
    fn as_raw(&self) -> &dyn RawWidget;
}

pub trait WidgetUniqueType {
    fn unique_type(&self) -> TypeId;
}

impl<T> WidgetUniqueType for T {
    default fn unique_type(&self) -> TypeId {
        unreachable!()
    }
}

impl<T: WidgetDerive> WidgetUniqueType for T {
    fn unique_type(&self) -> TypeId {
        std::any::TypeId::of::<T::UniqueTypeId>()
    }
}

pub trait WidgetDebug {
    fn debug_name(&self) -> &'static str;
    fn debug_name_short(&self) -> &'static str;
}

impl<T> WidgetDebug for T {
    default fn debug_name(&self) -> &'static str {
        let full_name = std::any::type_name::<T>();
        full_name
    }

    fn debug_name_short(&self) -> &'static str {
        let full_name = std::any::type_name::<T>();

        let mut start = 0;
        let mut end = full_name.len();

        for (n, char) in full_name.chars().enumerate() {
            if char == '<' {
                end = n;
                break;
            } else if char == ':' {
                start = n + 1;
            }
        }

        &full_name[start..end]
    }
}
