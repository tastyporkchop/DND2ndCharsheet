use mogwai::prelude::*;
use mogwai::utils as mogwaiutils;
use web_sys::{HtmlSelectElement};
use log::{error};

/// Extracts the value from an HTML input field from the given HTML Event
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

/// Extracts the value from an HTML `select` field from the given HTML Event
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


/// Sets `:invalid` pseudo css class on an element with `element_id`
pub fn input_error_handler(element_id: &str, is_valid: bool) {
    let input = mogwaiutils::document()
        // TODO: remove expect and pass error up
        .get_element_by_id(element_id).expect("to find element")
        .dyn_into::<HtmlInputElement>();

    match input {
        Ok(element) => {
            if is_valid {
                element.set_custom_validity("");
            } else {
                // TODO: set a good custom string
                element.set_custom_validity("invalid input");
            }
        },
        Err(e) => {
            error!("couldn't find element:{} error:{:?}", element_id, e);
        },
    }
}

/// Utility method: Wraps an input field with div and supplies css for formatting
pub fn build_form_field_input(input: GizmoBuilder, id: &str, name: &str) -> GizmoBuilder {
    div()
        .attribute("class", "pure-control-group")
        .with(label().attribute("for", id).text(name))
        .with(input
            .id(id)
            .attribute("type", "text")
        )
}

/// Utility method: Wraps an select field with div and supplies css for formatting
pub fn build_form_field_select(select: GizmoBuilder, id: &str, name: &str, options: Vec<&str>) -> GizmoBuilder {
    let mut select = select
        .id(id)
        .boolean_attribute("required");

    for opt in options {
        select = select.with(option().attribute("value", opt).text(opt));
    }

    div()
        .attribute("class", "pure-control-group")
        .with(label().attribute("for", id).text(name))
        .with(select)
}

