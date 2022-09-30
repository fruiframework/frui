#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(type_alias_impl_trait)]
//
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod api;
pub mod app;

pub mod prelude {
    pub use frui_macros::{
        InheritedWidget, LeafWidget, MultiChildWidget, SingleChildWidget, ViewWidget,
    };

    pub use super::{
        api::{
            contexts::{
                build_ctx::{
                    BuildContext, InheritedState, InheritedStateRef, InheritedStateRefMut,
                    WidgetState,
                },
                render_ctx::{ChildContext, Constraints, Offset, RenderContext, RenderState, Size},
            },
            implementors::{
                inherited::InheritedWidget, leaf::LeafWidget, multi::MultiChildWidget,
                single::SingleChildWidget, view::ViewWidget,
            },
            impls::BoxedWidget,
            Widget, WidgetKind,
        },
        app::runner::{native::run_app, PaintContext},
    };

    pub use druid_shell::{
        kurbo::*,
        piet::{
            Brush, Color, FontFamily, FontStyle, FontWeight, RenderContext as PietRenderContext,
        },
        KeyEvent, MouseButton,
    };

    // Widget exports.
    pub use super::api::local_key::LocalKey;
}

#[doc(hidden)]
pub mod macro_exports {
    pub use crate::api::implementors::WidgetDerive;
    pub use crate::api::widget_eq::cheap_eq::CheapEq;
}

#[doc(hidden)]
pub use druid_shell;
