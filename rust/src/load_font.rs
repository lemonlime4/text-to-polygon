use std::cell::RefCell;

use harfrust::FontRef;
use skrifa::raw::ReadError;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default())
}

#[derive(Default)]
struct State {
    font_bytes: Vec<u8>,
    // font: FontRef
}

pub fn load_font<'a>(
    decompressed: &'a mut Vec<u8>,
    font_bytes: &'a [u8],
) -> Result<FontRef<'a>, String> {
    Ok(match FontRef::new(font_bytes) {
        Ok(font) => font,
        Err(err) => {
            use ReadError::InvalidSfnt;
            use wuff::*;

            *decompressed = match err {
                InvalidSfnt(WOFF1_SFNT) => decompress_woff1(font_bytes),
                InvalidSfnt(WOFF2_SFNT) => decompress_woff2(font_bytes),
                _ => return Err(err.to_string()),
            }
            .map_err(|e| e.to_string())?;

            FontRef::new(decompressed).map_err(|e| e.to_string())?
        }
    })
}

const WOFF1_SFNT: u32 = 0x774F4646;
const WOFF2_SFNT: u32 = 0x774F4632;
