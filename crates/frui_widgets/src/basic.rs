use std::ops::Deref;

use frui::prelude::*;
use frui::render::*;

use crate::{Alignment, BoxLayoutData, Directional, EdgeInsets, TextDirection};

pub trait ChildParentDataProvider<T: RenderWidget> {
    fn ensure_parent_data<F, P>(&self, cx: &LayoutCx<T>, default: F)
    where
        F: Fn() -> P,
        P: 'static;
}

impl<T: RenderWidget> ChildParentDataProvider<T> for T {
    fn ensure_parent_data<F, P>(&self, cx: &LayoutCx<T>, default: F)
    where
        F: Fn() -> P,
        P: 'static,
    {
        for child in cx.children() {
            if child.try_parent_data::<P>().is_none() {
                let data = default();
                child.set_parent_data(data);
            }
        }
    }
}

#[derive(InheritedWidget, Builder)]
pub struct Directionality<T: Widget> {
    pub direction: TextDirection,
    pub child: T,
}

impl<T: Widget> WidgetState for Directionality<T> {
    type State = TextDirection;

    fn create_state(&self) -> Self::State {
        self.direction
    }
}

impl<T: Widget> InheritedWidget for Directionality<T> {
    fn build<'w>(&'w self) -> Self::Widget<'w> {
        &self.child
    }
}

impl Directionality<()> {
    pub fn of<T>(cx: &LayoutCx<T>) -> Option<TextDirection> {
        let state = cx.depend_on_inherited_widget::<Self>();
        state.map(|s| *s.as_ref().deref())
    }

    pub fn of_or_default<T>(cx: &LayoutCx<T>) -> TextDirection {
        Self::of(cx).unwrap_or_default()
    }

    pub fn unwrap_or_default<T>(
        text_direction: Option<TextDirection>,
        cx: &LayoutCx<T>,
    ) -> TextDirection {
        text_direction.unwrap_or_else(|| Self::of_or_default(cx))
    }
}

#[derive(RenderWidget, Builder)]
pub struct Align<T: Widget, A: Directional<Output = Alignment>> {
    pub child: T,
    pub alignment: A,
    pub widgh_factor: Option<f64>,
    pub height_factor: Option<f64>,
    pub text_direction: Option<TextDirection>,
}

impl Align<(), Alignment> {
    pub fn builder() -> Self {
        Self {
            child: (),
            alignment: Alignment::default(),
            widgh_factor: None,
            height_factor: None,
            text_direction: None,
        }
    }
}

impl<T, A> RenderWidget for Align<T, A>
where
    T: Widget,
    A: Directional<Output = Alignment>,
{
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size {
        self.ensure_parent_data(cx, || BoxLayoutData::default());
        let text_direction = self
            .text_direction
            .unwrap_or_else(|| Directionality::of_or_default(cx));
        let alignment = self.alignment.resolve(&text_direction);
        let shrink_wrap_width =
            self.widgh_factor.is_some() || constraints.max_width == f64::INFINITY;
        let shrink_wrap_height =
            self.height_factor.is_some() || constraints.max_height == f64::INFINITY;

        let child = cx.child(0);
        let child_size = child.layout(constraints.loosen());
        let size = constraints.constrain(Size::new(
            if shrink_wrap_width {
                child_size.width * self.widgh_factor.unwrap_or(1.0)
            } else {
                f64::INFINITY
            },
            if shrink_wrap_height {
                child_size.height * self.height_factor.unwrap_or(1.0)
            } else {
                f64::INFINITY
            },
        ));
        let mut child_parent_data = child.try_parent_data_mut::<BoxLayoutData>().unwrap();
        child_parent_data.offset = alignment.along(size - child_size);
        size
    }

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let child_offset = cx
            .child(0)
            .try_parent_data::<BoxLayoutData>()
            .unwrap()
            .offset;
        cx.child(0).paint(canvas, &(child_offset + *offset))
    }
}

#[derive(RenderWidget, Builder)]
pub struct Padding<T: Widget, P: Directional<Output = EdgeInsets>> {
    pub child: T,
    pub padding: P,
}

impl Padding<(), EdgeInsets> {
    pub fn builder() -> Self {
        Self {
            child: (),
            padding: EdgeInsets::ZERO,
        }
    }
}

impl<T, P> RenderWidget for Padding<T, P>
where
    T: Widget,
    P: Directional<Output = EdgeInsets>,
{
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size {
        self.ensure_parent_data(cx, BoxLayoutData::default);
        let text_direction = Directionality::of_or_default(cx);
        let padding = self.padding.resolve(&text_direction);
        let child_constraints = padding.deflate_constraints(&constraints);
        let child_size = cx.child(0).layout(child_constraints);
        let child = cx.child(0);
        let mut child_parent_data = child.try_parent_data_mut::<BoxLayoutData>().unwrap();
        child_parent_data.offset = padding.top_left();
        constraints.constrain(child_size + padding.collapsed_size())
    }

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset) {
        let child_offset = cx
            .child(0)
            .try_parent_data::<BoxLayoutData>()
            .unwrap()
            .offset;
        cx.child(0).paint(canvas, &(*offset + child_offset))
    }
}
