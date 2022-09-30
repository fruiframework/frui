use crate::app::tree::WidgetNodeRef;

pub mod build_ctx;
pub mod render_ctx;

#[repr(transparent)]
pub struct Context {
    pub(crate) node: WidgetNodeRef,
}
