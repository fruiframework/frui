#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(type_alias_impl_trait)]
//
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod api;
pub mod app;

pub mod prelude {
    pub use super::{
        api::{
            contexts::build_cx::{
                BuildCx, InheritedState, InheritedStateRef, InheritedStateRefMut, WidgetState,
            },
            implementers::{inherited::InheritedWidget, view::ViewWidget},
            impls::BoxedWidget,
            pointer_events::*,
            Widget,
        },
        app::runner::native::run_app,
    };

    pub use crate::render::{Offset, Size};

    pub use druid_shell::piet::{Color, FontWeight};

    // Macros exports.
    pub use frui_macros::{Builder, InheritedWidget, RenderWidget, ViewWidget};

    // Core widgets exports.
    pub use super::api::local_key::LocalKey;
}

pub mod render {
    pub use crate::api::implementers::render::RenderWidget;

    pub use crate::api::contexts::render::*;
    pub use crate::app::runner::Canvas;
    pub use crate::app::TEXT_FACTORY;

    pub use druid_shell::{kurbo, piet};
    pub use druid_shell::{
        kurbo::{Affine, Point, Rect as DruidRect, Vec2},
        piet::{Color, RenderContext},
    };
}

#[doc(hidden)]
pub mod macro_exports {

    pub use crate::{
        api::{
            contexts::{
                render::{LayoutCxOS, PaintCxOS},
                RawBuildCx,
            },
            implementers::{
                InheritedWidgetOS, RawWidget, RenderWidgetOS, ViewWidgetOS, WidgetDerive,
            },
            structural_eq::{StructuralEq, StructuralEqImpl},
            WidgetPtr,
        },
        prelude::Widget,
        render::{Canvas, Constraints, Offset, Size},
    };
}

#[doc(hidden)]
pub use druid_shell;
