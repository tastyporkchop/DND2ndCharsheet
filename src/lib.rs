use crate::character_model::Character;
use log::Level;
use mogwai::prelude::*;
use std::panic;
use std::result::Result;
use wasm_bindgen::prelude::*;

mod character_model;
mod common;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let character = Character {
        ..Default::default()
    };
    character.into_component().run()
}
