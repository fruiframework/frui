use std::ops::Deref;

use crate::render::*;
use crate::{api::Widget, macro_exports::RenderWidgetOS, prelude::*};

use super::implementers::{RawWidget, WidgetDerive};

pub trait BoxedWidget: Widget + Sized {
    /// Convenience method used to type erase and box a widget.
    fn boxed<'a>(self) -> Box<dyn Widget + 'a>
    where
        Self: 'a;
}

impl<T: Widget> BoxedWidget for T {
    default fn boxed<'a>(self) -> Box<dyn Widget + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}

impl<T: Widget> BoxedWidget for Box<T> {
    fn boxed<'a>(self) -> Box<dyn Widget + 'a>
    where
        Self: 'a,
    {
        // Avoid double boxing.
        self
    }
}

//
// Implementations
//

impl<T: Widget> Widget for &T {
    fn as_raw(&self) -> &dyn RawWidget {
        T::as_raw(*self)
    }
}

impl<T: Widget> Widget for &mut T {
    fn as_raw(&self) -> &dyn RawWidget {
        T::as_raw(*self)
    }
}

impl<'a> Widget for &'a dyn Widget {
    fn as_raw(&self) -> &dyn RawWidget {
        self.deref().as_raw()
    }
}

impl<'a> Widget for Box<dyn Widget + 'a> {
    fn as_raw(&self) -> &dyn RawWidget {
        self.deref().as_raw()
    }
}

impl<T: Widget> Widget for Box<T> {
    fn as_raw(&self) -> &dyn RawWidget {
        self.deref().as_raw()
    }
}

impl_widget_os_deref!(impl<T: Widget> RawWidget for &T);
impl_widget_os_deref!(impl<T: Widget> RawWidget for &mut T);
impl_widget_os_deref!(impl<T: Widget> RawWidget for Box<T>);
impl_widget_os_deref!(impl<'a> RawWidget for &'a dyn Widget);
impl_widget_os_deref!(impl<'a> RawWidget for Box<dyn Widget + 'a>);

//
// Unit type implementation
//

#[doc(hidden)]
pub enum Unique {}

impl Widget for () {
    fn as_raw(&self) -> &dyn RawWidget {
        self
    }
}

impl WidgetDerive for () {
    type Widget<'a> = ();

    type UniqueTypeId = Unique;
}

impl RenderWidget for () {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![]
    }

    fn layout(&self, _: &LayoutCx<Self>, c: Constraints) -> Size {
        c.smallest()
    }

    fn paint(&self, _: &mut PaintCx<Self>, _: &mut Canvas, _: &Offset) {}
}

impl RawWidget for () {
    fn build<'w>(&'w self, cx: &'w super::contexts::RawBuildCx) -> Vec<super::WidgetPtr<'w>> {
        <Self as RenderWidgetOS>::build(self, cx)
    }

    fn layout<'w>(&self, cx: LayoutCxOS, constraints: Constraints) -> Size {
        <Self as RenderWidgetOS>::layout(self, cx, constraints)
    }

    fn paint<'w>(&'w self, cx: PaintCxOS, canvas: &mut Canvas, offset: &Offset) {
        <Self as RenderWidgetOS>::paint(self, cx, canvas, offset)
    }

    fn inherited_key(&self) -> Option<std::any::TypeId> {
        <Self as RenderWidgetOS>::inherited_key(self)
    }
}

macro_rules! impl_widget_os_deref_ {
    ($($impl:tt)*) => {
        $($impl)* {
            fn build<'w>(&'w self, cx: &'w super::contexts::RawBuildCx) -> Vec<super::WidgetPtr<'w>> {
                self.deref().build(cx)
            }

            fn layout<'w>(
                &self,
                cx: LayoutCxOS,
                constraints: Constraints,
            ) -> Size {
                self.deref().layout(cx, constraints)
            }

            fn paint<'w>(
                &'w self,
                cx: PaintCxOS,
                canvas: &mut Canvas,
                offset: &Offset,
            ) {
                self.deref().paint(cx, canvas, offset)
            }


            fn inherited_key(&self) -> Option<std::any::TypeId> {
                self.deref().inherited_key()
            }
        }

    };
}

pub(self) use impl_widget_os_deref_ as impl_widget_os_deref;
