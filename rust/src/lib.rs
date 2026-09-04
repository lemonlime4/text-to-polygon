mod load_font;
mod utils;

// debug
mod to_svg_path;

use glam::{Vec2, ivec2, vec2};
use harfrust::{FontRef, GlyphBuffer, ShapeOptions, Shaper, UnicodeBuffer};
use skrifa::{
    GlyphId, MetadataProvider,
    instance::{LocationRef, Size},
    outline::{DrawError, DrawSettings, OutlinePen},
};

use core::fmt;
use std::iter;
use wasm_bindgen::prelude::*;

use crate::load_font::STATE;

#[wasm_bindgen]
pub fn to_desmos_expr(text: &str) -> Result<Option<String>, String> {
    STATE
        .with_borrow(|state| -> Option<Result<String, DrawError>> {
            let font = state.loaded_font.as_ref()?;
            let buf = get_glyph_buf(font.borrow_shaper(), text);
            Some(draw_glyph_buf(font.borrow_font_ref(), buf))
        })
        .transpose()
        .map_err(|e| e.to_string())
}

pub fn get_glyph_buf(shaper: &Shaper, text: &str) -> GlyphBuffer {
    let mut unicode_buf = UnicodeBuffer::new();
    unicode_buf.push_str(text);
    unicode_buf.guess_segment_properties();
    shaper.shape(unicode_buf, ShapeOptions::new())
}

pub fn draw_glyph_buf(font: &FontRef, buf: GlyphBuffer) -> Result<String, DrawError> {
    let size = Size::new(16.0);
    let location = LocationRef::default();
    let scale = size.linear_scale(font.metrics(size, location).units_per_em);
    let outlines = font.outline_glyphs();
    let mut pen = Pen::default();
    for (glyph_info, glyph_pos) in iter::zip(buf.glyph_infos(), buf.glyph_positions()) {
        let glyph = outlines.get(GlyphId::new(glyph_info.glyph_id)).unwrap();
        let settings = DrawSettings::unhinted(size, location);
        pen.offset = ivec2(glyph_pos.x_offset, glyph_pos.y_offset).as_vec2() * scale;
        glyph.draw(settings, &mut pen)?;
        pen.pos += ivec2(glyph_pos.x_advance, glyph_pos.y_advance).as_vec2() * scale;
    }
    Ok(format!("{:.3}", PolygonFormatter(&pen.output)))
}

#[derive(Default)]
struct Pen {
    pos: Vec2,
    offset: Vec2,

    path_start: Vec2,
    output: Vec<Vec2>,
}

struct PolygonFormatter<'a>(&'a [Vec2]);
impl fmt::Display for PolygonFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.0;
        let precision = f.precision().unwrap_or(10);

        f.write_str("\\operatorname{polygon}\\left(\\left[")?;

        for x in points[1..].iter().map(|v| v.x) {
            if x.is_nan() {
                f.write_str("0\\infinity,")?;
            } else {
                write!(f, "{x:.0$},", precision)?;
            }
        }

        f.write_str("0\\infinity\\right],\\left[")?;

        for y in points[1..].iter().map(|v| v.y) {
            if y.is_nan() {
                f.write_str("0\\infinity,")?;
            } else {
                write!(f, "{y:.0$},", precision)?;
            }
        }

        f.write_str("0\\infinity\\right]\\right)")?;
        Ok(())
    }
}

impl Pen {
    fn push(&mut self, p: Vec2) {
        self.output.push(self.pos + self.offset + p);
    }
}

impl OutlinePen for Pen {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path_start = vec2(x, y);
        self.push(Vec2::NAN);
        self.push(vec2(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.push(vec2(x, y));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.push(vec2(cx0, cy0));
        self.push(vec2(x, y));
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.push(vec2(cx0, cy0));
        self.push(vec2(cx1, cy1));
        self.push(vec2(x, y));
    }

    fn close(&mut self) {
        self.push(self.path_start);
    }
}
