use crate::app::tree::WidgetNodeRef;

pub mod build_ctx;
pub mod render;

/// This context allows to access [`_BuildCtx`] from anywhere in the build
/// method, while making sure that given reference is valid for reads for the
/// lifetime of that widget.
///
/// Usually we would be able to just pass a simple structure which implements
/// clone (e.g. Rc<Node>), but doing it that way makes it impossible to access
/// context from multiple closures at once (since that context argument has to
/// move). To fix this you could clone context for every single closure, but
/// that gets tedious very fast.
///
/// Instead we borrow that context ourselves, allowing consumers to share a
/// single `ctx` between every closure that appears in the build method (of
/// which lifetime is <= widget node).
///
/// [`_BuildCtx`]: build_ctx::_BuildCtx
#[repr(transparent)]
pub struct RawBuildCtx {
    pub(crate) node: WidgetNodeRef,
}
