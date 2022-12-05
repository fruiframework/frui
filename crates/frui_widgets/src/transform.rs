use frui::prelude::*;
use frui::render::*;

#[derive(RenderWidget)]
pub struct Transform<W: Widget>(pub Affine, pub W);

impl<W: Widget> RenderWidget for Transform<W> {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.1]
    }

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size {
        cx.child(0).layout(constraints)
    }

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let r = canvas.with_save(|cv| {
            cv.transform(self.0);
            cx.child(0).paint(cv, offset);

            Ok(())
        });

        r.unwrap();
    }
}

impl<W: Widget> HitTest for Transform<W> {
    fn hit_test<'a>(&'a self, cx: &'a mut HitTestCx<Self>, point: Point) -> bool {
        if cx.layout_box().contains(point) {
            for mut child in cx.children() {
                if child.hit_test_with_transform(point, self.0.inverse()) {
                    return true;
                }
            }
        }

        false
    }
}
