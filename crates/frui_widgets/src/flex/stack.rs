use std::ops::{Deref, DerefMut};

use crate::alignment::{Alignment, AlignmentDirectional};
use crate::{AlignmentGeometry, BoxLayoutData, TextDirection, WidgetList};

use frui::prelude::*;

pub enum StackFit {
    Loose,
    Expand,
    Passthrough,
}

#[derive(RenderWidget, Builder)]
pub struct Stack<WL: WidgetList, A: AlignmentGeometry> {
    pub children: WL,
    pub fit: StackFit,
    pub alignment: A,
    pub text_direction: TextDirection,
}

/// RenderData which Stack's children should hold, if not the child widget
/// will be located at top-left with it's own size.
#[derive(Copy, Clone, Default, Debug)]
pub struct StackLayoutData {
    base: BoxLayoutData,
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,

    /// The child's width.
    ///
    /// Ignored if both left and right are `Some(f64)`
    pub width: Option<f64>,

    /// The child's height.
    ///
    /// Ignored if both top and bottom are 'Some(f64)`
    pub height: Option<f64>,
}

impl StackLayoutData {
    fn is_positioned(&self) -> bool {
        self.top.is_some()
            || self.right.is_some()
            || self.bottom.is_some()
            || self.left.is_some()
            || self.width.is_some()
            || self.height.is_some()
    }
}

impl Deref for StackLayoutData {
    type Target = BoxLayoutData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for StackLayoutData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Stack<(), AlignmentDirectional> {
    pub fn builder() -> Self {
        Stack {
            children: (),
            fit: StackFit::Loose,
            alignment: AlignmentDirectional::TOP_START,
            text_direction: TextDirection::Ltr,
        }
    }

    fn is_positioned(child: &ChildContext) -> bool {
        child
            .try_parent_data::<StackLayoutData>()
            .map_or(false, |d| d.is_positioned())
    }

    fn layout_positioned_child(
        child: &mut ChildContext,
        size: Size,
        alignment: &Alignment,
    ) -> bool {
        let mut has_visual_overflow = false;
        let mut child_constraints = Constraints::default();
        {
            let child_layout_data = child.try_parent_data::<StackLayoutData>().unwrap();
            if child_layout_data.left.is_some() && child_layout_data.right.is_some() {
                child_constraints = child_constraints.tighten(
                    Some(
                        size.width
                            - child_layout_data.left.unwrap()
                            - child_layout_data.right.unwrap(),
                    ),
                    None,
                );
            } else if child_layout_data.width.is_some() {
                child_constraints = child_constraints.tighten(child_layout_data.width, None);
            }

            if child_layout_data.top.is_some() && child_layout_data.bottom.is_some() {
                child_constraints = child_constraints.tighten(
                    None,
                    Some(
                        size.height
                            - child_layout_data.bottom.unwrap()
                            - child_layout_data.top.unwrap(),
                    ),
                );
            } else if child_layout_data.height.is_some() {
                child_constraints = child_constraints.tighten(None, child_layout_data.height);
            }
        }
        child.layout(child_constraints);
        let child_size = child.size();
        {
            let mut child_layout_data = child.try_parent_data_mut::<StackLayoutData>().unwrap();
            let x = child_layout_data.left.unwrap_or_else(|| {
                child_layout_data.right.map_or_else(
                    || alignment.along(size - child_size).x,
                    |right| size.width - right - child_size.width,
                )
            });

            let y = child_layout_data.top.unwrap_or_else(|| {
                child_layout_data.bottom.map_or_else(
                    || alignment.along(size - child_size).y,
                    |bottom| size.height - bottom - child_size.height,
                )
            });

            has_visual_overflow |= x < 0.0
                || x + child_size.width > size.width
                || y < 0.0
                || y + child_size.height > size.height;

            child_layout_data.offset = Offset { x, y };
        }
        has_visual_overflow
    }
}

impl<WL: WidgetList, A: AlignmentGeometry> Stack<WL, A> {
    fn get_layout_offset(&self, child: &ChildContext, alignment: &Alignment, size: Size) -> Offset {
        let child_size = child.size();
        child.try_parent_data::<StackLayoutData>().map_or_else(
            || alignment.along(size - child_size),
            |data| data.offset,
        )
    }
}

impl<WL: WidgetList, A: AlignmentGeometry> RenderWidget for Stack<WL, A> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let alignment = self.alignment.resolve(&self.text_direction);
        let mut width = constraints.min_width;
        let mut height = constraints.min_height;
        let non_positioned_constraints = match self.fit {
            StackFit::Loose => constraints.loosen(),
            StackFit::Expand => Constraints::tight(constraints.biggest()),
            StackFit::Passthrough => constraints,
        };
        let mut has_non_positioned_child = false;
        for mut child in ctx.children() {
            if !Stack::is_positioned(&child) {
                has_non_positioned_child = true;
                let child_size = child.layout(non_positioned_constraints);
                width = width.max(child_size.width);
                height = height.max(child_size.height);
            }
        }

        let size = if has_non_positioned_child {
            Size::new(width, height)
        } else {
            constraints.biggest()
        };

        for mut child in ctx.children() {
            let child_size = child.size();
            if !Stack::is_positioned(&child) {
                if let Some(mut layout_data) = child.try_parent_data_mut::<StackLayoutData>() {
                    layout_data.offset = alignment.along(size - child_size);
                }
            } else {
                Stack::layout_positioned_child(&mut child, size, &alignment);
            }
        }

        size
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, _: &Offset) {
        let alignment = self.alignment.resolve(&self.text_direction);
        let size = ctx.size();
        for mut child in ctx.children() {
            child.paint(canvas, &self.get_layout_offset(&child, &alignment, size));
        }
    }
}

impl<WL: WidgetList, A: AlignmentGeometry> HitTest for Stack<WL, A> {
    fn hit_test<'a>(&'a self, ctx: &'a mut HitTestCtx<Self>, point: Point) -> bool {
        if ctx.layout_box().contains(point) {
            for mut child in ctx.children().rev() {
                if child.hit_test_with_paint_offset(point) {
                    // If widget on top handled an event, it won't be passed to
                    // other children, so we can return early.
                    return true;
                }
            }

            return true;
        }

        false
    }
}

#[derive(RenderWidget, Builder)]
pub struct Positioned<T: Widget> {
    pub child: T,
    pub left: Option<f64>,
    pub right: Option<f64>,
    pub top: Option<f64>,
    pub bottom: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

impl<T: Widget> ParentData for Positioned<T> {
    type Data = StackLayoutData;

    fn create_data(&self) -> Self::Data {
        StackLayoutData {
            base: BoxLayoutData::default(),
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
}

impl<T> RenderWidget for Positioned<T>
where
    T: Widget,
{
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

impl Positioned<()> {
    pub fn builder() -> Positioned<()> {
        Positioned {
            child: (),
            left: None,
            right: None,
            top: None,
            bottom: None,
            width: None,
            height: None,
        }
    }
}
