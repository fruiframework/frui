use std::ops::{Deref, DerefMut};

use frui::prelude::*;

pub use alignment::*;
pub use center::*;
pub use flex::*;
pub use stack::*;

pub mod alignment;
pub mod center;
pub mod flex;
pub mod stack;

#[derive(Debug, Clone, Copy, Default)]
pub struct BoxLayoutData {
    offset: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Leading space will not work with `MainAxisAlignment::SpaceAround` and `MainAxisAlignment::SpaceEvenly`
    SpaceAround,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
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
        CrossAxisAlignment::Baseline => todo!(),
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
            MainAxisAlignment::SpaceAround => todo!(),
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

//
// Todo:
#[derive(Debug, Clone, Copy)]
pub enum FlexFit {
    Loose,
    Tight,
}

/// Used by flexible widgets to determine the flex factor of a child.
#[derive(Debug, Clone, Copy)]
pub struct FlexData {
    fit: FlexFit,
    flex_factor: usize,
    box_data: BoxLayoutData,
}

impl Default for FlexData {
    fn default() -> Self {
        FlexData {
            flex_factor: 0,
            fit: FlexFit::Loose,
            box_data: BoxLayoutData::default(),
        }
    }
}

impl Deref for FlexData {
    type Target = BoxLayoutData;

    fn deref(&self) -> &Self::Target {
        &self.box_data
    }
}

impl DerefMut for FlexData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.box_data
    }
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
            fit: self.fit,
            box_data: BoxLayoutData::default(),
        }
    }
}

impl<W: Widget> RenderWidget for Flexible<W> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        constraints.constrain(ctx.child(0).layout(constraints))
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}

pub struct Expanded;

impl Expanded {
    pub fn new<T: Widget>(child: T) -> Flexible<T> {
        Flexible {
            fit: FlexFit::Tight,
            flex: 1,
            child,
        }
    }
}
