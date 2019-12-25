extern crate console_error_panic_hook;
extern crate console_log;
extern crate log;
extern crate mogwai;
extern crate serde;
extern crate serde_json;

use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
//use std::str::from_utf8;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn new_form(tx: Transmitter<Event>) -> GizmoBuilder
{
    form()
        .with(label().text("The Value"))
        .with(input().attribute("type", "text")
            .tx_on("input", tx))
        .with(label().text("A button"))
        .with(input().attribute("type", "button").value("Submit or die!"))
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let (tx, rx) = txrx_map(|e :&Event| {
        // if Option was ok then something like this could be done:
        //let input: HtmlInputElement = e.target()?.dyn_into().ok()?;
        //Some(input.value().trim().to_string())
        match e.target() {
            Some(target) => {
                match target.dyn_into::<HtmlInputElement>() {
                    Ok(input) => input.value().trim().to_string(),
                    Err(_) => String::new(),
                }
            },
            None => String::new()
        }
    });

    div()
        .with(
            h1()
                .text("Hello from mogwai"))
        .with(new_form(tx))
        .rx_text("", rx)
        .build()?
        .run()
}
