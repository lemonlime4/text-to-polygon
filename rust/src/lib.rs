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
    format::LatexFormatter,
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
        .with_borrow(|state| -> anyhow::Result<Option<_>> {
            let method = method.to_rust()?;
            let settings = settings.to_rust()?;
            let Some(font_state) = state.loaded_font.as_ref() else {
                return Ok(None);
            };
            let font_ref = FontRef::new(&font_state.bytes)?;
            let buf = get_glyph_buf(&ShaperFont::new(&font_state.font), text)?;
            let points = sample_glyph_buf(&*method.get_sampler(), &font_ref, &settings, buf)?;
            let latex = format!("{}", LatexFormatter(&points));
            Ok(Some(latex))
        })
        .map_err(|e| format!("{e}"))
}
