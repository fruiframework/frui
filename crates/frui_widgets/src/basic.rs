use frui::prelude::*;

use crate::{BoxChildWidget, BoxLayoutWidget, ChildWidget, BoxLayoutData};


#[derive(RenderWidget, Default, Builder)]
pub struct ConstrainedBox<T: BoxChildWidget> {
    pub child: T,
    pub constraints: Constraints,
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

impl<T: BoxChildWidget> RenderWidget for ConstrainedBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints.enforce(constraints));
        if child_size != Size::ZERO {
            child_size
        } else {
            self.constraints.enforce(constraints).constrain(Size::ZERO)
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}

impl<T> BoxLayoutWidget for ConstrainedBox<T>
where
    T: BoxChildWidget
{
    fn get_constraints(&self) -> Constraints {
        self.constraints
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
        if self.get_constraints().has_bounded_width() && self.get_constraints().has_tight_width() {
            self.get_constraints().max_width
        } else {
            let width = self.child().get_max_intrinsic_width(height);
            assert!(width.is_infinite());
            if !self.get_constraints().has_infinite_width() {
                self.get_constraints().constrain_width(Some(width))
            } else {
                width
            }
        }
    }

    fn compute_max_intrinsic_height(&self, width: f64) -> f64 {
        if self.get_constraints().has_bounded_height() && self.get_constraints().has_tight_height() {
            self.get_constraints().max_height
        } else {
            let height = self.child().get_max_intrinsic_height(width);
            assert!(height.is_infinite());
            if !self.get_constraints().has_infinite_height() {
                self.get_constraints().constrain_height(Some(height))
            } else {
                height
            }
        }
    }

    fn compute_min_intrinsic_width(&self, height: f64) -> f64 {
        if self.get_constraints().has_bounded_width() && self.get_constraints().has_tight_width() {
            self.get_constraints().min_width
        } else {
            let width = self.child().get_min_intrinsic_width(height);
            assert!(width.is_infinite());
            if !self.get_constraints().has_infinite_width() {
                self.get_constraints().constrain_width(Some(width))
            } else {
                width
            }
        }
    }

    fn compute_min_intrinsic_height(&self, width: f64) -> f64 {
        if self.get_constraints().has_bounded_height() && self.get_constraints().has_tight_height() {
            self.get_constraints().min_height
        } else {
            let height = self.child().get_min_intrinsic_height(width);
            assert!(height.is_infinite());
            if !self.get_constraints().has_infinite_height() {
                self.get_constraints().constrain_height(Some(height))
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

#[derive(RenderWidget, Builder)]
pub struct UnconstrainedBox<T: BoxChildWidget> {
    pub child: T
}

impl<'a, T: BoxChildWidget> ParentData for UnconstrainedBox<T> {
    type Data = BoxLayoutData;

    fn create_data(&self) -> Self::Data {
        BoxLayoutData::default()
    }
}

impl<T: BoxChildWidget> ChildWidget for UnconstrainedBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: BoxChildWidget> RenderWidget for UnconstrainedBox<T> {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints.loosen());
        if child_size != Size::ZERO {
            child_size
        } else {
            constraints.biggest()
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }
}

#[derive(RenderWidget, Builder)]
pub struct ColoredBox<T: BoxChildWidget> {
    pub child: T,
    pub color: Color,
}

impl<T: BoxChildWidget> ChildWidget for ColoredBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: BoxChildWidget> RenderWidget for ColoredBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let child_size = ctx.child(0).layout(constraints.loosen());
        if child_size != Size::ZERO {
            child_size
        } else {
            constraints.smallest()
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        let rect = Rect::from_origin_size(*offset, ctx.size());
        let brush = &canvas.solid_brush(self.color.clone());
        canvas.fill(rect, brush);
        ctx.child(0).paint(canvas, offset)
    }
}

impl <T: BoxChildWidget> BoxLayoutWidget for ColoredBox<T> {
    fn get_constraints(&self) -> Constraints {
        Constraints::default()
    }
}