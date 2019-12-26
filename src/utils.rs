use mogwai::prelude::*;
use web_sys::HtmlObjectElement;
//use wasm_bindgen::prelude::*;

pub fn event_input_value(ev:&Event) -> Option<String> {
    let input:HtmlInputElement =
        ev
            .target()?
            .dyn_into()
            .ok()?;
    Some(
        input
            .value()
            .trim()
            .to_string()
    )
}

pub fn event_target_set_valid(ev: &Event) -> Option<String> {
    let input:HtmlObjectElement =
        ev
            .target()?
            .dyn_into()
            .ok()?;
    Some(String::from(""))
}

//pub fn event_target_set_invalid(ev: &EventTarget) {
//    let target: &Option<HtmlObjectElement> =
//        &ev.dyn_into().ok();
//
//    if let Some(t) = target {
//        t.set_custom_validity("not valid");
//    }
//}
