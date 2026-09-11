mod curves;
mod format;
mod load_font;
mod sampling;
mod shaping;
mod utils;

pub use load_font::load_font;

// debug
mod to_svg_path;

use wasm_bindgen::prelude::*;

use crate::{
    format::PolygonFormatter,
    load_font::STATE,
    sampling::{FixedCountSampler, sample_glyph_buf},
    shaping::get_glyph_buf,
};

// returns None only when no font loaded
#[wasm_bindgen]
pub fn to_desmos_expr(text: &str) -> Result<Option<String>, String> {
    STATE
        .with_borrow(|state| -> Option<Result<_, _>> {
            let font = state.loaded_font.as_ref()?;
            let buf = get_glyph_buf(font.borrow_shaper(), text);
            let points = sample_glyph_buf(&FixedCountSampler(20), font.borrow_font_ref(), buf);
            Some(points.map(|ps| format!("{}", PolygonFormatter(&ps))))
        })
        .transpose()
        .map_err(|e| e.to_string())
}
