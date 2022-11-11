use frui::prelude::*;
use frui::render::*;

#[derive(RenderWidget)]
pub struct Transform<W: Widget>(pub Affine, pub W);

impl<W: Widget> RenderWidget for Transform<W> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.1]
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        ctx.child(0).layout(constraints)
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let r = canvas.with_save(|cv| {
            cv.transform(self.0);
            ctx.child(0).paint(cv, offset);

            Ok(())
        });

        r.unwrap();
    }
}

impl<W: Widget> HitTest for Transform<W> {
    fn hit_test<'a>(&'a self, ctx: &'a mut HitTestCtx<Self>, point: Point) -> bool {
        if ctx.layout_box().contains(point) {
            for mut child in ctx.children() {
                if child.hit_test_with_transform(point, self.0.inverse()) {
                    return true;
                }
            }
        }

        false
    }
}
