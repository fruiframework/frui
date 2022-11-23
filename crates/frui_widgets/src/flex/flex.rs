use frui::prelude::*;
use frui::render::*;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Whether the children should be laid out vertically ([`Column`]) or
    /// horizontally ([`Row`]).
    pub direction: Axis,

    /// If `direction` is set to [`Horizontal`](Axis::Horizontal), it specifies
    /// whether first element in `children` should be on left or right of this
    /// [`Flex`]. It also affects what `cross_axis_size` considers start/end.
    pub text_direction: TextDirection,

    /// If `direction` is set to [`Vertical`](Axis::Vertical), it specifies
    /// whether first element in `children` should be on top or bottom of this
    /// [`Flex`]. It also affects what `cross_axis_size` considers start/end.
    pub vertical_direction: VerticalDirection,

    /// Minimum amount of space added between children widgets.
    pub space_between: f64,

    /// Whether [`Flex`] should take full available size on the main axis or
    /// only the minimum to fit the children.
    pub main_axis_size: MainAxisSize,

    /// Whether [`Flex`] should take full available size on the cross axis or
    /// only the minimum to fit the children.
    pub cross_axis_size: CrossAxisSize,

    /// How children should be laid out on the main axis.
    pub main_axis_alignment: MainAxisAlignment,

    /// How children should be laid out on the cross axis.
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
            main_axis_size: MainAxisSize::Min,
            cross_axis_size: CrossAxisSize::Min,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
        }
    }
}

impl<WL: WidgetList> RenderWidget for Flex<WL> {
    fn build<'w>(&'w self, _ctx: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let main_size_max = self.direction.max(constraints);
        let can_flex = main_size_max < f64::INFINITY;
        let child_count = ctx.children().len();

        //
        // Override child parent data to store calculated offsets.
        
        self.ensure_parent_data(ctx, || FlexData::default());

        //
        // Layout inflexible children.

        let InflexResult {
            flex_count,
            allocated_space,
            mut cross_size_min,
        } = self.layout_inflexible(ctx.children(), constraints);

        let flexible = flex_count > 0;

        //
        // Compute main axis sizes based on inflexible children.

        let MainAxisSizes {
            main_size_min,
            leading_space,
            space_between,
        } = self.compute_main_size(flexible, child_count, constraints, allocated_space);

        let free_space = (main_size_max - main_size_min).max(0.);

        //
        // Layout flexible children.

        if flexible {
            assert!(can_flex, "flex received unbounded constraints");

            let cross_size_min_flex =
                self.layout_flexible(ctx.children(), constraints, free_space, flex_count);

            cross_size_min = cross_size_min.max(cross_size_min_flex);
        }

        //
        // Compute size of this `Flex` widget.

        let size = self.compute_size(flexible, main_size_min, cross_size_min, constraints);

        let main_size = size.main(self.direction);
        let cross_size = size.cross(self.direction);

        //
        // Position chlidren.

        let (main_axis_flipped, mut main_offset) =
            self.compute_main_offset(main_size, leading_space);

        for child in ctx.children() {
            let child_size = child.size();
            let child_offset = &mut child
                .try_parent_data_mut::<FlexData>()
                .unwrap()
                .box_data
                .offset;

            if main_axis_flipped {
                main_offset -= child_size.main(self.direction);
                *child_offset.main_mut(self.direction) = main_offset;
                main_offset -= space_between;
            } else {
                *child_offset.main_mut(self.direction) = main_offset;
                main_offset += child_size.main(self.direction) + space_between;
            }

            let cross_offset =
                self.compute_cross_offset(child_size.cross(self.direction), cross_size);

            *child_offset.cross_mut(self.direction) = cross_offset;
        }

        size
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        for mut child in ctx.children() {
            let child_offset: Offset = child
                .try_parent_data::<FlexData>()
                .map_or(*offset, |d| (*offset + d.offset));
            child.paint(canvas, &child_offset);
        }
    }
}

impl<WL: WidgetList> Flex<WL> {
    fn layout_inflexible(&self, children: LayoutCtxIter, constraints: Constraints) -> InflexResult {
        let mut flex_count = 0;
        let mut cross_size_min = 0.0;
        let mut allocated_space = 0.0;

        // Compute total flex and layout non-flexible children
        for child in children.clone() {
            let flex: usize = get_flex(&child).unwrap_or(0);

            if flex > 0 {
                flex_count += flex;
            } else {
                let child_constraints = self.inflexible_constraints(constraints);

                let child_size = child.layout(child_constraints);
                allocated_space += child_size.main(self.direction);
                cross_size_min = f64::max(cross_size_min, child_size.cross(self.direction));
            }
        }

        InflexResult {
            flex_count,
            cross_size_min,
            allocated_space,
        }
    }

    fn inflexible_constraints(&self, constraints: Constraints) -> Constraints {
        match self.cross_axis_alignment {
            CrossAxisAlignment::Stretch => match self.direction {
                Axis::Horizontal => Constraints::new_tight_for(None, Some(constraints.max_height)),
                Axis::Vertical => Constraints::new_tight_for(Some(constraints.max_width), None),
            },
            _ => match self.direction {
                Axis::Horizontal => {
                    Constraints::new(0.0, f64::INFINITY, 0.0, constraints.max_height)
                }
                Axis::Vertical => Constraints::new(0.0, constraints.max_width, 0.0, f64::INFINITY),
            },
        }
    }

    fn compute_main_size(
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

        // Space between computed based on available space.
        let space_between;

        if !flexible && matches!(self.main_axis_size, MainAxisSize::Max) {
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

        // Space before the first child.
        let mut leading_space;

        if flexible || matches!(self.main_axis_size, MainAxisSize::Min) {
            match self.main_axis_alignment {
                SpaceAround => leading_space = space_between / 2.,
                SpaceEvenly => leading_space = space_between,
                _ => leading_space = 0.0,
            }
        } else {
            leading_space = match self.main_axis_alignment {
                Start | SpaceBetween => 0.0,
                End => total_space - back_to_back,
                Center => (total_space - back_to_back) / 2.,
                SpaceAround => space_between / 2.,
                SpaceEvenly => space_between,
            }
        };

        // In case it's negative (if constraints are too small to fit).
        leading_space = leading_space.max(0.);

        // Total space if each of flex widgets had 0 size.
        let main_size_min = match self.main_axis_alignment {
            Start | Center | End | SpaceBetween => back_to_back,
            SpaceAround | SpaceEvenly => leading_space + back_to_back + leading_space,
        };

        MainAxisSizes {
            main_size_min,
            leading_space,
            space_between,
        }
    }

    fn layout_flexible(
        &self,
        children: LayoutCtxIter,
        constraints: Constraints,
        mut free_space: f64,
        mut flex_count: usize,
    ) -> f64 {
        let mut cross_size_min = 0.;

        // Layout `FlexFit::Loose` children first since they can take less than
        // `space_per_flex * flex`, then layout `FlexFit::Tight` children which
        // must have that exact size.
        let flex_children = children.filter(is_flex);
        let children_fit_ordered = flex_children
            .clone()
            .filter(fit_loose)
            .chain(flex_children.filter(fit_tight));

        for child in children_fit_ordered {
            let flex = child.try_parent_data::<FlexData>().unwrap().flex_factor;

            let space_per_flex = free_space / (flex_count as f64);

            let max_child_extent = space_per_flex * flex as f64;

            let min_child_extent = match get_fit(&child).unwrap() {
                FlexFit::Loose => 0.0,
                FlexFit::Tight => max_child_extent,
            };

            let flex_constraints =
                self.flex_constraints(min_child_extent, max_child_extent, constraints);

            let child_size = child.layout(flex_constraints);

            flex_count -= flex;
            free_space -= child_size.main(self.direction);
            cross_size_min = f64::max(cross_size_min, child_size.cross(self.direction));
        }

        cross_size_min
    }

    fn flex_constraints(
        &self,
        min_child_extent: f64,
        max_child_extent: f64,
        constraints: Constraints,
    ) -> Constraints {
        match self.cross_axis_alignment {
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
        }
    }

    fn compute_size(
        &self,
        flexible: bool,
        main_size_min: f64,
        cross_size_min: f64,
        constraints: Constraints,
    ) -> Size {
        let mut size = constraints.biggest();

        let cross_size = match self.cross_axis_size {
            CrossAxisSize::Min => cross_size_min,
            CrossAxisSize::Max => constraints
                .biggest()
                .cross(self.direction)
                .max(cross_size_min),
        };

        *size.cross_mut(self.direction) = cross_size;

        let main_size = size.main_mut(self.direction);

        // If Flex contains flexible widgets, it will take all available space
        // on the main axis.
        //
        // We also make sure that overflow error appears when there is no space
        // to lay out flexible children of size of at least 0.
        if flexible || matches!(self.main_axis_size, MainAxisSize::Max) {
            *main_size = main_size.max(main_size_min)
        } else {
            *main_size = main_size_min
        }

        // As a temporary solution, we're rounding resulting size to mitigate
        // rounding errors so that overflow error doesn't occur.
        let main_size = size.main_mut(self.direction);
        *main_size = main_size.round();
        let cross_size = size.cross_mut(self.direction);
        *cross_size = cross_size.round();

        size
    }

    fn compute_main_offset(&self, main_size: f64, leading_space: f64) -> (bool, f64) {
        let main_offset;
        let main_axis_flipped = self.is_main_axis_flipped();

        if main_axis_flipped {
            main_offset = main_size - leading_space;
        } else {
            main_offset = leading_space
        }

        (main_axis_flipped, main_offset)
    }

    fn compute_cross_offset(&self, size: f64, total_size: f64) -> f64 {
        use CrossAxisAlignment::*;

        let available = total_size - size;

        match (self.cross_axis_alignment, self.start_is_top_left()) {
            (Start, true) | (End, false) => 0.0,
            (Start, false) | (End, true) => available,
            (Center, _) => available / 2.,
            (Stretch, _) => 0.0,
            (Baseline, _) => todo!("implement baseline alignment"),
        }
    }

    fn start_is_top_left(&self) -> bool {
        match (self.direction, self.text_direction, self.vertical_direction) {
            (Axis::Vertical, TextDirection::Ltr, _) => true,
            (Axis::Horizontal, _, VerticalDirection::Down) => true,
            _ => false,
        }
    }

    fn is_main_axis_flipped(&self) -> bool {
        match (self.direction, self.text_direction, self.vertical_direction) {
            (Axis::Vertical, _, VerticalDirection::Up) => true,
            (Axis::Horizontal, TextDirection::Rtl, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct InflexResult {
    flex_count: usize,
    cross_size_min: f64,
    allocated_space: f64,
}

#[derive(Debug)]
struct MainAxisSizes {
    /// Total size of [`Flex`] if every flexible child had size 0.
    main_size_min: f64,
    /// Padding before first child.
    leading_space: f64,
    /// Space between children.
    space_between: f64,
}

fn get_flex(child: &LayoutCtxOS) -> Option<usize> {
    child.try_parent_data::<FlexData>().map(|d| d.flex_factor)
}

fn get_fit(child: &LayoutCtxOS) -> Option<FlexFit> {
    child.try_parent_data::<FlexData>().map(|d| d.fit)
}

fn is_flex(c: &LayoutCtxOS) -> bool {
    get_flex(c).unwrap_or(0) > 0
}

fn fit_loose(c: &LayoutCtxOS) -> bool {
    get_fit(c).unwrap() == FlexFit::Loose
}

fn fit_tight(c: &LayoutCtxOS) -> bool {
    get_fit(c).unwrap() == FlexFit::Tight
}

trait AxisExt {
    fn main(&self, axis: Axis) -> f64;
    fn main_mut(&mut self, axis: Axis) -> &mut f64;

    fn cross(&self, axis: Axis) -> f64;
    fn cross_mut(&mut self, axis: Axis) -> &mut f64;
}

impl AxisExt for Offset {
    fn main(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.x,
            Axis::Vertical => self.y,
        }
    }

    fn main_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.x,
            Axis::Vertical => &mut self.y,
        }
    }

    fn cross(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.y,
            Axis::Vertical => self.x,
        }
    }

    fn cross_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.y,
            Axis::Vertical => &mut self.x,
        }
    }
}

impl AxisExt for Size {
    fn main(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.width,
            Axis::Vertical => self.height,
        }
    }

    fn main_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.width,
            Axis::Vertical => &mut self.height,
        }
    }

    fn cross(&self, axis: Axis) -> f64 {
        match axis {
            Axis::Horizontal => self.height,
            Axis::Vertical => self.width,
        }
    }

    fn cross_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            Axis::Horizontal => &mut self.height,
            Axis::Vertical => &mut self.width,
        }
    }
}
