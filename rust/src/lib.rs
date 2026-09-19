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
    sampling::{RecursiveSampler, sample_glyph_buf},
    shaping::get_glyph_buf,
};

// returns None only when no font loaded
#[wasm_bindgen]
pub fn to_desmos_expr(text: &str) -> Result<Option<String>, String> {
    STATE
        .with_borrow(|state| -> Option<Result<_, _>> {
            let font = state.loaded_font.as_ref()?;
            let buf = get_glyph_buf(font.borrow_shaper(), text);
            let points =
                sample_glyph_buf(&RecursiveSampler::new(0.01), font.borrow_font_ref(), buf);
            // let points: Result<_, &str> =
            //     Ok(AdaptiveSampler::new(0.1).sample_cubic(curves::CubicBezier {
            //         p0: vec2(-4.94, -2.1),
            //         c0: vec2(-1.9, 14.45),
            //         c1: vec2(1.5, 18.26),
            //         p1: vec2(15.7, 14.9),
            //     }));
            Some(points.map(|ps| format!("{}", PolygonFormatter(&ps))))
        })
        .transpose()
        .map_err(|e| e.to_string())
}
