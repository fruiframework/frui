use frui::prelude::*;

use crate::{BoxChildWidget, BoxLayoutWidget, ChildWidget, BoxLayoutData};


#[derive(SingleChildWidget, Default)]
pub struct ConstrainedBox<T: BoxChildWidget> {
    pub child: T,
    pub contraints: Constraints,
}

impl<'a, T: BoxChildWidget> ParentData for ConstrainedBox<T> {
    type Data = BoxLayoutData;

    fn create_data(&self) -> Self::Data {
        BoxLayoutData::default()
    }
}

impl<T: BoxChildWidget> ChildWidget for ConstrainedBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: BoxChildWidget> SingleChildWidget for ConstrainedBox<T> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child().layout(constraints);
        if child_size != Size::ZERO {
            child_size
        } else {
            self.compute_size_for_no_child(constraints)
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child().paint(canvas, offset)
    }
}

impl<'a, T> BoxLayoutWidget for ConstrainedBox<T>
where
    T: BoxChildWidget + 'a
{
    default fn get_contraints(&self) -> Constraints {
        Constraints::default()
    }

    fn get_min_intrinsic_width(&self, height: f64) -> f64 {
        // wrap compute result with cache, maybe `memorize`?
        self.compute_min_intrinsic_width(height)
    }

    fn get_min_intrinsic_height(&self, width: f64) -> f64 {
        self.compute_min_intrinsic_height(width)
    }

    fn get_max_intrinsic_width(&self, height: f64) -> f64 {
        self.compute_max_intrinsic_width(height)
    }

    fn get_max_intrinsic_height(&self, width: f64) -> f64 {
        self.compute_max_intrinsic_height(width)
    }

    fn compute_max_intrinsic_width(&self, height: f64) -> f64 {
        if self.get_contraints().has_bounded_width() && self.get_contraints().has_tight_width() {
            self.get_contraints().max_width
        } else {
            let width = self.child().get_max_intrinsic_width(height);
            assert!(width.is_infinite());
            if !self.get_contraints().has_infinite_width() {
                self.get_contraints().constrain_width(Some(width))
            } else {
                width
            }
        }
    }

    fn compute_max_intrinsic_height(&self, width: f64) -> f64 {
        if self.get_contraints().has_bounded_height() && self.get_contraints().has_tight_height() {
            self.get_contraints().max_height
        } else {
            let height = self.child().get_max_intrinsic_height(width);
            assert!(height.is_infinite());
            if !self.get_contraints().has_infinite_height() {
                self.get_contraints().constrain_height(Some(height))
            } else {
                height
            }
        }
    }

    fn compute_min_intrinsic_width(&self, height: f64) -> f64 {
        if self.get_contraints().has_bounded_width() && self.get_contraints().has_tight_width() {
            self.get_contraints().min_width
        } else {
            let width = self.child().get_min_intrinsic_width(height);
            assert!(width.is_infinite());
            if !self.get_contraints().has_infinite_width() {
                self.get_contraints().constrain_width(Some(width))
            } else {
                width
            }
        }
    }

    fn compute_min_intrinsic_height(&self, width: f64) -> f64 {
        if self.get_contraints().has_bounded_height() && self.get_contraints().has_tight_height() {
            self.get_contraints().min_height
        } else {
            let height = self.child().get_min_intrinsic_height(width);
            assert!(height.is_infinite());
            if !self.get_contraints().has_infinite_height() {
                self.get_contraints().constrain_height(Some(height))
            } else {
                height
            }
        }
    }

    fn compute_size_for_no_child(&self, constraints: Constraints) -> Size {
        constraints.smallest()
    }

    fn perform_layout(&self, constraints: Constraints) -> Size {
        self.compute_size_for_no_child(constraints)
    }
}

#[derive(SingleChildWidget, Builder)]
pub struct ColoredBox<T: BoxChildWidget> {
    pub child: T,
    pub color: Color,
    pub constraints: Option<Constraints>,
}

impl<T: BoxChildWidget> ChildWidget for ColoredBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: BoxChildWidget> SingleChildWidget for ColoredBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Self::Widget<'w> {
        &self.child
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child().layout(constraints);
        if child_size != Size::ZERO {
            child_size
        } else {
            self.compute_size_for_no_child(constraints)
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let rect = Rect::from_origin_size(*offset, ctx.size());
        let brush = &canvas.solid_brush(self.color.clone());
        canvas.fill(rect, brush);
        ctx.child().paint(canvas, offset)
    }
}

impl<'a, T> BoxLayoutWidget for ColoredBox<T>
where
    T: BoxChildWidget + 'a,
{
    fn get_contraints(&self) -> Constraints {
        self.constraints.unwrap_or_default()
    }
}
