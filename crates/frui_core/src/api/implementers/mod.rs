use std::any::TypeId;

use frui_macros::copy_trait_as;

use crate::prelude::{Constraints, Offset, PaintContext, Size};

use super::{
    any_ext::AnyExt,
    contexts::{
        build_ctx::widget_state::WidgetStateOS,
        render_ctx::{AnyRenderContext, ParentDataOS, RenderStateOS},
        Context,
    },
    events::WidgetEventOS,
    local_key::WidgetLocalKey,
    structural_eq::StructuralEqOS,
    WidgetDebug, WidgetPtr, WidgetUniqueType,
};

pub(crate) mod inherited;
pub(crate) mod leaf;
pub(crate) mod multi;
pub mod render;
pub(crate) mod single;
pub(crate) mod view;

/// This trait can be implemented using `#[derive(WidgetKind)]`.
pub trait WidgetDerive {
    /// Specifies the exact type of child/children of a given widget. It is
    /// automatically being inferred from the context using the TAIT feature.
    type Widget<'a>: super::Widget
    where
        Self: 'a;

    /// Implementation should make sure [`TypeId`] of this type is unique for
    /// given structure definition, even if that structure contains generic
    /// parameters. This is used to preserve state between generic widgets.
    #[doc(hidden)]
    type UniqueTypeId: 'static;
}

/// Object safe implementation for each widget kind. For example, `ViewWidget`
/// has its matching `ViewWidgetOS`.
///
/// Those implemetations are then routed through `RawWidget` using the derive
/// macro and accessed by framework through `&dyn RawWidget`.
///
/// ## `RawWidget`
///
/// `RawWidget` is the base widget implementation containing all the necessary
/// methods like `paint`, `layout`, `build`, etc. All widget implementations are
/// routed through this trait (by the derive macro) and are accessed by
/// framework through `&dyn RawWidget`.
/// 
/// ## Render widget implementers
/// 
/// `LeafWidget`, `SingleChildWidget`, and `MultiChildWidget` are all "widget
/// implementers" that allow for implementing render methods, like `layout` and
/// `paint`. Initially I made the distinction between the amount of children
/// each implementer can have to make methods like `child()` and `children()` of
/// given `ctx` infallible. 
/// 
/// However, as of now that distinction seems to only introduce unnecessary
/// complexity, so I added `RenderWidget` implementer which combines
/// `LeafWidget`, `SingleChildWidget`, and `MultiChildWidget` implementers into
/// one.
#[doc(hidden)]
#[rustfmt::skip]
#[copy_trait_as(
    RawWidget,
    ViewWidgetOS, InheritedWidgetOS,
    LeafWidgetOS, SingleChildWidgetOS, MultiChildWidgetOS, RenderWidgetOS
)]
pub trait OS:
    WidgetStateOS
    + RenderStateOS
    + ParentDataOS
    + WidgetEventOS
    + WidgetLocalKey
    + WidgetUniqueType
    + WidgetDebug
    + StructuralEqOS
    + AnyExt
{
    fn build<'w>(&'w self, ctx: &'w Context) -> Vec<WidgetPtr<'w>>;

    fn layout<'w>(&self, ctx: &'w mut AnyRenderContext, constraints: Constraints) -> Size;

    fn paint<'w>(
        &'w self,
        ctx: &'w mut AnyRenderContext,
        canvas: &mut PaintContext,
        offset: &Offset,
    );

    fn inherited_key(&self) -> Option<TypeId> {
        None
    }
}
