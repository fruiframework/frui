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
    pub clip: bool,
    pub alignment: A,
    pub fit: StackFit,
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

impl Stack<(), AlignmentDirectional> {
    pub fn builder() -> Self {
        Stack {
            clip: true,
            children: (),
            fit: StackFit::Loose,
            alignment: AlignmentDirectional::TOP_START,
            text_direction: TextDirection::Ltr,
        }
    }

    fn is_positioned(child: &LayoutCtxOS) -> bool {
        child
            .try_parent_data::<StackLayoutData>()
            .map_or(false, |d| d.is_positioned())
    }

    fn layout_positioned_child(child: &mut LayoutCtxOS, size: Size, alignment: &Alignment) -> bool {
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
}

impl<WL: WidgetList, A: AlignmentGeometry> Stack<WL, A> {
    fn get_layout_offset(&self, child: &PaintCtxOS, alignment: &Alignment, size: Size) -> Offset {
        let child_size = child.size();
        child.try_parent_data::<StackLayoutData>().map_or_else(
            || alignment.along(size - child_size),
            |data| data.base.offset,
        )
    }
}

impl<WL: WidgetList, A: AlignmentGeometry> RenderWidget for Stack<WL, A> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        self.children.get()
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let alignment = self.alignment.resolve(&self.text_direction);
        let mut width = constraints.min_width;
        let mut height = constraints.min_height;
        let non_positioned_constraints = match self.fit {
            StackFit::Loose => constraints.loosen(),
            StackFit::Expand => Constraints::new_tight(constraints.biggest()),
            StackFit::Passthrough => constraints,
        };

        let mut non_positioned_children_count = 0;

        for child in ctx.children() {
            if !Stack::is_positioned(&child) {
                non_positioned_children_count += 1;
                let child_size = child.layout(non_positioned_constraints);
                width = width.max(child_size.width);
                height = height.max(child_size.height);
            }
        }

        let has_non_positioned_child = non_positioned_children_count > 0;

        let size = if has_non_positioned_child {
            if cfg!(debug_assertions) {
                if width == 0. || height == 0. {
                    if non_positioned_children_count != ctx.children().len() {
                        log::warn!(concat!(
                            "not positioned children in Stack have height or width 0. This is most likely a bug. ",
                            "Consider setting `fit` option of `Stack` to StackFit::Expand."
                        ))
                    }
                }
            }

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

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let size = ctx.size();
        let alignment = self.alignment.resolve(&self.text_direction);

        if self.clip {
            let r = canvas.with_save(|cv| {
                cv.clip(Rect::new(
                    offset.x,
                    offset.y,
                    offset.x + size.width,
                    offset.y + size.height,
                ));

                for mut child in ctx.children() {
                    let offset = *offset + self.get_layout_offset(&child, &alignment, size);
                    child.paint(cv, &offset);
                }

                Ok(())
            });
            r.unwrap();
        } else {
            for mut child in ctx.children() {
                let offset = *offset + self.get_layout_offset(&child, &alignment, size);
                child.paint(canvas, &offset);
            }
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
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
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
