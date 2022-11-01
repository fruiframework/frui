use frui::prelude::*;

pub use alignment::*;
pub use center::*;
pub use column::*;
pub use row::*;
pub use stack::*;

pub mod alignment;
pub mod center;
pub mod column;
pub mod row;
pub mod stack;

#[derive(Debug, Clone, Copy, Default)]
pub struct BoxLayoutData {
    offset: Offset,
}

pub trait LayoutData<T = BoxLayoutData> {
    fn layout_data(&self) -> &T;

    fn layout_data_mut(&mut self) -> &mut T;
}

impl LayoutData for BoxLayoutData {
    fn layout_data(&self) -> &BoxLayoutData {
        self
    }

    fn layout_data_mut(&mut self) -> &mut BoxLayoutData {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MainAxisSize {
    Min,
    Max,
}

impl Default for MainAxisSize {
    fn default() -> Self {
        MainAxisSize::Min
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceEvenly,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        MainAxisAlignment::Start
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CrossAxisSize {
    Min,
    Max,
}

impl Default for CrossAxisSize {
    fn default() -> Self {
        CrossAxisSize::Min
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        CrossAxisAlignment::Start
    }
}

//
// Following functions are shared between Row and Column.

fn compute_cross_axis_offset(
    cross_axis_alignment: CrossAxisAlignment,
    initial_x: f64,
    self_width: f64,
    child_width: f64,
) -> f64 {
    match cross_axis_alignment {
        CrossAxisAlignment::Start => initial_x,
        CrossAxisAlignment::End => initial_x + self_width - child_width,
        CrossAxisAlignment::Center => initial_x + (self_width - child_width) / 2.,
        CrossAxisAlignment::Stretch => initial_x,
    }
}

fn compute_main_axis_offset(
    space_between: f64,
    main_axis_size: MainAxisSize,
    main_axis_alignment: MainAxisAlignment,
    //
    total_flex: f64,
    children_count: f64,
    remaining_space: f64, // On the main axis, after laying out inflexible children.
    inflexible_children_height: f64,
) -> (f64, f64, f64, f64) {
    let has_flex = total_flex > 0.;

    let initial_offset;
    let space_between_children;

    if has_flex {
        initial_offset = 0.0;
        space_between_children = space_between;
    } else if let MainAxisSize::Min = main_axis_size {
        if let MainAxisAlignment::SpaceEvenly = main_axis_alignment {
            initial_offset = space_between;
            space_between_children = space_between;
        } else {
            initial_offset = 0.0;
            space_between_children = space_between;
        }
    } else {
        match main_axis_alignment {
            // Place the children as close to the start of the main axis as possible.
            MainAxisAlignment::Start => {
                initial_offset = 0.0;
                space_between_children = space_between;
            }
            // Place the children as close to the end of the main axis as possible.
            MainAxisAlignment::End => {
                let available_space =
                    (remaining_space - space_between * (children_count - 1.)).max(0.);

                initial_offset = available_space;
                space_between_children = space_between;
            }
            // Place the children as close to the middle of the main axis as possible.
            MainAxisAlignment::Center => {
                let available_space =
                    (remaining_space - space_between * (children_count - 1.)).max(0.);

                initial_offset = available_space / 2.;
                space_between_children = space_between;
            }
            // Place the free space evenly between the children.
            MainAxisAlignment::SpaceBetween => {
                // Maximum space between children calculated from the available space.
                let x = remaining_space / (children_count - 1.).max(1.); // max(1) to avoid dividing by 0

                initial_offset = 0.0;
                space_between_children = space_between.max(x);
            }
            // Place the free space evenly between the children as well as before and
            // after the first and last child.
            MainAxisAlignment::SpaceEvenly => {
                // Maximum space between children calculated from the available space.
                let x = remaining_space / (children_count + 1.);

                space_between_children = space_between.max(x);
                initial_offset = space_between_children;
            }
        };
    }

    let space_between_count = match main_axis_alignment {
        MainAxisAlignment::SpaceEvenly => children_count + 1.,
        _ => children_count - 1.,
    }
    .max(0.);

    let space_per_flex = if total_flex != 0.0 {
        ((
            // Remaining available space on the main axis.
            remaining_space
            // Space in-between every child.
            - space_between_count * space_between_children)
            // Space available for flex divided by total flex.
            / total_flex)
            // Cap at 0.0 if negative.
            .max(0.0)
    } else {
        0.0
    };

    let total_height = space_between_children * space_between_count
        + inflexible_children_height
        + total_flex * space_per_flex;

    (
        initial_offset,
        space_between_children,
        space_per_flex,
        total_height,
    )
}

fn get_flex(child: &ChildContext) -> usize {
    match child.try_parent_data::<FlexData>() {
        Some(data) => data.flex_factor,
        None => 0,
    }
}

/// Used by flexible widgets to determine the flex factor of a child.
pub struct FlexData {
    flex_factor: usize,
}

//
// Todo:

pub enum FlexFit {
    Loose,
    Tight,
}

#[derive(RenderWidget)]
pub struct Flexible<W: Widget> {
    pub fit: FlexFit,
    pub flex: usize,
    pub child: W,
}

impl<W: Widget> ParentData for Flexible<W> {
    type Data = FlexData;

    fn create_data(&self) -> Self::Data {
        FlexData {
            flex_factor: self.flex,
        }
    }
}

impl<W: Widget> RenderWidget for Flexible<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}
