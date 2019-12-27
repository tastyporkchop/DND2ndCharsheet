//extern crate console_error_panic_hook;
//extern crate console_log;
//extern crate log;
//extern crate mogwai;
//extern crate serde;
//extern crate serde_json;

use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use crate::character_model::Character;
//use std::str::from_utf8;

mod character_model;
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

    let character = Character{
        ..Default::default()
    };
    character.into_component()
        .run()
}