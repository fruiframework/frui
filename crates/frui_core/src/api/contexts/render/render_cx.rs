use std::marker::PhantomData;

use druid_shell::IdleToken;

use super::{
    ext::{RenderExt, RenderOSExt},
    Constraints, Size,
};

use crate::{
    app::{runner::window_handler::APP_HANDLE, tree::NodeRef},
    prelude::{InheritedState, InheritedWidget, Widget, WidgetState},
};

pub struct LayoutCx<T> {
    cx: LayoutCxOS,
    _p: PhantomData<T>,
}

impl<T> LayoutCx<T> {
    pub(crate) fn new(any: LayoutCxOS) -> Self {
        Self {
            cx: any,
            _p: PhantomData,
        }
    }

    pub fn depend_on_inherited_widget<W>(&self) -> Option<InheritedState<W::State>>
    where
        W: InheritedWidget + WidgetState,
    {
        // Register and get inherited widget of specified key.
        let node = self
            .node
            .depend_on_inherited_widget_of_key::<W::UniqueTypeId>()?;

        Some(InheritedState {
            node,
            _p: PhantomData,
        })
    }
}

impl<W: Widget> RenderExt<W> for LayoutCx<W> {
    fn node(&self) -> &NodeRef {
        &self.cx.node
    }
}

impl<T> std::ops::Deref for LayoutCx<T> {
    type Target = LayoutCxOS;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<T> std::ops::DerefMut for LayoutCx<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}

#[derive(Clone)]
pub struct LayoutCxOS {
    node: NodeRef,
}

impl RenderOSExt for LayoutCxOS {
    fn node(&self) -> &NodeRef {
        &self.node
    }
}

impl LayoutCxOS {
    pub(crate) fn new(node: NodeRef) -> Self {
        Self { node }
    }

    pub fn layout(&self, constraints: Constraints) -> Size {
        let widget = self.node.widget();

        let size = widget.layout(self.clone(), constraints);

        if cfg!(debug_assertions) {
            if size > constraints.biggest() {
                if widget.debug_name_short() != "DebugContainer" {
                    log::warn!("`{}` overflowed", widget.debug_name_short());
                }
            }
        }

        let render_data = &mut self.node.borrow_mut().render_data;

        render_data.size = size;
        render_data.laid_out = true;
        render_data.constraints = constraints;

        size
    }

    pub fn child(&self, index: usize) -> LayoutCxOS {
        self.try_child(index)
            .expect("specified node didn't have any children")
    }

    pub fn children(&self) -> LayoutCxIter {
        LayoutCxIter {
            child_idx: 0,
            parent_cx: self,
        }
    }

    fn try_child(&self, index: usize) -> Option<LayoutCxOS> {
        let child = self.node.child(index)?;

        Some(LayoutCxOS::new(child))
    }

    pub fn schedule_layout(&mut self) {
        APP_HANDLE.with(|handle| {
            handle
                .borrow_mut()
                .as_mut()
                .expect("APP_HANDLE wasn't set")
                .schedule_idle(IdleToken::new(0));
        });
    }
}

pub struct LayoutCxIter<'a> {
    child_idx: usize,
    parent_cx: &'a LayoutCxOS,
}

impl<'a> LayoutCxIter<'a> {
    pub fn len(&self) -> usize {
        self.parent_cx.node.children().len()
    }
}

impl Clone for LayoutCxIter<'_> {
    fn clone(&self) -> Self {
        Self {
            // Reset iterator.
            child_idx: 0,
            parent_cx: self.parent_cx,
        }
    }
}

impl<'a> Iterator for LayoutCxIter<'a> {
    type Item = LayoutCxOS;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.parent_cx.try_child(self.child_idx);
        self.child_idx += 1;
        r
    }
}
