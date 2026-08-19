mod utils;

use harfrust::{FontRef, GlyphBuffer, ShapeOptions, ShaperData, UnicodeBuffer};
use skrifa::{
    GlyphId, MetadataProvider,
    instance::{LocationRef, Size},
    outline::{DrawSettings, OutlinePen},
};

use std::cell::RefCell;
use std::iter;
use wasm_bindgen::prelude::*;

thread_local! {
    static STATE: RefCell<f64> = RefCell::new(1.0);
}

// todo figure out how to store this
struct State {
    font_bytes: Vec<u8>,
}

#[wasm_bindgen]
pub fn get_counter() -> f64 {
    STATE.with(|state| *state.borrow())
}

#[wasm_bindgen]
pub fn set_counter(value: f64) {
    STATE.with(|state| *state.borrow_mut() = value)
}

#[wasm_bindgen]
pub fn draw(font_bytes: &[u8], text: &str) -> Result<String, String> {
    let font = FontRef::new(font_bytes).map_err(|err| err.to_string())?;
    Ok(draw_glyph_buf(&font, get_glyph_buf(&font, text))?)
}

pub fn get_glyph_buf(font: &FontRef, text: &str) -> GlyphBuffer {
    let shaper_data = ShaperData::new(&font);
    let shaper = shaper_data.shaper(&font).build();

    let mut unicode_buf = UnicodeBuffer::new();
    unicode_buf.push_str(text);
    unicode_buf.guess_segment_properties();
    shaper.shape(unicode_buf, ShapeOptions::new())
}

pub fn draw_glyph_buf(font: &FontRef, buf: GlyphBuffer) -> Result<String, String> {
    let size = Size::new(16.0);
    let location = LocationRef::default();
    let scale = size.linear_scale(font.metrics(size, location).units_per_em);
    let outlines = font.outline_glyphs();
    let mut svg_path = SvgPath::default();
    for (glyph_info, glyph_pos) in iter::zip(buf.glyph_infos(), buf.glyph_positions()) {
        let glyph = outlines.get(GlyphId::new(glyph_info.glyph_id)).unwrap();
        let settings = DrawSettings::unhinted(size, location);
        svg_path.x_offset = glyph_pos.x_offset as f32 * scale;
        svg_path.y_offset = glyph_pos.y_offset as f32 * scale;
        glyph
            .draw(settings, &mut svg_path)
            .map_err(|err| err.to_string())?;
        svg_path.x += glyph_pos.x_advance as f32 * scale;
        svg_path.y += glyph_pos.y_advance as f32 * scale;
        // svg_path.advance_x +=
    }
    Ok(format!("{}", svg_path.path))
}

#[derive(Default)]
struct SvgPath {
    path: String,
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
}
impl OutlinePen for SvgPath {
    fn move_to(&mut self, x: f32, y: f32) {
        let x = x + self.x + self.x_offset;
        let y = y + self.y + self.y_offset;
        self.path.push_str(&format!("M{x:.1},{y:.1} "));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let x = x + self.x + self.x_offset;
        let y = y + self.y + self.y_offset;
        self.path.push_str(&format!("L{x:.1},{y:.1} "));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let cx0 = cx0 + self.x + self.x_offset;
        let cy0 = cy0 + self.y + self.y_offset;
        let x = x + self.x + self.x_offset;
        let y = y + self.y + self.y_offset;
        self.path
            .push_str(&format!("Q{cx0:.1},{cy0:.1} {x:.1},{y:.1} "));
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let cx0 = cx0 + self.x + self.x_offset;
        let cy0 = cy0 + self.y + self.y_offset;
        let cx1 = cx1 + self.x + self.x_offset;
        let cy1 = cy1 + self.y + self.y_offset;
        let x = x + self.x + self.x_offset;
        let y = y + self.y + self.y_offset;
        self.path.push_str(&format!(
            "C{cx0:.1},{cy0:.1} {cx1:.1},{cy1:.1} {x:.1},{y:.1} "
        ));
    }

    fn close(&mut self) {
        self.path.push_str("Z ");
    }
}
