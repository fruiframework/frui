use frui::prelude::*;

use crate::*;

#[derive(PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
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

#[derive(RenderWidget, Builder)]
pub struct Flex<WL: WidgetList> {
    pub children: WL,
    pub direction: Axis,
    pub space_between: f64,
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub cross_axis_size: CrossAxisSize,
    pub text_direction: TextDirection,
    pub vertical_direction: VerticalDirection,
}

struct FlexLayoutSizes {
    main_size: f64,
    cross_size: f64,
    allocated_size: f64,
}

impl Flex<()> {
    pub fn builder() -> Self {
        Self {
            children: (),
            direction: Axis::Horizontal,
            main_axis_size: MainAxisSize::Max,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Center,
            text_direction: TextDirection::Ltr,
            vertical_direction: VerticalDirection::Down,
            space_between: 0f64,
            cross_axis_size: Default::default(),
        }
    }
}

impl<WL: WidgetList> Flex<WL> {
    fn get_flex(child: &ChildContext) -> Option<usize> {
        child.try_parent_data::<FlexData>().map(|d| d.flex_factor)
    }

    fn get_fit(child: &ChildContext) -> Option<FlexFit> {
        child.try_parent_data::<FlexData>().map(|d| d.fit)
    }

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

    /// compute flex and layout un-flexiable children and return sizes of flexiable children
    fn compute_sizes(
        &self,
        children: &mut Vec<ChildContext>,
        constraints: Constraints,
    ) -> FlexLayoutSizes {
        let mut total_flex = 0;
        let max_main_size = match self.direction {
            Axis::Horizontal => constraints.max_width,
            Axis::Vertical => constraints.max_height,
        };
        let child_count = children.len();
        let can_flex = max_main_size <= f64::INFINITY;
        let mut cross_size: f64 = 0.0;
        let mut allocated_size: f64 = match self.main_axis_alignment {
            MainAxisAlignment::SpaceAround => self.space_between / 2.0,
            MainAxisAlignment::SpaceEvenly => self.space_between,
            _ => 0.0,
        };
        let mut allocated_space_size = allocated_size;

        // compute total flex and layout un-flexiable children
        for child in children.iter_mut() {
            let flex: usize = Flex::<WL>::get_flex(&child).unwrap_or(0usize);
            if flex > 0 {
                total_flex += flex;
            } else {
                let child_constraints = match self.cross_axis_alignment {
                    CrossAxisAlignment::Stretch => match self.direction {
                        Axis::Horizontal => {
                            Constraints::tight_for(None, Some(constraints.max_height))
                        }
                        Axis::Vertical => Constraints::tight_for(Some(constraints.max_width), None),
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
                allocated_size += self.get_main_size(&child_size);
                cross_size = cross_size.max(self.get_cross_size(&child_size));
            }
        }
        let allocated_between_size = self.space_between * (child_count - 1) as f64;
        allocated_size += allocated_between_size;
        allocated_space_size += allocated_between_size;
        let free_space: f64 = 0f64.max(if can_flex { max_main_size } else { 0.0 } - allocated_size);
        let mut allocated_flex_space = 0.0;
        if total_flex > 0 {
            let space_per_flex = if can_flex {
                free_space / (total_flex as f64)
            } else {
                f64::NAN
            };
            for (idx, child) in children.iter_mut().enumerate() {
                let flex = Flex::<WL>::get_flex(&child).unwrap_or(0usize);
                if flex > 0 {
                    let max_child_extent = if can_flex {
                        if idx == child_count - 1 {
                            free_space - allocated_flex_space
                        } else {
                            space_per_flex * flex as f64
                        }
                    } else {
                        f64::INFINITY
                    };
                    let min_child_extent = match Flex::<WL>::get_fit(&child) {
                        Some(FlexFit::Tight) => max_child_extent,
                        _ => 0.0,
                    };
                    let inner_constraints = match self.cross_axis_alignment {
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
                    let child_size = child.layout(inner_constraints);
                    let child_main_size = self.get_main_size(&child_size);
                    assert!(child_main_size <= max_child_extent);
                    allocated_size += child_main_size;
                    allocated_flex_space += max_child_extent;
                    cross_size = cross_size.max(self.get_cross_size(&child_size));
                }
            }
        }

        let ideal_size = if can_flex && self.main_axis_size == MainAxisSize::Max {
            max_main_size
        } else {
            allocated_size
        };
        cross_size = match self.direction {
            Axis::Horizontal => match self.cross_axis_size {
                CrossAxisSize::Max => constraints.max_height,
                CrossAxisSize::Min => constraints.constrain_height(Some(cross_size)),
            },
            Axis::Vertical => match self.cross_axis_size {
                CrossAxisSize::Max => constraints.max_width,
                CrossAxisSize::Min => constraints.constrain_width(Some(cross_size)),
            },
        };
        FlexLayoutSizes {
            main_size: ideal_size,
            cross_size: cross_size,
            allocated_size: allocated_size - allocated_space_size,
        }
    }
}

impl<WL: WidgetList> RenderWidget for Flex<WL> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let mut children: Vec<ChildContext> = ctx.children().collect();
        let child_count = children.len();
        for child in children.iter_mut() {
            if child.try_parent_data::<FlexData>().is_none() {
                child.set_parent_data(FlexData::default());
            }
        }

        let sizes = self.compute_sizes(&mut children, constraints);

        let allocated_size = sizes.allocated_size;
        let actual_size = sizes.main_size;
        let cross_size = sizes.cross_size;
        // let mut max_baseline_distance: f64 = 0.0;
        if self.cross_axis_alignment == CrossAxisAlignment::Baseline {
            log::warn!("Baseline alignment not yet implemented");
            // TODO: support baseline alignment
        }

        let (size, actual_size, cross_size) = match self.direction {
            Axis::Horizontal => {
                let size = Size::new(actual_size, cross_size);
                (size, size.width, size.height)
            }
            Axis::Vertical => {
                let size = Size::new(cross_size, actual_size);
                (size, size.height, size.width)
            }
        };
        let actual_size_delta = actual_size - allocated_size;
        // let overflow = (-actual_size_delta).max(0.0);

        let remaining_space = actual_size_delta.max(0.0);
        let flip_main_axis = !start_is_top_left(
            &self.direction,
            &self.text_direction,
            &self.vertical_direction,
        );

        let multi_child_space = if child_count > 1 { self.space_between } else { 0.0 };
        let (leading_space, between_space) = match self.main_axis_alignment {
            MainAxisAlignment::Start => (0.0, multi_child_space),
            MainAxisAlignment::Center => (remaining_space / 2.0, multi_child_space),
            MainAxisAlignment::End => (remaining_space, multi_child_space),
            MainAxisAlignment::SpaceBetween => (
                0.0,
                if child_count > 1 {
                    multi_child_space.max(remaining_space / (child_count - 1) as f64)
                } else {
                    0.0
                },
            ),
            MainAxisAlignment::SpaceAround => {
                let space = remaining_space / child_count as f64;
                (space / 2.0, multi_child_space.max(space))
            }
            MainAxisAlignment::SpaceEvenly => {
                let space = if child_count > 0 {
                    remaining_space / (child_count + 1) as f64
                } else {
                    0.0
                };
                (space, multi_child_space.max(space))
            }
        };

        let mut child_main_position = if flip_main_axis {
            actual_size - leading_space
        } else {
            leading_space
        };

        for child in children.iter_mut() {
            let child_size = { child.size() };
            let mut child_parent_data = { child.try_parent_data_mut::<FlexData>().unwrap() };
            let child_cross_position = match self.cross_axis_alignment {
                CrossAxisAlignment::Start | CrossAxisAlignment::End => {
                    if start_is_top_left(
                        &self.direction.flip(),
                        &self.text_direction,
                        &self.vertical_direction,
                    ) == (self.cross_axis_alignment == CrossAxisAlignment::Start)
                    {
                        0f64
                    } else {
                        cross_size - self.get_cross_size(&child_size)
                    }
                }
                CrossAxisAlignment::Center => (cross_size - self.get_cross_size(&child_size)) / 2.0,
                CrossAxisAlignment::Stretch => 0.0,
                CrossAxisAlignment::Baseline => todo!("baseline alignment"),
            };

            if flip_main_axis {
                child_main_position -= self.get_main_size(&child_size);
            }
            child_parent_data.offset = match self.direction {
                Axis::Horizontal => Size::new(child_main_position, child_cross_position).into(),
                Axis::Vertical => Size::new(child_cross_position, child_main_position).into(),
            };
            if flip_main_axis {
                child_main_position -= between_space;
            } else {
                child_main_position += self.get_main_size(&child_size) + between_space;
            }
        }
        size
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        for mut child in ctx.children() {
            let child_offset: Offset = child
                .try_parent_data::<FlexData>()
                .map_or(*offset, |d| (d.offset + offset));
            child.paint(canvas, &child_offset);
        }
    }
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
