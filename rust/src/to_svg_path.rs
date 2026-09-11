//! Outputting svg curves for debug
use glam::Vec2;

use wasm_bindgen::prelude::*;

use crate::{
    get_glyph_buf,
    load_font::STATE,
    shaping::{BasePen, PenOutput, draw_glyph_buf},
};

// returns None only when no font loaded
#[wasm_bindgen]
pub fn to_svg_path(text: &str) -> Result<Option<String>, String> {
    STATE
        .with_borrow(|state| -> Option<Result<_, _>> {
            let font = state.loaded_font.as_ref()?;
            let buf = get_glyph_buf(font.borrow_shaper(), text);
            Some(draw_glyph_buf(
                Box::new(SvgOutput::default()),
                font.borrow_font_ref(),
                buf,
            ))
        })
        .transpose()
        .map_err(|e| e.to_string())
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
        let p1 = base.glyph_pos() + p1;
        self.path.push_str(&format!("M{:.4} {:.4}", p1.x, p1.y));
    }
    fn line_to(&mut self, base: &BasePen, p1: Vec2) {
        let p1 = base.glyph_pos() + p1;
        self.path.push_str(&format!("L{:.4} {:.4}", p1.x, p1.y));
    }
    fn quad_to(&mut self, base: &BasePen, c: Vec2, p1: Vec2) {
        let c = base.glyph_pos() + c;
        let p1 = base.glyph_pos() + p1;
        self.path
            .push_str(&format!("Q{:.4} {:.4} {:.4} {:.4}", c.x, c.y, p1.x, p1.y));
    }
    fn curve_to(&mut self, base: &BasePen, c0: Vec2, c1: Vec2, p1: Vec2) {
        let c0 = base.glyph_pos() + c0;
        let c1 = base.glyph_pos() + c1;
        let p1 = base.glyph_pos() + p1;
        self.path.push_str(&format!(
            "C{:.4} {:.4} {:.4} {:.4} {:.4} {:.4}",
            c0.x, c0.y, c1.x, c1.y, p1.x, p1.y
        ));
    }
    fn close(&mut self, _base: &BasePen) {
        self.path.push_str("Z");
    }
}
