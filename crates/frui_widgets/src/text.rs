use frui::{app::TEXT_FACTORY, prelude::*};

use druid_shell::piet::{
    kurbo::Point, Color, FontFamily, FontWeight, PietTextLayout, Text as TextExt, TextLayout,
    TextLayoutBuilder,
};

#[derive(LeafWidget)]
pub struct Text<S: AsRef<str>> {
    text: S,
    font_size: f64,
    font_color: Color,
    font_weight: FontWeight,
    font_family: FontFamily,
}

impl<S: AsRef<str>> Text<S> {
    pub fn new(string: S) -> Self {
        Self {
            text: string,
            font_size: 16.,
            font_color: Color::WHITE,
            font_weight: FontWeight::default(),
            font_family: FontFamily::default(),
        }
    }

    pub fn size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.font_color = color;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn font(mut self, font: FontFamily) -> Self {
        self.font_family = font;
        self
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
impl<S: AsRef<str>> LeafWidget for Text<S> {
    fn layout(&self, ctx: RenderContext<Self>, constraints: Constraints) -> Size {
        let max_width = constraints.max().width;

        *ctx.rstate_mut() = TEXT_FACTORY.with(|f| {
            f.get()
                .new_text_layout(self.text.as_ref().to_owned())
                .font(self.font_family.clone(), self.font_size)
                .text_color(self.font_color.clone())
                .range_attribute(.., self.font_weight)
                .max_width(max_width)
                .build()
                .unwrap()
        });

        let text_size = ctx.rstate().size();

        Size {
            width: text_size.width,
            height: text_size.height,
        }
    }

    fn paint(&self, ctx: RenderContext<Self>, canvas: &mut PaintContext, offset: &Offset) {
        PietRenderContext::draw_text(
            canvas,
            &ctx.rstate(),
            Point {
                x: offset.x,
                y: offset.y,
            },
        );
    }
}

#[cfg(feature = "miri")]
impl<S: AsRef<str>> LeafWidget for Text<S> {
    fn layout(&self, _: RenderContext<Self>, constraints: Constraints) -> Size {
        Size {
            width: constraints.max().width,
            height: constraints.max().height,
        }
    }

    fn paint(&self, _: RenderContext<Self>, _: &mut PaintContext, _: &Offset) {}
}
