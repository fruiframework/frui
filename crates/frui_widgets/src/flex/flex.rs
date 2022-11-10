use frui::prelude::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn max(&self, constraints: Constraints) -> f64 {
        match self {
            Axis::Horizontal => constraints.max_width,
            Axis::Vertical => constraints.max_height,
        }
    }

    fn flip(&self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum VerticalDirection {
    Up,
    Down,
}

pub struct Column;

impl Column {
    pub fn builder() -> Flex<()> {
        Flex::builder().direction(Axis::Vertical)
    }
}

pub struct Row;

impl Row {
    pub fn builder() -> Flex<()> {
        Flex::builder().direction(Axis::Horizontal)
    }
}

#[derive(RenderWidget, Builder)]
pub struct Flex<WL: WidgetList> {
    pub children: WL,
    pub direction: Axis,
    pub text_direction: TextDirection,
    pub vertical_direction: VerticalDirection,
    pub space_between: f64,
    pub main_axis_size: MainAxisSize,
    pub cross_axis_size: CrossAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
}

impl Flex<()> {
    pub fn builder() -> Self {
        Self {
            children: (),
            direction: Axis::Horizontal,
            text_direction: TextDirection::Ltr,
            vertical_direction: VerticalDirection::Down,
            space_between: 0.0,
            // The default differs from Flutter, but the reasoning is to allow
            // "Column in Column" or "Row in Row" without the need to specify
            // `MainAxisSize::Min` everytime.
            main_axis_size: MainAxisSize::Min,
            cross_axis_size: CrossAxisSize::Min,
            // Todo: Since `MainAxisSize` is `Min` by default, maybe set
            // `MainAxisAlignment` to `Center` by default?
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
        }
    }
}

impl<WL: WidgetList> RenderWidget for Flex<WL> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let main_size_max = self.direction.max(constraints);
        let can_flex = main_size_max < f64::INFINITY;
        let child_count = ctx.children().len();

        for child in ctx.children() {
            if child.try_parent_data::<FlexData>().is_none() {
                child.set_parent_data(FlexData::default());
            }
        }

        let InflexResult {
            flex_count,
            allocated_space,
        } = self.layout_inflexible(ctx.children(), constraints);

        let flexible = flex_count > 0;

        let MainAxisSizes {
            total_min,
            padding_top,
            space_between,
        } = self.compute_main_sizes(flexible, child_count, constraints, allocated_space);

        let free_space = (main_size_max - total_min).max(0.);

        if flexible {
            assert!(can_flex, "flex received unbounded constraints");
            self.layout_flexible(ctx.children(), constraints, free_space, flex_count);
        }

        //
        // Position chlidren:

        let mut main_offset = padding_top;

        for child in ctx.children() {
            let child_main = *child.size().main(self.direction);
            let child_offset = &mut child
                .try_parent_data_mut::<FlexData>()
                .unwrap()
                .box_data
                .offset;
            *child_offset.main(self.direction) = main_offset;
            main_offset += child_main + space_between;
        }

        let mut size = constraints.biggest();

        // Ensure overflow error appears when there is no space to lay out
        // flexible children of size of at least 0.
        let main_size = size.main(self.direction);
        *main_size = main_size.max(total_min);
        size

        // For some reason Row renders correctly, and Column not!
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        for child in ctx.children() {
            let child_offset: Offset = child
                .try_parent_data::<FlexData>()
                .map_or(*offset, |d| (*offset + d.offset));
            child.paint(canvas, &child_offset);
        }
    }
}

fn is_flex(c: &ChildContext) -> bool {
    get_flex(c).unwrap_or(0) > 0
}

impl<WL: WidgetList> Flex<WL> {
    fn get_main_size(&self, size: &Size) -> f64 {
        match self.direction {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    fn get_cross_size(&self, size: &Size) -> f64 {
        match self.direction {
            Axis::Horizontal => size.height,
            Axis::Vertical => size.width,
        }
    }

    fn layout_inflexible(&self, children: ChildIter, constraints: Constraints) -> InflexResult {
        let mut flex_count = 0;
        let mut allocated_space = 0.;

        // Compute total flex and layout non-flexible children
        for child in children.clone() {
            let flex: usize = get_flex(&child).unwrap_or(0);

            if flex > 0 {
                flex_count += flex;
            } else {
                let child_constraints = match self.cross_axis_alignment {
                    CrossAxisAlignment::Stretch => match self.direction {
                        Axis::Horizontal => {
                            Constraints::new_tight_for(None, Some(constraints.max_height))
                        }
                        Axis::Vertical => {
                            Constraints::new_tight_for(Some(constraints.max_width), None)
                        }
                    },
                    _ => match self.direction {
                        Axis::Horizontal => {
                            Constraints::new(0.0, f64::INFINITY, 0.0, constraints.max_height)
                        }
                        Axis::Vertical => {
                            Constraints::new(0.0, constraints.max_width, 0.0, f64::INFINITY)
                        }
                    },
                };

                let child_size = child.layout(child_constraints);
                allocated_space += self.get_main_size(&child_size);
                // cross_size = cross_size.max(self.get_cross_size(&child_size));
            }
        }

        InflexResult {
            flex_count,
            allocated_space,
        }
    }

    /// Todo: Maybe separate laying out from computing offsets? Like get flex
    /// count buy laying out manually, but keep it in a separate function to
    /// compute the `padding`, `space_between`, etc. ?
    fn compute_main_sizes(
        &self,
        flexible: bool,
        child_count: usize,
        constraints: Constraints,
        allocated_space: f64,
    ) -> MainAxisSizes {
        use MainAxisAlignment::*;

        // Caller should enforce following requirements.
        assert!(child_count >= 1);
        assert!(self.space_between >= 0.0);

        let child_count = child_count as f64;
        let total_space = match self.direction {
            Axis::Horizontal => constraints.max_width,
            Axis::Vertical => constraints.max_height,
        };

        // Space between at least 2 children, not padding before first and last child.
        let space_between;

        if !flexible {
            let available = total_space - allocated_space;

            space_between = match self.main_axis_alignment {
                // Start:        [[][XX]--------]
                // Center:       [----[][XX]----]
                // End:          [--------[][XX]]
                Start | Center | End => 0.0,
                // SpaceBetween: [[]--------[XX]]
                SpaceBetween => available / (child_count - 1.),
                // SpaceAround:  [--[]----[XX]--]
                SpaceAround => available / child_count,
                // SpaceEvenly:  [---[]---[XX]---]
                SpaceEvenly => available / (child_count + 1.),
            }
        } else {
            space_between = 0.0;
        }

        // Actual space between taking into account the minimum.
        let space_between = space_between.max(self.space_between);

        // Space from first child to end of last child (including the space
        // between those children).
        let back_to_back = space_between * (child_count - 1.) + allocated_space;

        // Padding before the first child.
        let padding_top;

        if flexible {
            match self.main_axis_alignment {
                SpaceAround => padding_top = space_between / 2.,
                SpaceEvenly => padding_top = space_between,
                _ => padding_top = 0.0,
            }
        } else {
            padding_top = match self.main_axis_alignment {
                Start | SpaceBetween => 0.0,
                End => total_space - back_to_back,
                Center => (total_space - back_to_back) / 2.,
                SpaceAround => space_between / 2.,
                SpaceEvenly => space_between,
            }
        };

        // In case it's negative (if constraints are too small to fit).
        let padding_top = padding_top.max(0.);

        // Total space if each of flex widgets had 0 size.
        let total_min = {
            match self.main_axis_alignment {
                Start | Center | End | SpaceBetween => back_to_back,
                SpaceAround => padding_top + back_to_back + padding_top,
                SpaceEvenly => padding_top + back_to_back + padding_top,
            }
        };

        MainAxisSizes {
            total_min,
            padding_top,
            space_between,
        }
    }

    fn layout_flexible(
        &self,
        children: ChildIter,
        constraints: Constraints,
        free_space: f64,
        flex_count: usize,
    ) {
        let space_per_flex = free_space / (flex_count as f64);

        for child in children.filter(is_flex) {
            let flex = child.try_parent_data::<FlexData>().unwrap().flex_factor;

            let max_child_extent = space_per_flex * flex as f64;

            let min_child_extent = match get_fit(&child).unwrap() {
                FlexFit::Loose => 0.0,
                FlexFit::Tight => max_child_extent,
            };

            // FitFlex::Tight forces tight constraints on its child.
            // FitFlex::Loose forces loose constraints on its child.
            //
            // Can we implement this differently?

            let flex_constraints = match self.cross_axis_alignment {
                CrossAxisAlignment::Stretch => match self.direction {
                    Axis::Horizontal => Constraints {
                        min_width: min_child_extent,
                        max_width: max_child_extent,
                        min_height: constraints.max_height,
                        max_height: constraints.max_height,
                    },
                    Axis::Vertical => Constraints {
                        min_width: constraints.max_width,
                        max_width: constraints.max_width,
                        min_height: min_child_extent,
                        max_height: max_child_extent,
                    },
                },
                _ => match self.direction {
                    Axis::Horizontal => Constraints {
                        min_width: min_child_extent,
                        max_width: max_child_extent,
                        min_height: 0.0,
                        max_height: constraints.max_height,
                    },
                    Axis::Vertical => Constraints {
                        min_width: 0.0,
                        max_width: constraints.max_width,
                        min_height: min_child_extent,
                        max_height: max_child_extent,
                    },
                },
            };

            child.layout(flex_constraints);
        }
    }
}

#[derive(Debug)]
struct InflexResult {
    flex_count: usize,
    allocated_space: f64,
}

fn get_flex(child: &ChildContext) -> Option<usize> {
    child.try_parent_data::<FlexData>().map(|d| d.flex_factor)
}

/// Todo: Use this!
fn get_fit(child: &ChildContext) -> Option<FlexFit> {
    child.try_parent_data::<FlexData>().map(|d| d.fit)
}

#[derive(Debug)]
struct MainAxisSizes {
    // 1. Here return `total_min: f64` which is how big column is on the main
    //    axis if every flexible widget had size 0.
    // 2. In the caller do `total_flex_space = (constraints.max_size - total_min).max(0.)`.
    // 3. Then do `space_per_flex= total_flex_space / flex_count`.
    // 4. The rest is easy.
    /// Total size of [`Flex`] if every flexible had size 0.
    total_min: f64,
    /// Padding before first child.
    padding_top: f64,
    /// Space between children.
    space_between: f64,
}

#[derive(Debug)]
struct FlexLayoutSizes {
    /// Total size of a widget on the main axis. It includes whitespace.
    main_size: f64,
    /// Total size of a widget on the cross axis.
    cross_size: f64,
    /// Sum of the sizes of the non-flexible children.
    allocated_size: f64,
}

fn start_is_top_left(
    direction: &Axis,
    text_direction: &TextDirection,
    vertical_direction: &VerticalDirection,
) -> bool {
    match direction {
        Axis::Horizontal => match text_direction {
            TextDirection::Ltr => true,
            TextDirection::Rtl => false,
        },
        Axis::Vertical => match vertical_direction {
            VerticalDirection::Up => false,
            VerticalDirection::Down => true,
        },
    }
}

trait AxisExt {
    fn main(&mut self, axis: Axis) -> &mut f64;
}

impl AxisExt for Offset {
    fn main(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.x,
            Axis::Vertical => &mut self.y,
        }
    }
}

impl AxisExt for Size {
    fn main(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.width,
            Axis::Vertical => &mut self.height,
        }
    }
}
