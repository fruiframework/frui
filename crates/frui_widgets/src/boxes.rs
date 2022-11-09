use frui::prelude::*;

use crate::BoxLayoutData;

pub trait ChildWidget {
    type ChildType<'a>: Widget
    where
        Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w>;
}

#[derive(RenderWidget, Default, Builder)]
pub struct ConstrainedBox<T: RenderWidget + Widget> {
    pub child: T,
    pub constraints: Constraints,
}

impl<'a, T: RenderWidget + Widget> ParentData for ConstrainedBox<T> {
    type Data = BoxLayoutData;

    fn create_data(&self) -> Self::Data {
        BoxLayoutData::default()
    }
}

impl<T: RenderWidget + Widget> ChildWidget for ConstrainedBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: RenderWidget + Widget> RenderWidget for ConstrainedBox<T> {
    fn build<'w>(&'w self, _ctx: BuildContext<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let constraints = self.constraints.enforce(constraints);
        let child_size = ctx.child(0).layout(constraints);

        if child_size != Size::ZERO {
            child_size
        } else {
            self.constraints.enforce(constraints).constrain(Size::ZERO)
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        ctx.child(0).paint(canvas, offset)
    }

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
                self.get_constraints().constrain_width(width)
            } else {
                width
            }
        }
    }

    fn compute_max_intrinsic_height(&self, width: f64) -> f64 {
        if self.get_constraints().has_bounded_height() && self.get_constraints().has_tight_height()
        {
            self.get_constraints().max_height
        } else {
            let height = self.child().get_max_intrinsic_height(width);
            assert!(height.is_infinite());
            if !self.get_constraints().has_infinite_height() {
                self.get_constraints().constrain_height(height)
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
                self.get_constraints().constrain_width(width)
            } else {
                width
            }
        }
    }

    fn compute_min_intrinsic_height(&self, width: f64) -> f64 {
        if self.get_constraints().has_bounded_height() && self.get_constraints().has_tight_height()
        {
            self.get_constraints().min_height
        } else {
            let height = self.child().get_min_intrinsic_height(width);
            assert!(height.is_infinite());
            if !self.get_constraints().has_infinite_height() {
                self.get_constraints().constrain_height(height)
            } else {
                height
            }
        }
    }
}

#[derive(RenderWidget, Builder)]
pub struct UnconstrainedBox<T: RenderWidget + Widget> {
    pub child: T,
}

impl<'a, T: RenderWidget + Widget> ParentData for UnconstrainedBox<T> {
    type Data = BoxLayoutData;

    fn create_data(&self) -> Self::Data {
        BoxLayoutData::default()
    }
}

impl<T: RenderWidget + Widget> ChildWidget for UnconstrainedBox<T> {
    type ChildType<'a> = T where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w> {
        &self.child
    }
}

impl<T: RenderWidget + Widget> RenderWidget for UnconstrainedBox<T> {
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
pub struct SizedBox;

impl SizedBox {
    pub fn from_size<T: RenderWidget + Widget>(child: T, size: Size) -> impl RenderWidget + Widget {
        ConstrainedBox {
            child,
            constraints: Constraints::new_tight_for(Some(size.width), Some(size.height)),
        }
    }

    pub fn new<T: RenderWidget + Widget>(
        child: T,
        width: Option<f64>,
        height: Option<f64>,
    ) -> impl RenderWidget + Widget {
        ConstrainedBox {
            child,
            constraints: Constraints::new_tight_for(width, height),
        }
    }

    pub fn square<T: RenderWidget + Widget>(child: T, size: f64) -> impl RenderWidget + Widget {
        Self::from_size(child, Size::new(size, size))
    }

    pub fn shrink<T: RenderWidget + Widget>(child: T) -> impl RenderWidget + Widget {
        ConstrainedBox {
            child,
            constraints: Constraints::new_tight_for(None, None),
        }
    }
}
