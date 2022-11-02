use frui::prelude::*;

use crate::{
    widget_list::WidgetList,
    CrossAxisAlignment,
    CrossAxisSize,
    MainAxisAlignment,
    MainAxisSize,
};

use super::{compute_cross_axis_offset, compute_main_axis_offset, get_flex};

#[derive(RenderWidget)]
pub struct Row<T: WidgetList> {
    pub children: T,
    pub space_between: f64,
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_size: CrossAxisSize,
    pub cross_axis_alignment: CrossAxisAlignment,
}

impl Row<()> {
    pub fn builder() -> Self {
        Row {
            children: (),
            space_between: 0.0,
            main_axis_size: Default::default(),
            main_axis_alignment: Default::default(),
            cross_axis_size: Default::default(),
            cross_axis_alignment: Default::default(),
        }
    }
}

impl<WidgetList_: WidgetList> Row<WidgetList_> {
    /// List of children widgets to be laid out by the [`Row`].
    ///
    /// # Note
    ///
    /// See [`WidgetList`] for all the types that you can use as `children`.
    pub fn children(self, children: impl WidgetList) -> Row<impl WidgetList> {
        Row {
            children,
            space_between: self.space_between,
            main_axis_size: self.main_axis_size,
            main_axis_alignment: self.main_axis_alignment,
            cross_axis_size: self.cross_axis_size,
            cross_axis_alignment: self.cross_axis_alignment,
        }
    }

    /// Whether [`Row`] should take full available width or only the minimum to fit children.
    ///
    /// # Note
    ///
    /// The default is [`MainAxisSize::Min`].
    ///
    /// If one of your children is flexible, [`MainAxisSize`] of this [`Row`] will be the same
    /// as if it was set to [`MainAxisSize::Max`].
    pub fn main_axis_size(mut self, size: MainAxisSize) -> Self {
        self.main_axis_size = size;
        self
    }

    /// Specifies how [`Row`] should layout its children on the horizontal axis.
    ///
    /// # Note
    ///
    /// The default is [`MainAxisAlignment::Start`].
    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = alignment;
        self
    }

    /// Whether [`Row`] should take full available height or only the minimum to fit children.
    ///
    /// # Note
    ///
    /// The default is [`CrossAxisSize::Min`].
    pub fn cross_axis_size(mut self, size: CrossAxisSize) -> Self {
        self.cross_axis_size = size;
        self
    }

    /// Specifies how [`Row`] should layout its children on the vertical axis.
    ///
    /// # Note
    ///
    /// The default is [`MainAxisAlignment::Start`].
    pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = alignment;
        self
    }

    /// Minimum amount of empty space added between children widgets.
    ///
    /// # Note
    ///
    /// The default is 0.
    pub fn space_between(mut self, amount: f64) -> Self {
        assert!(amount >= 0.0);
        self.space_between = amount;
        self
    }
}

pub struct ColumnRenderState {
    initial_offset_x: f64,
    space_between_x: f64,
}

impl<T: WidgetList> RenderState for Row<T> {
    type State = ColumnRenderState;

    fn create_state(&self) -> Self::State {
        ColumnRenderState {
            initial_offset_x: 0.,
            space_between_x: 0.,
        }
    }
}

impl<T: WidgetList> RenderWidget for Row<T> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: RenderContext<Self>, mut constraints: Constraints) -> Size {
        // Whether we can layout flexible items or not.
        let can_flex = constraints.max_width < f64::INFINITY;

        if let CrossAxisAlignment::Stretch = self.cross_axis_alignment {
            constraints.min_height = constraints.max_height;
        }

        // Total size of the Column.
        let mut size = Size::default();
        // Total flex factor of all flexible children.
        let mut total_flex = 0;

        //
        // Layout inflexible items.

        for mut child in ctx.children() {
            let child_flex = get_flex(&child);

            if child_flex > 0 {
                total_flex += child_flex;
            } else {
                let mut constraints = constraints.clone();
                constraints.max_width -= size.width;

                let child_size = child.layout(constraints);
                size.height = size.height.max(child_size.height);
                size.width += child_size.width;
            }
        }

        //
        // Compute offsets and space between children.

        let (initial_offset_x, space_between_x, space_per_flex, total_width) =
            compute_main_axis_offset(
                self.space_between,
                self.main_axis_size,
                self.main_axis_alignment,
                total_flex as f64,
                ctx.children().len() as f64,
                (constraints.max_width - size.width).max(0.),
                size.width,
            );

        ctx.rstate_mut().space_between_x = space_between_x;
        ctx.rstate_mut().initial_offset_x = initial_offset_x;

        //
        // Layout flexible items.

        if total_flex > 0 {
            assert!(
                can_flex,
                "cannot layout `Column` of unbounded height with flexible children"
            );

            for mut child in ctx.children() {
                let child_flex = get_flex(&child);

                if child_flex > 0 {
                    let constraints = Constraints {
                        max_width: space_per_flex * child_flex as f64,
                        ..constraints
                    };

                    let child_size = child.layout(constraints);
                    size.height = size.height.max(child_size.height);
                }
            }

            size.height = size.height.max(size.height);
        }

        match self.main_axis_size {
            MainAxisSize::Max => size.width = total_width.max(constraints.max_width),
            MainAxisSize::Min => size.width = total_width,
        };

        if let CrossAxisSize::Max = self.cross_axis_size {
            size.height = size.height.max(constraints.max_height)
        }

        size
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let self_height = ctx.size().height;
        let space_between_x = ctx.rstate().space_between_x;

        let mut offset_x = offset.x + ctx.rstate().initial_offset_x;

        for mut child in ctx.children() {
            let offset_y = compute_cross_axis_offset(
                self.cross_axis_alignment,
                offset.y,
                self_height,
                child.size().height,
            );

            let offset = Offset {
                x: offset_x,
                y: offset_y,
            };

            child.paint(canvas, &offset);
            offset_x += child.size().width + space_between_x;
        }
    }
}

impl<T: WidgetList> HitTest for Row<T> {
    fn hit_test<'a>(&'a self, ctx: &'a mut HitTestCtx<Self>, point: Point) -> bool {
        if ctx.layout_box().contains(point) {
            for mut child in ctx.children() {
                // We don't transform children widgets apart from simple offset
                // translation, so we can use `hit_test_with_paint_offset`.
                if child.hit_test_with_paint_offset(point) {
                    return true;
                }
            }
        }

        false
    }
}
