use frui::prelude::*;
use frui::render::*;

use druid_shell::piet::{
    kurbo::Point, Color, FontFamily, FontWeight, PietTextLayout, Text as TextExt, TextLayout,
    TextLayoutBuilder,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextDirection {
    Rtl,
    Ltr,
}

#[derive(RenderWidget, Builder)]
pub struct Text<S: AsRef<str>> {
    text: S,
    size: f64,
    color: Color,
    weight: FontWeight,
    family: FontFamily,
}

impl<S: AsRef<str>> Text<S> {
    pub fn new(string: S) -> Self {
        Self {
            text: string,
            size: 16.,
            color: Color::WHITE,
            weight: FontWeight::default(),
            // Layout of `FontFamily::SYSTEM_UI` is incredibly slow. Other fonts
            // seem to render just fine. This issue is related to Piet.
            //
            // For now, the default will be `FontFamily::MONOSPACE`.
            family: FontFamily::MONOSPACE,
        }
    }
}

#[cfg(not(feature = "miri"))]
impl<S: AsRef<str>> RenderState for Text<S> {
    type State = PietTextLayout;

    fn create_state(&self) -> Self::State {
        TEXT_FACTORY.with(|f| f.get().new_text_layout("").build().unwrap())
    }
}

#[cfg(not(feature = "miri"))]
impl<S: AsRef<str>> RenderWidget for Text<S> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![] as Vec<()>
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let max_width = constraints.biggest().width;

        *ctx.render_state_mut() = TEXT_FACTORY.with(|f| {
            f.get()
                .new_text_layout(self.text.as_ref().to_owned())
                .font(self.family.clone(), self.size)
                .text_color(self.color.clone())
                .range_attribute(.., self.weight)
                .max_width(max_width)
                .build()
                .unwrap()
        });

        let text_size = ctx.render_state().size().into();

        constraints.constrain(text_size)
    }

    fn paint(&self, ctx: &mut PaintCtx<Self>, canvas: &mut Canvas, offset: &Offset) {
        RenderContext::draw_text(
            canvas,
            &ctx.render_state(),
            Point {
                x: offset.x,
                y: offset.y,
            },
        );
    }
}

#[cfg(feature = "miri")]
pub struct TextRenderState([u8; 30]);

#[cfg(feature = "miri")]
impl<S: AsRef<str>> RenderState for Text<S> {
    type State = TextRenderState;

    fn create_state(&self) -> Self::State {
        TextRenderState([1; 30])
    }
}

#[cfg(feature = "miri")]
impl<S: AsRef<str>> RenderWidget for Text<S> {
    fn build<'w>(&'w self, _: BuildCtx<'w, Self>) -> Vec<Self::Widget<'w>> {
        vec![] as Vec<()>
    }

    fn layout(&self, ctx: &LayoutCtx<Self>, constraints: Constraints) -> Size {
        let _: &mut TextRenderState = &mut ctx.render_state_mut();

        Size {
            width: constraints.smallest().width,
            height: constraints.smallest().height,
        }
    }

    fn paint(&self, _: &mut PaintCtx<Self>, _: &mut Canvas, _: &Offset) {}
}
