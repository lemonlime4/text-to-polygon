use std::cell::RefCell;

use harfrust::{FontRef, Shaper, ShaperData};
use ouroboros::self_referencing;
use serde::{Deserialize, Serialize};
use skrifa::raw::ReadError;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;
use wuff::{WuffErr, decompress_woff1, decompress_woff2};

thread_local! {
    pub static STATE: RefCell<State> = const { RefCell::new(State{loaded_font:None}) };
}

pub struct State {
    pub loaded_font: Option<Font>,
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct FontSettings {
    pub font_size: f32,
}

#[self_referencing]
pub struct Font {
    pub bytes: Vec<u8>,
    #[covariant]
    #[borrows(bytes)]
    pub font_ref: FontRef<'this>,
    #[borrows(font_ref)]
    pub shaper_data: ShaperData,
    #[covariant]
    #[borrows(font_ref, shaper_data)]
    pub shaper: Shaper<'this>,
}

fn make_font(bytes: Vec<u8>) -> Result<Font, ReadError> {
    Font::try_new(
        bytes,
        |bytes| FontRef::new(bytes),
        |font_ref| Ok(ShaperData::new(font_ref)),
        |font_ref, shaper_data| Ok(shaper_data.shaper(font_ref).build()),
    )
}

#[wasm_bindgen]
pub fn load_font(bytes: Vec<u8>) -> Result<(), String> {
    let bytes = decompress(bytes).map_err(|err| err.to_string())?;
    STATE
        .with_borrow_mut(|state| -> Result<(), ReadError> {
            state.loaded_font = Some(make_font(bytes)?);
            Ok(())
        })
        .map_err(|err| err.to_string())
}

fn decompress(bytes: Vec<u8>) -> Result<Vec<u8>, WuffErr> {
    match &bytes[0..4] {
        b"wOFF" => decompress_woff1(&bytes),
        b"wOF2" => decompress_woff2(&bytes),
        _ => Ok(bytes),
    }
}
