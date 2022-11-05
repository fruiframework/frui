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
pub struct Column<T: WidgetList> {
    pub children: T,
    pub space_between: f64,
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_size: CrossAxisSize,
    pub cross_axis_alignment: CrossAxisAlignment,
}

impl Column<()> {
    pub fn builder() -> Self {
        Column {
            children: (),
            space_between: 0.0,
            main_axis_size: Default::default(),
            main_axis_alignment: Default::default(),
            cross_axis_size: Default::default(),
            cross_axis_alignment: Default::default(),
        }
    }
}

impl<WL: WidgetList> Column<WL> {
    /// List of children widgets to be laid out by the [`Column`].
    ///
    /// # Note
    ///
    /// See [`WidgetList`] for all the types that you can use as `children`.
    pub fn children(self, children: impl WidgetList) -> Column<impl WidgetList> {
        Column {
            children,
            space_between: self.space_between,
            main_axis_size: self.main_axis_size,
            main_axis_alignment: self.main_axis_alignment,
            cross_axis_size: self.cross_axis_size,
            cross_axis_alignment: self.cross_axis_alignment,
        }
    }

    /// Whether [`Column`] should take full available height or only the minimum to fit children.
    ///
    /// # Note
    ///
    /// The default is [`MainAxisSize::Min`].
    ///
    /// If one of your children is flexible, [`MainAxisSize`] of this [`Column`] will be the same
    /// as if it was set to [`MainAxisSize::Max`].
    pub fn main_axis_size(mut self, size: MainAxisSize) -> Self {
        self.main_axis_size = size;
        self
    }

    /// Specifies how [`Column`] should layout its children on the vertical axis.
    ///
    /// # Note
    ///
    /// The default is [`MainAxisAlignment::Start`].
    pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = alignment;
        self
    }

    /// Whether [`Column`] should take full available width or only the minimum to fit children.
    ///
    /// # Note
    ///
    /// The default is [`CrossAxisSize::Min`].
    pub fn cross_axis_size(mut self, size: CrossAxisSize) -> Self {
        self.cross_axis_size = size;
        self
    }

    /// Specifies how [`Column`] should layout its children on the horizontal axis.
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
    initial_offset_y: f64,
    space_between_y: f64,
}

impl<T: WidgetList> RenderState for Column<T> {
    type State = ColumnRenderState;

    fn create_state(&self) -> Self::State {
        ColumnRenderState {
            initial_offset_y: 0.,
            space_between_y: 0.,
        }
    }
}

impl<WL: WidgetList> RenderWidget for Column<WL> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: RenderContext<Self>, mut constraints: Constraints) -> Size {
        // Whether we can layout flexible items or not.
        let can_flex = constraints.max_height < f64::INFINITY;

        if let CrossAxisAlignment::Stretch = self.cross_axis_alignment {
            constraints.min_width = constraints.max_width;
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
                constraints.max_height -= size.height;

                let child_size = child.layout(constraints);
                size.height += child_size.height;
                size.width = size.width.max(child_size.width);
            }
        }

        //
        // Compute offsets and space between children.

        let (initial_offset_y, space_between_y, space_per_flex, total_height) =
            compute_main_axis_offset(
                self.space_between,
                self.main_axis_size,
                self.main_axis_alignment,
                total_flex as f64,
                ctx.children().len() as f64,
                (constraints.max_height - size.height).max(0.),
                size.height,
            );

        ctx.rstate_mut().space_between_y = space_between_y;
        ctx.rstate_mut().initial_offset_y = initial_offset_y;

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
                        max_height: space_per_flex * child_flex as f64,
                        ..constraints
                    };

                    let child_size = child.layout(constraints);
                    size.width = size.width.max(child_size.width);
                }
            }

            size.width = size.width.max(size.width);
        }

        match self.main_axis_size {
            MainAxisSize::Max => size.height = total_height.max(constraints.max_height),
            MainAxisSize::Min => size.height = total_height,
        };

        if let CrossAxisSize::Max = self.cross_axis_size {
            size.width = size.width.max(constraints.max_width)
        }

        size
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let self_width = ctx.size().width;
        let space_between_y = ctx.rstate().space_between_y;

        let mut offset_y = offset.y + ctx.rstate().initial_offset_y;

        for mut child in ctx.children() {
            let offset_x = compute_cross_axis_offset(
                self.cross_axis_alignment,
                offset.x,
                self_width,
                child.size().width,
            );

            let offset = Offset {
                x: offset_x,
                y: offset_y,
            };

            child.paint(canvas, &offset);
            offset_y += child.size().height + space_between_y;
        }
    }
}
