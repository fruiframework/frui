pub use frui::prelude::Widget;

/// Represents a list of widgets.
///
/// # Widget tuple
///
/// Most often you will use [`WidgetList`] in form of a tuple:
///
/// ```
/// Column::builder()
///     .children((WidgetA, WidgetB, WidgetC));
/// ```
///
/// This pattern is great if you have a statically known list of widgets, since
/// each child widget can have different type.
///
/// Do note that currently only tuples with less than 50 widgets work. To handle
/// bigger lists you can use [`wlist`] macro or [`Vec`]/array of widgets.
///
///
/// # Widget list macro
///
/// If tuple pattern doesn't make it for you, you can use [`wlist`] macro. It
/// automatically erases types of provided children, boxes them and puts inside
/// of a [`Vec`].
///
///
/// ```
/// Column::builder()
///     .children(wlist![WidgetA, WidgetB, WidgetC]);
/// ```
///
/// This pattern is great if number of widgets you have is greater than 50 since
/// widget tuple works only up to 50 widgets.
///
/// Do note that using [`wlist`] boxes every widget which may cause performance
/// issues if wildly overused.
///
/// ## Todo
///
/// It is possible to avoid boxing and allocating vec for that widget list, I've
/// written the idea on the Discord. Implement it.
///
///
/// # Widget vector / array / slice
///
/// As an alternative to the previous patterns, you can also use vectors, arrays
/// and slices as [`WidgetList`]. Those come with their respectful constraints,
/// like the fact that types of elements stored in these kinds of collections
/// must be the same.
///
/// [`wlist`]: (crate::wlist)
pub trait WidgetList {
    fn get(&self) -> Vec<&dyn Widget>;
}

/// See [`WidgetList`] documentation for usage.
#[macro_export]
macro_rules! wlist {
    ($($x:expr),* $(,)?) => {
        std::vec![$($crate::macro_exports::BoxedWidget::boxed($x)),*] as Vec<Box<dyn $crate::macro_exports::Widget>>
    }
}

frui_macros::impl_widget_list!(0..50);

impl<W: Widget> WidgetList for [W] {
    fn get(&self) -> Vec<&dyn Widget> {
        self.iter().map(|e| e as &dyn Widget).collect()
    }
}

impl<W: Widget> WidgetList for Vec<W> {
    fn get(&self) -> Vec<&dyn Widget> {
        self.iter().map(|e| e as &dyn Widget).collect()
    }
}

impl<W: Widget, const N: usize> WidgetList for [W; N] {
    fn get(&self) -> Vec<&dyn Widget> {
        self.iter().map(|e| e as &dyn Widget).collect()
    }
}
