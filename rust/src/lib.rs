mod curves;
mod format;
mod sampling;
mod shaping;
mod state;
mod utils;

pub use state::load_font;

// debug
mod to_svg_path;

use tsify::Ts;
use wasm_bindgen::prelude::*;

use crate::{
    format::PolygonFormatter,
    sampling::{SamplingMethod, sample_glyph_buf},
    shaping::get_glyph_buf,
    state::{FontSettings, STATE},
};

// returns None only when no font loaded
#[wasm_bindgen]
pub fn to_desmos_expr(
    text: &str,
    settings: Ts<FontSettings>,
    method: Ts<SamplingMethod>,
) -> Result<Option<String>, String> {
    STATE
        .with_borrow(|state| -> Option<Result<_, _>> {
            let font = state.loaded_font.as_ref()?;
            let buf = get_glyph_buf(font.borrow_shaper(), text);
            let points = sample_glyph_buf(
                &*method.to_rust().unwrap().get_sampler(),
                font.borrow_font_ref(),
                &settings.to_rust().unwrap(),
                buf,
            );
            Some(points.map(|ps| format!("{}", PolygonFormatter(&ps))))
        })
        .transpose()
        .map_err(|e| e.to_string())
}
