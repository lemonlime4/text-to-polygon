use glam::{Vec2, ivec2, vec2};
use harfrust::{FontRef, GlyphBuffer, ShapeOptions, Shaper, UnicodeBuffer};
use skrifa::{
    GlyphId, MetadataProvider,
    instance::{LocationRef, Size},
    outline::{DrawError, DrawSettings, OutlinePen},
};

use std::iter;

use crate::{
    curves::{CubicBezier, Line, QuadBezier},
    sampling::Sampler,
};

pub fn get_glyph_buf(shaper: &Shaper, text: &str) -> GlyphBuffer {
    let mut unicode_buf = UnicodeBuffer::new();
    unicode_buf.push_str(text);
    unicode_buf.guess_segment_properties();
    shaper.shape(unicode_buf, ShapeOptions::new())
}

pub fn sample_glyph_buf(
    sampler: &dyn Sampler,
    font_ref: &FontRef,
    buf: GlyphBuffer,
) -> Result<Vec<Vec2>, DrawError> {
    let size = Size::new(16.0);
    let location = LocationRef::default();
    let scale = size.linear_scale(font_ref.metrics(size, location).units_per_em);
    let outlines = font_ref.outline_glyphs();
    let mut pen = Pen {
        sampler,
        pos: Vec2::ZERO,
        offset: Vec2::ZERO,
        output: Vec::new(),
        path_start: Vec2::ZERO,
        current_point: Vec2::ZERO,
    };
    for (glyph_info, glyph_pos) in iter::zip(buf.glyph_infos(), buf.glyph_positions()) {
        let glyph = outlines.get(GlyphId::new(glyph_info.glyph_id)).unwrap();
        let settings = DrawSettings::unhinted(size, location);
        pen.offset = ivec2(glyph_pos.x_offset, glyph_pos.y_offset).as_vec2() * scale;
        glyph.draw(settings, &mut pen)?;
        pen.pos += ivec2(glyph_pos.x_advance, glyph_pos.y_advance).as_vec2() * scale;
    }
    Ok(pen.output)
}

struct Pen<'a> {
    pos: Vec2,
    offset: Vec2,
    /// in glyph space
    path_start: Vec2,
    /// in glyph space
    current_point: Vec2,

    sampler: &'a dyn Sampler,
    output: Vec<Vec2>,
}

impl<'a> OutlinePen for Pen<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.output.push(Vec2::NAN);
        self.output.push(vec2(x, y));
        self.path_start = vec2(x, y);
        self.current_point = vec2(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.output.extend(
            self.sampler
                .sample_line(Line {
                    p0: self.current_point,
                    p1: vec2(x, y),
                })
                .into_iter()
                .map(|p| self.pos + self.offset + p),
        );
        self.current_point = vec2(x, y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.output.extend(
            self.sampler
                .sample_quad(QuadBezier {
                    p0: self.current_point,
                    c: vec2(cx0, cy0),
                    p1: vec2(x, y),
                })
                .into_iter()
                .map(|p| self.pos + self.offset + p),
        );
        self.current_point = vec2(x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.output.extend(
            self.sampler
                .sample_cubic(CubicBezier {
                    p0: self.current_point,
                    c0: vec2(cx0, cy0),
                    c1: vec2(cx1, cy1),
                    p1: vec2(x, y),
                })
                .into_iter()
                .map(|p| self.pos + self.offset + p),
        );
        self.current_point = vec2(x, y);
    }

    fn close(&mut self) {
        self.output.push(self.path_start);
        self.current_point = self.path_start;
    }
}
