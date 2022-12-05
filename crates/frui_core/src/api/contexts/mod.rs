use crate::app::tree::Node;

pub mod build_cx;
pub mod render;

/// This context allows to access [`_BuildCx`] from anywhere in the build
/// method, while making sure that given reference is valid for reads for the
/// lifetime of that widget.
///
/// Usually we would be able to just pass a simple structure which implements
/// clone (e.g. `Rc<Node>`), but doing it that way makes it impossible to access
/// context from multiple closures at once (since that context argument has to
/// move). To fix this you could clone context for every single closure, but
/// that gets tedious very fast.
///
/// Instead we borrow that context ourselves, allowing consumers to share a
/// single `cx` between every closure that appears in the build method (of which
/// lifetime is <= widget node).
///
/// [`_BuildCx`]: build_cx::_BuildCx
#[repr(transparent)]
pub struct RawBuildCx {
    pub(crate) _node: Node,
}
