use std::cell::RefCell;

use harfrust::{FontRef, Shaper, ShaperData};
use ouroboros::self_referencing;
use skrifa::raw::ReadError;
use wasm_bindgen::prelude::wasm_bindgen;
use wuff::{WuffErr, decompress_woff1, decompress_woff2};

thread_local! {
    pub static STATE: RefCell<State> = const { RefCell::new(State { loaded_font:None}) };
}

#[derive(Default)]
pub struct State {
    pub loaded_font: Option<Font>,
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

// pub fn load_font<'a>(
//     font_bytes: &'a [u8],
// ) -> Result<FontRef<'a>, String> {
//     Ok(match FontRef::new(font_bytes) {
//         Ok(font) => font,
//         Err(err) => {
//             use ReadError::InvalidSfnt;
//             use wuff::*;

//             *decompressed = match err {
//                 InvalidSfnt(WOFF1_SFNT) => decompress_woff1(font_bytes),
//                 InvalidSfnt(WOFF2_SFNT) => decompress_woff2(font_bytes),
//                 _ => return Err(err.to_string()),
//             }
//             .map_err(|e| e.to_string())?;

//             FontRef::new(decompressed).map_err(|e| e.to_string())?
//         }
//     })
// }

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
