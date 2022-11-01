use std::ops::Deref;

use crate::{
    api::{implementers::leaf::LeafWidget, Widget},
    prelude::*,
};

use super::implementers::{LeafWidgetOS, RawWidget, WidgetDerive};

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

impl LeafWidget for () {
    fn layout(&self, _: RenderContext<Self>, _: Constraints) -> Size {
        Size::default()
    }

    fn paint(&self, _: RenderContext<Self>, _: &mut PaintContext, _: &Offset) {}
}

impl RawWidget for () {
    fn build<'w>(&'w self, ctx: &'w super::contexts::Context) -> Vec<super::WidgetPtr<'w>> {
        <Self as LeafWidgetOS>::build(self, ctx)
    }

    fn layout<'w>(
        &self,
        ctx: &'w mut super::contexts::render_ctx::AnyRenderContext,
        constraints: Constraints,
    ) -> Size {
        <Self as LeafWidgetOS>::layout(self, ctx, constraints)
    }

    fn paint<'w>(
        &'w self,
        ctx: &'w mut super::contexts::render_ctx::AnyRenderContext,
        canvas: &mut PaintContext,
        offset: &Offset,
    ) {
        <Self as LeafWidgetOS>::paint(self, ctx, canvas, offset)
    }

    fn inherited_key(&self) -> Option<std::any::TypeId> {
        <Self as LeafWidgetOS>::inherited_key(self)
    }
}

macro_rules! impl_widget_os_deref_ {
    ($($impl:tt)*) => {
        $($impl)* {
            fn build<'w>(&'w self, ctx: &'w super::contexts::Context) -> Vec<super::WidgetPtr<'w>> {
                self.deref().build(ctx)
            }

            fn layout<'w>(
                &self,
                ctx: &'w mut super::contexts::render_ctx::AnyRenderContext,
                constraints: Constraints,
            ) -> Size {
                self.deref().layout(ctx, constraints)
            }

            fn paint<'w>(
                &'w self,
                ctx: &'w mut super::contexts::render_ctx::AnyRenderContext,
                canvas: &mut PaintContext,
                offset: &Offset,
            ) {
                self.deref().paint(ctx, canvas, offset)
            }


            fn inherited_key(&self) -> Option<std::any::TypeId> {
                self.deref().inherited_key()
            }
        }

    };
}

pub(self) use impl_widget_os_deref_ as impl_widget_os_deref;
