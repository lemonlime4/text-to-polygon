use glam::{IVec2, Vec2, ivec2, vec2};
use harfrust::{FontRef, GlyphBuffer, ShapeOptions, Shaper, UnicodeBuffer};
use skrifa::{
    GlyphId, MetadataProvider,
    instance::{LocationRef, Size},
    outline::{DrawError, DrawSettings, OutlinePen},
};

use std::iter;

use crate::state::FontSettings;

pub fn get_glyph_buf(shaper: &Shaper, text: &str) -> GlyphBuffer {
    let mut unicode_buf = UnicodeBuffer::new();
    unicode_buf.push_str(text);
    unicode_buf.guess_segment_properties();
    shaper.shape(unicode_buf, ShapeOptions::new())
}

pub fn draw_glyph_buf<'a, T>(
    mut output: Box<dyn PenOutput<Output = T> + 'a>,
    font_ref: &FontRef,
    settings: &FontSettings,
    buf: GlyphBuffer,
) -> Result<T, DrawError> {
    let size = Size::new(settings.font_size);
    let location = LocationRef::default();
    let outlines = font_ref.outline_glyphs();
    let mut pen = Pen {
        base: BasePen {
            advance: IVec2::ZERO,
            offset: IVec2::ZERO,
            path_start: Vec2::ZERO,
            current_point: Vec2::ZERO,
            scale: size.linear_scale(font_ref.metrics(size, location).units_per_em),
        },
        output: &mut *output,
    };
    for (glyph_info, glyph_pos) in iter::zip(buf.glyph_infos(), buf.glyph_positions()) {
        let glyph = outlines.get(GlyphId::new(glyph_info.glyph_id)).unwrap();
        let settings = DrawSettings::unhinted(size, location);
        pen.base.offset = ivec2(glyph_pos.x_offset, glyph_pos.y_offset);
        glyph.draw(settings, &mut pen)?;
        pen.base.advance += ivec2(glyph_pos.x_advance, glyph_pos.y_advance);
    }
    Ok(output.output())
}

struct Pen<'a, T> {
    base: BasePen,
    output: &'a mut dyn PenOutput<Output = T>,
}

pub struct BasePen {
    advance: IVec2,
    offset: IVec2,
    scale: f32,
    /// in scaled glyph space
    pub path_start: Vec2,
    /// in scaled glyph space
    pub current_point: Vec2,
}

impl BasePen {
    pub fn glyph_pos(&self) -> Vec2 {
        self.scale * (self.advance + self.offset).as_vec2()
    }
}

pub trait PenOutput {
    type Output;
    fn output(self: Box<Self>) -> Self::Output;
    fn move_to(&mut self, base: &BasePen, p1: Vec2);
    fn line_to(&mut self, base: &BasePen, p1: Vec2);
    fn quad_to(&mut self, base: &BasePen, c: Vec2, p1: Vec2);
    fn curve_to(&mut self, base: &BasePen, c0: Vec2, c1: Vec2, p1: Vec2);
    fn close(&mut self, base: &BasePen);
}

impl<'a, T> OutlinePen for Pen<'a, T> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.output.move_to(&self.base, vec2(x, y));
        self.base.path_start = vec2(x, y);
        self.base.current_point = vec2(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.output.line_to(&self.base, vec2(x, y));
        self.base.current_point = vec2(x, y);
    }

    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.output.quad_to(&self.base, vec2(cx, cy), vec2(x, y));
        self.base.current_point = vec2(x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.output
            .curve_to(&self.base, vec2(cx0, cy0), vec2(cx1, cy1), vec2(x, y));
        self.base.current_point = vec2(x, y);
    }

    fn close(&mut self) {
        self.output.close(&self.base);
        self.base.current_point = self.base.path_start;
    }
}
