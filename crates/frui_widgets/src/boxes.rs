use frui::prelude::*;

pub trait ChildWidget {
    type ChildType<'a>: Widget where Self: 'a;

    fn child<'w>(&'w self) -> &Self::ChildType<'w>;
}

pub trait BoxLayoutWidget {
    fn get_constraints(&self) -> Constraints {
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

    fn compute_max_intrinsic_width(&self, _height: f64) -> f64 {
        0.0
    }
    fn compute_max_intrinsic_height(&self, _width: f64) -> f64 {
        0.0
    }
    fn compute_min_intrinsic_width(&self, _height: f64) -> f64 {
        0.0
    }
    fn compute_min_intrinsic_height(&self, _width: f64) -> f64 {
        0.0
    }

    fn compute_size_for_no_child(&self, constraints: Constraints) -> Size {
        constraints.smallest()
    }

    fn perform_layout(&self, constraints: Constraints) -> Size {
        self.compute_size_for_no_child(constraints)
    }
}

pub trait BoxChildWidget: BoxLayoutWidget + Widget {}

impl <T: BoxLayoutWidget + Widget> BoxChildWidget for T {}

pub trait BoxSingleChildWidget: RenderWidget + BoxLayoutWidget {
    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        println!("Default BoxSingleChildWidget's layout");
        let child_size = ctx.child(0).layout(constraints);
        if child_size != Size::ZERO {
            child_size
        } else {
            self.compute_size_for_no_child(constraints)
        }
    }
}

impl<T: RenderWidget + BoxLayoutWidget> BoxSingleChildWidget for T {}
