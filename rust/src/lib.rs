mod curves;
mod format;
mod sampling;
mod shaping;
mod state;
mod utils;

use harfrust::ShaperFont;
use skrifa::FontRef;
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
    STATE.with_borrow(|state| -> Result<Option<_>, _> {
        let Some(font_state) = state.loaded_font.as_ref() else {
            return Ok(None);
        };
        let font_ref = FontRef::new(&font_state.bytes).map_err(|e| e.to_string())?;
        let buf =
            get_glyph_buf(&ShaperFont::new(&font_state.font), text).map_err(|e| e.to_string())?;
        let points = sample_glyph_buf(
            &*method.to_rust().unwrap().get_sampler(),
            &font_ref,
            &settings.to_rust().unwrap(),
            buf,
        )
        .map_err(|e| e.to_string());
        points.map(|ps| Some(format!("{}", PolygonFormatter(&ps))))
    })
}
