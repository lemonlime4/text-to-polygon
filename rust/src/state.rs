use std::{cell::RefCell, sync::Arc};

use harfrust::Font;
use serde::{Deserialize, Serialize};
use skrifa::{FontRef, raw::ReadError::InvalidSfnt};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;
use wuff::{decompress_woff1, decompress_woff2};

thread_local! {
    pub static STATE: RefCell<State> = const { RefCell::new(State{loaded_font:None}) };
}

pub struct State {
    pub loaded_font: Option<FontState>,
}

pub struct FontState {
    pub font: Font,
    pub bytes: Arc<Vec<u8>>,
}

impl FontState {
    fn new(bytes: Vec<u8>) -> Result<Self, &'static str> {
        let bytes = Arc::new(bytes);
        let shared: Arc<dyn AsRef<[u8]> + Send + Sync> = bytes.clone();
        Ok(Self {
            bytes,
            font: Font::new(shared, 0).ok_or("Font::new parse error")?,
        })
    }
}

#[derive(Tsify, Serialize, Deserialize)]
pub struct FontSettings {
    pub font_size: f32,
}

#[wasm_bindgen]
pub fn load_font(mut bytes: Vec<u8>) -> Result<(), String> {
    const WOFF1_SFNT: u32 = 0x774F4646;
    const WOFF2_SFNT: u32 = 0x774F4632;

    match FontRef::new(&bytes) {
        Err(InvalidSfnt(WOFF1_SFNT)) => {
            bytes = decompress_woff1(&bytes).map_err(|e| format!("Decompression error:\n{e}"))?;
        }
        Err(InvalidSfnt(WOFF2_SFNT)) => {
            bytes = decompress_woff2(&bytes).map_err(|e| format!("Decompression error:\n{e}"))?;
        }
        Err(e) => Err(format!("Error parsing file:\n{e}"))?,
        _ => {}
    };

    STATE.with_borrow_mut(|state| {
        state.loaded_font = Some(FontState::new(bytes)?);
        Ok(())
    })
}
