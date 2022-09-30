use frui::prelude::Widget;

/// Represents a collection of widgets.
///
///
/// # Example usage
///
/// ## Widget tuple
///
/// Most often you will use [`WidgetList`] in a form of a tuple:
///
/// ```
/// Column::builder()
///     .children((WidgetA, WidgetB, WidgetC));
/// ```
///
/// This pattern is great if you have a statically known list of widgets, since
/// each child widget can have different types.
///
/// Do note that currently only tuples with less than 50 widgets work, but in
/// the future this limit might be increased / lifted.
///
/// ## Widget vector / array / slice
///
/// If tuple pattern doesn't make it for you, you can also use vector, array
/// and slice as [`WidgetList`]. Those come with their respectful constraints,
/// like e.g. type stored in that collection must be the same - you can't have
/// widgets of different types, unless you box / type erase them:
///
/// ```
/// Column::builder()
///     .children(&widgets_a); // &[WidgetA]
/// Column::builder()
///     .children([WidgetB, WidgetB, WidgetB]); // [WidgetB]
/// Column::builder()
///     .children(vec![WidgetC, WidgetC, WidgetC]); // Vec<WidgetC>
///
/// // And type erased, e.g.:
/// Column::builder()
///     .children([
///         Box::new(WidgetA) as Box<dyn Widget>,
///         Box::new(WidgetB),
///         Box::new(WidgetC),
///     ]); // [Box<dyn Widget>]
/// Column::builder()
///     .children(vec![
///         &widget_a as &dyn Widget,
///         &widget_b,
///         &widget_c,
///     ]); // Vec<&dyn Widget>
/// ```
pub trait WidgetList {
    fn get(&self) -> Vec<&dyn Widget>;
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
