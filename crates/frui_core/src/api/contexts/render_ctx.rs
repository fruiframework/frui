use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
    ops::{AddAssign, Deref, DerefMut},
    sync::atomic::Ordering,
};

use druid_shell::{kurbo::Point, IdleToken};

use crate::{
    api::events::Event,
    app::{
        runner::{handler::APP_HANDLE, PaintContext},
        tree::WidgetNodeRef,
    },
    prelude::{MultiChildWidget, SingleChildWidget, WidgetState},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Offset {
    pub x: f64,
    pub y: f64,
}

impl From<Offset> for Point {
    fn from(offset: Offset) -> Self {
        Point {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl From<&Offset> for Point {
    fn from(offset: &Offset) -> Self {
        Point {
            x: offset.x,
            y: offset.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl From<druid_shell::kurbo::Size> for Size {
    fn from(size: druid_shell::kurbo::Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

impl From<Size> for druid_shell::kurbo::Size {
    fn from(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        None
    }

    fn lt(&self, other: &Self) -> bool {
        self.width < other.width || self.height < other.height
    }

    fn le(&self, other: &Self) -> bool {
        self.width <= other.width || self.height <= other.height
    }

    fn gt(&self, other: &Self) -> bool {
        self.width > other.width || self.height > other.height
    }

    fn ge(&self, other: &Self) -> bool {
        self.width >= other.width || self.height >= other.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Constraints {
    pub min_width: f64,
    pub max_width: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl Constraints {
    pub fn max(&self) -> Size {
        Size {
            width: self.max_width,
            height: self.max_height,
        }
    }

    pub fn loosen(&self) -> Self {
        Self {
            min_width: 0.0,
            max_width: self.max_width,
            min_height: 0.0,
            max_height: self.max_height,
        }
    }

    pub fn tighten(&self) -> Self {
        Self {
            min_width: self.max_width,
            max_width: self.max_width,
            min_height: self.max_height,
            max_height: self.max_height,
        }
    }
}

pub trait RenderState {
    type State: 'static;

    fn create_state(&self) -> Self::State;
}

pub type RenderContext<'a, T> = &'a mut _RenderContext<'a, T>;

pub struct _RenderContext<'a, T> {
    ctx: &'a mut AnyRenderContext,
    _p: PhantomData<T>,
}

impl<'a, T> _RenderContext<'a, T> {
    pub(crate) fn new(any: &'a mut AnyRenderContext) -> Self {
        Self {
            ctx: any,
            _p: PhantomData,
        }
    }

    /// Render state.
    pub fn rstate(&self) -> Ref<T::State>
    where
        T: RenderState,
    {
        Ref::map(self.ctx.node.borrow(), |node| {
            node.render_data.state.deref().downcast_ref().unwrap()
        })
    }

    /// Render state mutably.
    pub fn rstate_mut(&mut self) -> RefMut<T::State>
    where
        T: RenderState,
    {
        RefMut::map(self.ctx.node.borrow_mut(), |node| {
            node.render_data.state.deref_mut().downcast_mut().unwrap()
        })
    }

    /// Widget state.
    pub fn wstate(&self) -> Ref<T::State>
    where
        T: WidgetState,
    {
        Ref::map(self.ctx.node.borrow(), |node| {
            node.state.deref().downcast_ref().unwrap()
        })
    }

    /// Widget state mutably.
    pub fn wstate_mut(&self) -> RefMut<T::State>
    where
        T: WidgetState,
    {
        if !STATE_UPDATE_SUPRESSED.load(Ordering::SeqCst) {
            self.ctx.node.mark_dirty();
        }

        RefMut::map(self.ctx.node.borrow_mut(), |node| {
            node.state.deref_mut().downcast_mut().unwrap()
        })
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

    //

    pub fn child(&mut self) -> ChildContext
    where
        T: SingleChildWidget,
    {
        self.ctx.child()
    }

    pub fn children(&mut self) -> ChildIter
    where
        T: MultiChildWidget,
    {
        self.ctx.children()
    }

    //

    #[track_caller]
    pub fn size(&self) -> Size {
        self.ctx.node.borrow().render_data.size
    }

    #[track_caller]
    pub fn offset(&self) -> Offset {
        self.ctx.node.borrow().render_data.offset
    }

    pub fn point_in_layout_bounds(&self, point: Point) -> bool {
        let Offset { x: o_x, y: o_y } = self.offset();
        let Point { x, y } = point;

        // Make point position local to the tested widget origin.
        let (x, y) = (x - o_x, y - o_y);

        let Size { width, height } = self.size();

        // Check if that point is in the widget bounds computed during layout.
        x >= 0.0 && x <= width && y >= 0.0 && y <= height
    }
}

pub struct ChildContext<'a> {
    ctx: AnyRenderContext,
    _p: PhantomData<&'a ()>,
}

impl<'a> ChildContext<'a> {
    pub fn size(&self) -> Size {
        self.ctx.node.borrow().render_data.size
    }

    pub fn try_data<'b, T: 'static>(&'b self) -> Option<Ref<'b, T>> {
        self.ctx
            .node
            .borrow()
            .render_data
            .state
            .downcast_ref::<T>()?;

        Some(Ref::map(self.ctx.node.borrow(), |node| {
            node.render_data.state.downcast_ref().unwrap()
        }))
    }

    pub fn layout(&mut self, constraints: Constraints) -> Size {
        self.ctx.layout(constraints.clone())
    }

    #[track_caller]
    pub fn paint(&mut self, canvas: &mut PaintContext, offset: &Offset) {
        self.ctx.paint(canvas, offset)
    }

    #[track_caller]
    pub fn handle_event(&mut self, event: &Event) {
        self.ctx.handle_event(event)
    }
}

pub struct AnyRenderContext {
    node: WidgetNodeRef,
}

impl AnyRenderContext {
    pub(crate) fn new(node: WidgetNodeRef) -> Self {
        Self { node }
    }

    pub(crate) fn child(&mut self) -> ChildContext {
        let child_node = self
            .node
            .children()
            .get(0)
            .expect("specified node didn't have any children");

        ChildContext {
            ctx: AnyRenderContext::new(crate::app::tree::WidgetNode::node_ref(child_node)),
            _p: PhantomData,
        }
    }

    pub(crate) fn children(&mut self) -> ChildIter {
        ChildIter {
            child_idx: 0,
            parent: &self.node,
        }
    }

    pub(crate) fn layout(&mut self, constraints: Constraints) -> Size {
        let widget = self.node.widget().clone();
        let size = widget.layout(self, constraints);

        if cfg!(debug_assertions) {
            if size > constraints.max() {
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

    pub(crate) fn paint(&mut self, piet: &mut PaintContext, offset: &Offset) {
        assert!(
            self.node.borrow().render_data.laid_out,
            "child was not laid out before paint"
        );

        // This should probably be calculated during layout probably.
        self.node.borrow_mut().render_data.offset = offset.clone();

        self.node.widget().clone().paint(self, piet, offset);
    }

    pub(crate) fn handle_event(&mut self, event: &Event) {
        self.node.widget().clone().handle_event(self, event);
    }
}

pub struct ChildIter<'a> {
    child_idx: usize,
    parent: &'a WidgetNodeRef,
}

impl<'a> ChildIter<'a> {
    pub fn len(&self) -> usize {
        self.parent.children().len()
    }
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = ChildContext<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_child = match self.parent.children().get(self.child_idx) {
            Some(c) => c,
            None => return None,
        };

        self.child_idx += 1;

        Some(ChildContext {
            ctx: AnyRenderContext::new(crate::app::tree::WidgetNode::node_ref(next_child)),
            _p: PhantomData,
        })
    }
}

pub(crate) use sealed::RenderStateOS;

use super::build_ctx::STATE_UPDATE_SUPRESSED;

mod sealed {
    use std::any::Any;

    pub trait RenderStateOS {
        fn create_render_state(&self) -> Box<dyn Any>;
    }

    impl<T> RenderStateOS for T {
        default fn create_render_state(&self) -> Box<dyn Any> {
            Box::new(())
        }
    }

    impl<T: super::RenderState> RenderStateOS for T {
        fn create_render_state(&self) -> Box<dyn Any> {
            Box::new(T::create_state(&self))
        }
    }
}
