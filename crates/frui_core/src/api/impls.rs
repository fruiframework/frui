use std::ops::Deref;

use crate::{
    api::{implementors::leaf::LeafWidget, Widget, WidgetKind},
    prelude::*,
};

use super::implementors::WidgetDerive;

pub trait BoxedWidget: Widget + Sized {
    /// Convenience method used to type erase and box a widget.
    fn boxed<'a>(self) -> Box<dyn Widget + 'a>
    where
        Self: 'a;
}

impl<T: Widget> BoxedWidget for T {
    fn boxed<'a>(self) -> Box<dyn Widget + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}

//
// Implementations
//

impl<T: Widget> Widget for &T {
    fn unique_type(&self) -> std::any::TypeId {
        T::unique_type(&self)
    }

    fn kind(&self) -> WidgetKind {
        T::kind(&self)
    }
}

impl<T: Widget> Widget for &mut T {
    fn unique_type(&self) -> std::any::TypeId {
        T::unique_type(&self)
    }

    fn kind(&self) -> WidgetKind {
        T::kind(&self)
    }
}

impl<'a> Widget for &'a dyn Widget {
    fn unique_type(&self) -> std::any::TypeId {
        self.deref().unique_type()
    }

    fn kind(&self) -> WidgetKind {
        self.deref().kind()
    }
}

impl<'a> Widget for Box<dyn Widget + 'a> {
    fn unique_type(&self) -> std::any::TypeId {
        self.deref().unique_type()
    }

    fn kind(&self) -> WidgetKind {
        self.deref().kind()
    }
}

impl<T: Widget> Widget for Box<T> {
    fn unique_type(&self) -> std::any::TypeId {
        self.deref().unique_type()
    }

    fn kind(&self) -> WidgetKind {
        self.deref().kind()
    }
}

impl Widget for () {
    fn unique_type(&self) -> std::any::TypeId {
        std::any::TypeId::of::<()>()
    }

    fn kind(&self) -> WidgetKind {
        WidgetKind::Leaf(self)
    }
}

impl WidgetDerive for () {
    type Widget<'a> = ();

    type UniqueTypeId = ();
}

impl LeafWidget for () {
    fn layout(&self, _: RenderContext<Self>, _: Constraints) -> Size {
        Size::default()
    }

    fn paint(&self, _: RenderContext<Self>, _: &mut PaintContext, _: &Offset) {}
}
