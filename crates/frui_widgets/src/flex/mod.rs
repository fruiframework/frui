use std::ops::{Deref, DerefMut};

use frui::prelude::*;
use frui::render::*;

pub use alignment::*;
pub use center::*;
pub use flex::*;
pub use stack::*;

pub mod alignment;
pub mod center;
pub mod flex;
pub mod stack;

#[derive(Debug, Clone, Copy, Default)]
pub struct BoxLayoutData {
    pub offset: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainAxisSize {
    Min,
    Max,
}

impl Default for MainAxisSize {
    fn default() -> Self {
        MainAxisSize::Min
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    /// Leading space will not work with `MainAxisAlignment::SpaceAround` and `MainAxisAlignment::SpaceEvenly`
    SpaceAround,
    SpaceEvenly,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        MainAxisAlignment::Start
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CrossAxisSize {
    Min,
    Max,
}

impl Default for CrossAxisSize {
    fn default() -> Self {
        CrossAxisSize::Min
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        CrossAxisAlignment::Start
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexFit {
    Loose,
    Tight,
}

/// Used by flexible widgets to determine the flex factor of a child.
#[derive(Debug, Clone, Copy)]
pub struct FlexData {
    fit: FlexFit,
    flex_factor: usize,
    box_data: BoxLayoutData,
}

impl Default for FlexData {
    fn default() -> Self {
        FlexData {
            flex_factor: 0,
            fit: FlexFit::Loose,
            box_data: BoxLayoutData::default(),
        }
    }
}

impl Deref for FlexData {
    type Target = BoxLayoutData;

    fn deref(&self) -> &Self::Target {
        &self.box_data
    }
}

impl DerefMut for FlexData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.box_data
    }
}

#[derive(RenderWidget, Builder)]
pub struct Flexible<W: Widget> {
    pub fit: FlexFit,
    pub flex: usize,
    pub child: W,
}

impl Flexible<()> {
    pub fn new(child: impl Widget) -> Flexible<impl Widget> {
        Flexible::builder().child(child)
    }

    pub fn builder() -> Self {
        Self {
            fit: FlexFit::Loose,
            flex: 1,
            child: (),
        }
    }
}

impl<W: Widget> ParentData for Flexible<W> {
    type Data = FlexData;

    fn create_data(&self) -> Self::Data {
        FlexData {
            flex_factor: self.flex,
            fit: self.fit,
            box_data: BoxLayoutData::default(),
        }
    }
}

impl<W: Widget> RenderWidget for Flexible<W> {
    fn build<'w>(&'w self, _: BuildCx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![&self.child]
    }

    fn layout(&self, cx: &LayoutCx<Self>, constraints: Constraints) -> Size {
        constraints.constrain(cx.child(0).layout(constraints))
    }

    fn paint(&self, cx: &mut PaintCx<Self>, canvas: &mut Canvas, offset: &Offset) {
        cx.child(0).paint(canvas, offset)
    }
}

pub struct Expanded;

impl Expanded {
    pub fn new<T: Widget>(child: T) -> Flexible<T> {
        Flexible {
            fit: FlexFit::Tight,
            flex: 1,
            child,
        }
    }
}
