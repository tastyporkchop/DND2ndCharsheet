use mogwai::prelude::*;
use mogwai::utils as mogwaiutils;
use log::{info, error};
use crate::utils;
//use crate::utils::{event_target_set_invalid, event_target_set_valid};
use web_sys::Document;
use web_sys::HtmlObjectElement;

#[derive(Debug, Clone)]
pub enum In {
    Str(String),
    Dex(String),
    Con(String),
    Int(String),
    Wis(String),
    Cha(String),
}

#[derive(Debug, Clone)]
pub enum Out {
    Str(Option<i32>),
    Dex(Option<i32>),
}

pub struct Character {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

impl Character {
    // TODO: implement derived values here
}

impl Component for Character {
    type ModelMsg = In;
    type ViewMsg = Out;

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, _sub: &Subscriber<Self::ModelMsg>) {
        match msg {
            In::Str(input) => {
                match input.parse::<i32>() {
                    Ok(input) => {
                        info!("updated str to {}", input);
                        self.str = input;
                        tx_view.send(&Out::Str(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Str(None))
                    },
                }
            },
            In::Dex(input) => {
                match input.parse::<i32>() {
                    Ok(input) => {
                        info!("updated dex to {}", input);
                        self.str = input;
                        tx_view.send(&Out::Dex(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Dex(None))
                    },
                }
            },
            In::Con(input) => {info!("updated str to {}", input)},
            In::Int(input) => {info!("updated str to {}", input)},
            In::Wis(input) => {info!("updated str to {}", input)},
            In::Cha(input) => {info!("updated str to {}", input)},
        }
    }

    fn builder(&self, tx: Transmitter<Self::ModelMsg>, rx: Receiver<Self::ViewMsg>) -> GizmoBuilder {
        // input field error handling
        rx.branch().respond(|msg| {
            match msg {
                Out::Str(Some(_)) => {
                    input_error_handler("str", true)
                }
                Out::Str(None) => {
                    input_error_handler("str", false)
                },
                Out::Dex(Some(_)) => {
                    input_error_handler("dex", true)
                },
                Out::Dex(None) => {
                    input_error_handler("dex", false)
                },
                _ => { },
            }
        });

        form()
            .attribute("class", "pure-form pure-form-stacked")
            // -- Str --
            .with(label().attribute("for", "str").text("Str"))
            .with(input()
                .id("str")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Str(input))
                })))
            // -- Dex --
            .with(label().attribute("for", "dex").text("Dex"))
            .with(input()
                .id("dex")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Dex(input))
                })))
            // -- Con --
            .with(label().attribute("for", "con").text("Con"))
            .with(input().id("con").attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Con(input))
                })))
            .with(label().attribute("for", "int").text("Int"))
            .with(input().id("int").attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Int(input))
                })))
            .with(label().attribute("for", "wis").text("Wis"))
            .with(input().id("wis").attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Wis(input))
                })))
            .with(label().attribute("for", "cha").text("Cha"))
            .with(input().id("cha").attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Cha(input))
                })))
    }
}

fn input_error_handler(element_id: &str, is_valid: bool) {
    let input = mogwaiutils::document()
        // TODO: remove expect and pass error up
        .get_element_by_id(element_id).expect("to find element")
        .dyn_into::<HtmlInputElement>();

    match input {
        Ok(element) => {
            if is_valid {
                info!("valid");
                element.set_custom_validity("");
            } else {
                info!("invalid");
                // TODO: set a good custom string
                element.set_custom_validity("invalid input");
            }
        },
        Err(e) => {
            error!("couldn't find element:{} error:{:?}", element_id, e);
        },
    }
}

