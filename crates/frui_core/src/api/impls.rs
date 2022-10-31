use std::ops::Deref;

use crate::{
    api::{implementors::leaf::LeafWidget, Widget},
    prelude::*,
};

use super::implementors::{LeafWidgetOS, RawWidgetOS, WidgetDerive};

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
    fn unique_type(&self) -> std::any::TypeId {
        <T as Widget>::unique_type(*self)
    }

    fn as_os(&self) -> &dyn RawWidgetOS {
        T::as_os(*self)
    }
}

impl<T: Widget> Widget for &mut T {
    fn unique_type(&self) -> std::any::TypeId {
        <T as Widget>::unique_type(*self)
    }

    fn as_os(&self) -> &dyn RawWidgetOS {
        T::as_os(*self)
    }
}

impl<'a> Widget for &'a dyn Widget {
    fn unique_type(&self) -> std::any::TypeId {
        Widget::unique_type(self.deref())
    }

    fn as_os(&self) -> &dyn RawWidgetOS {
        self.deref().as_os()
    }
}

impl<'a> Widget for Box<dyn Widget + 'a> {
    fn unique_type(&self) -> std::any::TypeId {
        Widget::unique_type(self.deref())
    }

    fn as_os(&self) -> &dyn RawWidgetOS {
        self.deref().as_os()
    }
}

impl<T: Widget> Widget for Box<T> {
    fn unique_type(&self) -> std::any::TypeId {
        Widget::unique_type(self.deref())
    }

    fn as_os(&self) -> &dyn RawWidgetOS {
        self.deref().as_os()
    }
}

impl_widget_os_deref!(impl<T: Widget> RawWidgetOS for &T);
impl_widget_os_deref!(impl<T: Widget> RawWidgetOS for &mut T);
impl_widget_os_deref!(impl<T: Widget> RawWidgetOS for Box<T>);
impl_widget_os_deref!(impl<'a> RawWidgetOS for &'a dyn Widget);
impl_widget_os_deref!(impl<'a> RawWidgetOS for Box<dyn Widget + 'a>);

//
// Unit type implementation
//

#[doc(hidden)]
pub enum Unique {}

impl Widget for () {
    fn unique_type(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Unique>()
    }

    fn as_os(&self) -> &dyn RawWidgetOS {
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

impl RawWidgetOS for () {
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
