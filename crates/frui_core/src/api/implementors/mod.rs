pub mod inherited;
pub mod leaf;
pub mod multi;
pub mod single;
pub mod view;

/// You can implement this trait using derive macro `#[derive(WidgetKind)]`.
pub trait WidgetDerive {
    /// Assuming this trait was derived through `#[derive(WidgetKind)]` this associated type should be
    /// derived automatically by the compiler. This requires that TAIT feature is enabled.
    type Widget<'a>: super::Widget
    where
        Self: 'a;

    #[doc(hidden)]
    type UniqueTypeId: 'static;
}
