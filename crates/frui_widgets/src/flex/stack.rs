use crate::alignment::{Alignment, AlignmentDirectional};
use crate::{AlignmentGeometry, BoxLayoutData, LayoutData, TextDirection, WidgetList};

use frui::prelude::*;

pub enum StackFit {
    Loose,
    Expand,
    Passthrough,
}

#[derive(MultiChildWidget)]
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
    pub base: BoxLayoutData,
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

impl LayoutData for StackLayoutData {
    fn layout_data(&self) -> &BoxLayoutData {
        &self.base
    }

    fn layout_data_mut(&mut self) -> &mut BoxLayoutData {
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

            child_layout_data.base.offset = Offset { x, y };
        }
        has_visual_overflow
    }

    fn is_positioned(child: &ChildContext) -> bool {
        child
            .try_parent_data::<StackLayoutData>()
            .map_or(false, |d| d.is_positioned())
    }
}

impl<WL: WidgetList, A: AlignmentGeometry> Stack<WL, A> {
    pub fn children(self, children: impl WidgetList) -> Stack<impl WidgetList, A> {
        Stack {
            children,
            fit: self.fit,
            alignment: self.alignment,
            text_direction: self.text_direction,
        }
    }

    pub fn alignment(self, alignment: impl AlignmentGeometry) -> Stack<WL, impl AlignmentGeometry> {
        Stack {
            children: self.children,
            fit: self.fit,
            text_direction: self.text_direction,
            alignment,
        }
    }

    pub fn fit(mut self, fit: StackFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn text_direction(mut self, text_direction: TextDirection) -> Self {
        self.text_direction = text_direction;
        self
    }

    fn get_layout_offset(&self, child: &ChildContext, alignment: &Alignment, size: Size) -> Offset {
        let child_size = child.size();
        child.try_parent_data::<StackLayoutData>().map_or_else(
            || alignment.along(size - child_size),
            |data| data.base.offset,
        )
    }
}

impl<WL: WidgetList, A: AlignmentGeometry> MultiChildWidget for Stack<WL, A> {
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
                    layout_data.base.offset = alignment.along(size - child_size);
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

#[derive(SingleChildWidget, Default)]
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

impl<T> SingleChildWidget for Positioned<T>
where
    T: Widget,
{
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        ctx.child().layout(constraints)
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child().paint(canvas, offset)
    }
}
