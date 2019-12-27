use mogwai::prelude::*;
use web_sys::{HtmlSelectElement};

pub fn event_input_value(ev:&Event) -> Option<String>
{
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

pub fn event_select_value(ev:&Event) -> Option<String>
{
    let input:HtmlSelectElement =
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
