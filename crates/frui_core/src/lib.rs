#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(type_alias_impl_trait)]
//
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod api;
pub mod app;

pub mod prelude {
    pub use frui_macros::{InheritedWidget, RenderWidget, ViewWidget};

    pub use super::{
        api::{
            contexts::{
                build_ctx::{
                    BuildContext, InheritedState, InheritedStateRef, InheritedStateRefMut,
                    WidgetState,
                },
                render_ctx::{
                    ext::RenderExt,
                    paint_ctx::{PaintContext, PaintContextOS},
                    Constraints, LayoutCtxChildIter, Offset, ParentData, RenderContext,
                    RenderContextOS, RenderState, Size,
                },
            },
            implementers::{inherited::InheritedWidget, render::RenderWidget, view::ViewWidget},
            impls::BoxedWidget,
            pointer_events::*,
            Widget,
        },
        app::runner::{native::run_app, Canvas},
    };

    pub use druid_shell::{
        kurbo::*,
        piet::{
            Brush, Color, FontFamily, FontStyle, FontWeight, RenderContext as PietRenderContext,
        },
        KeyEvent, MouseButton,
    };

    pub use frui_macros::Builder;

    // Widget exports.
    pub use super::api::local_key::LocalKey;
}
#[doc(hidden)]
pub mod macro_exports {

    pub use crate::{
        api::{
            contexts::render_ctx::paint_ctx::PaintContextOS,
            contexts::{render_ctx::RenderContextOS, Context},
            implementers::{
                InheritedWidgetOS, RawWidget, RenderWidgetOS, ViewWidgetOS, WidgetDerive,
            },
            structural_eq::{StructuralEq, StructuralEqImpl},
            WidgetPtr,
        },
        prelude::{Canvas, Constraints, Offset, Size, Widget},
    };
}

#[doc(hidden)]
pub use druid_shell;
