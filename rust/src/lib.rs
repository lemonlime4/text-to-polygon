mod curves;
mod format;
mod sampling;
mod shaping;
mod state;
mod utils;

use harfrust::ShaperFont;
use serde::Serialize;
use skrifa::FontRef;
pub use state::load_font;

// debug
mod to_svg_path;

use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::{
    format::LatexFormatter,
    sampling::{SamplingMethod, sample_glyph_buf},
    shaping::{SegmentOutput, get_glyph_buf},
    state::{FontSettings, STATE},
};

// tsify produces any if you pass glam::Vec2
#[derive(Tsify, Serialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// tsify produces incorrect typescript definitions if you pass it SegmentOutput<String>
#[derive(Tsify, Serialize)]
pub struct LatexOutput {
    latex: String,
    advance: crate::Vec2,
}

// returns None only when no font loaded
#[wasm_bindgen]
pub fn to_desmos_expr(
    text: &str,
    settings: Ts<FontSettings>,
    method: Ts<SamplingMethod>,
) -> Result<Option<Ts<LatexOutput>>, String> {
    STATE
        .with_borrow(|state| -> anyhow::Result<Option<_>> {
            let method = method.to_rust()?;
            let settings = settings.to_rust()?;
            let Some(font_state) = state.loaded_font.as_ref() else {
                return Ok(None);
            };
            let font_ref = FontRef::new(&font_state.bytes)?;
            let buf = get_glyph_buf(&ShaperFont::new(&font_state.font), text)?;
            let output = sample_glyph_buf(&*method.get_sampler(), &font_ref, &settings, buf)?;
            let output = SegmentOutput {
                output: format!("{}", LatexFormatter(&output.output)),
                advance: output.advance,
            };
            Ok(Some(
                LatexOutput {
                    latex: output.output,
                    advance: crate::Vec2 {
                        x: output.advance.x,
                        y: output.advance.y,
                    },
                }
                .into_ts()?,
            ))
        })
        .map_err(|e| format!("{e}"))
}
