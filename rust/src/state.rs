use std::{cell::RefCell, sync::Arc};

use harfrust::Font;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;
use wuff::{WuffErr, decompress_woff1, decompress_woff2};

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
pub fn load_font(bytes: Vec<u8>) -> Result<(), String> {
    let bytes = decompress(bytes).map_err(|err| err.to_string())?;
    STATE.with_borrow_mut(|state| {
        state.loaded_font = Some(FontState::new(bytes)?);
        Ok(())
    })
}

fn decompress(bytes: Vec<u8>) -> Result<Vec<u8>, WuffErr> {
    match &bytes[0..4] {
        b"wOFF" => decompress_woff1(&bytes),
        b"wOF2" => decompress_woff2(&bytes),
        _ => Ok(bytes),
    }
}
