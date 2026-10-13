//! Outputting svg curves for debug
use glam::Vec2;

use harfrust::ShaperFont;
use skrifa::FontRef;
use tsify::Ts;
use wasm_bindgen::prelude::*;

use crate::{
    get_glyph_buf,
    shaping::{BasePen, PenOutput, draw_glyph_buf},
    state::{FontSettings, STATE},
};

// returns None only when no font loaded
#[wasm_bindgen]
pub fn to_svg_path(text: &str, settings: Ts<FontSettings>) -> Result<Option<String>, String> {
    let settings = settings.to_rust().unwrap();
    STATE.with_borrow(|state| -> Result<Option<_>, _> {
        let Some(font_state) = state.loaded_font.as_ref() else {
            return Ok(None);
        };
        let font_ref = FontRef::new(&font_state.bytes).map_err(|e| e.to_string())?;
        let buf =
            get_glyph_buf(&ShaperFont::new(&font_state.font), text).map_err(|e| e.to_string())?;
        draw_glyph_buf(Box::new(SvgOutput::default()), &font_ref, &settings, buf)
            .map_err(|e| e.to_string())
            .map(|s| Some(s))
    })
}

#[derive(Default)]
struct SvgOutput {
    path: String,
}

impl PenOutput for SvgOutput {
    type Output = String;
    fn output(self: Box<Self>) -> Self::Output {
        self.path
    }
    fn move_to(&mut self, base: &BasePen, p1: Vec2) {
        let p1 = base.transform(p1);
        self.path.push_str(&format!("M{:.4} {:.4}", p1.x, p1.y));
    }
    fn line_to(&mut self, base: &BasePen, p1: Vec2) {
        let p1 = base.transform(p1);
        self.path.push_str(&format!("L{:.4} {:.4}", p1.x, p1.y));
    }
    fn quad_to(&mut self, base: &BasePen, c: Vec2, p1: Vec2) {
        let c = base.transform(c);
        let p1 = base.transform(p1);
        self.path
            .push_str(&format!("Q{:.4} {:.4} {:.4} {:.4}", c.x, c.y, p1.x, p1.y));
    }
    fn curve_to(&mut self, base: &BasePen, c0: Vec2, c1: Vec2, p1: Vec2) {
        let c0 = base.transform(c0);
        let c1 = base.transform(c1);
        let p1 = base.transform(p1);
        self.path.push_str(&format!(
            "C{:.4} {:.4} {:.4} {:.4} {:.4} {:.4}",
            c0.x, c0.y, c1.x, c1.y, p1.x, p1.y
        ));
    }
    fn close(&mut self, _base: &BasePen) {
        self.path.push_str("Z");
    }
}
