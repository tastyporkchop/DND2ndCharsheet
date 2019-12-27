use mogwai::prelude::*;
use mogwai::utils as mogwaiutils;
use log::{info, error};
use serde::Serialize;
use crate::utils;

#[derive(Debug, Clone)]
pub enum In {
    CharName(String),
    CharClass(String),
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
    Con(Option<i32>),
    Int(Option<i32>),
    Wis(Option<i32>),
    Cha(Option<i32>),
    JsonRender(Option<String>)
}

#[derive(Default, Serialize)]
pub struct Character {
    pub char_name: String,
    pub char_class: String,
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

impl Character {
    // TODO: implement derived values here
    fn to_json_string(&self) -> Option<String> {
        Some(serde_json::to_string_pretty(self).ok()?)
    }
}

impl Component for Character {
    type ModelMsg = In;
    type ViewMsg = Out;

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, _sub: &Subscriber<Self::ModelMsg>) {

        match msg {
            In::CharName(input) => {
                self.char_name = input.clone();
            },
            In::CharClass(input) => {
                self.char_class = input.clone();
            },
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
                        self.dex = input;
                        tx_view.send(&Out::Dex(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Dex(None))
                    },
                }
            },
            In::Con(input) => {
                match input.parse::<i32>() {
                    Ok(input) => {
                        info!("updated con to {}", input);
                        self.con = input;
                        tx_view.send(&Out::Con(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Con(None))
                    },
                }
            },
            In::Int(input) => {
                match input.parse::<i32>() {
                    Ok(input) => {
                        info!("updated int to {}", input);
                        self.int = input;
                        tx_view.send(&Out::Int(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Int(None))
                    },
                }
            },
            In::Wis(input) => {
                match input.parse::<i32>() {
                    Ok(input) => {
                        info!("updated wis to {}", input);
                        self.wis = input;
                        tx_view.send(&Out::Wis(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Wis(None))
                    },
                }
            },
            In::Cha(input) => {
                match input.parse::<i32>() {
                    Ok(input) => {
                        info!("updated cha to {}", input);
                        self.cha = input;
                        tx_view.send(&Out::Cha(Some(input)))
                    },
                    Err(_) => {
                        tx_view.send(&Out::Cha(None))
                    },
                }
            },
        }
        // after changes update render
        let json_render = self.to_json_string();
        tx_view.send(&Out::JsonRender(json_render));
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
                Out::Con(Some(_)) => {
                    input_error_handler("con", true)
                },
                Out::Con(None) => {
                    input_error_handler("con", false)
                },
                Out::Int(Some(_)) => {
                    input_error_handler("int", true)
                },
                Out::Int(None) => {
                    input_error_handler("int", false)
                },
                Out::Wis(Some(_)) => {
                    input_error_handler("wis", true)
                },
                Out::Wis(None) => {
                    input_error_handler("wis", false)
                },
                Out::Cha(Some(_)) => {
                    input_error_handler("cha", true)
                },
                Out::Cha(None) => {
                    input_error_handler("cha", false)
                },
                Out::JsonRender(_) => {
                    // do nothing
                }
            }
        });

        let char_form = form()
            .attribute("class", "pure-form pure-form-stacked")
            // -- Character name --
            .with(label().attribute("for", "character_name").text("Character"))
            .with(input()
                .id("character_name")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    info!("name event:{:?}", ev);
                    let input = utils::event_input_value(ev)?;
                    Some(In::CharName(input))
                }))
            )
            // -- Class --
            .with(label().attribute("for", "character_class").text("Class / Kit"))
            .with(select()
                .id("character_class")
                .with(option().attribute("value", "fighter").text("Fighter"))
                .with(option().attribute("value", "wizard").text("Wizard"))
                .with(option().attribute("value", "cleric").text("Cleric"))
                .with(option().attribute("value", "rogue").text("Rogue"))
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    info!("select event:{:?}", ev);
                    let input = utils::event_select_value(ev)?;
                    info!("class input:{}", input);
                    Some(In::CharClass(input))
                }))
            )
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
            .with(input()
                .id("con")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Con(input))
                })))
            .with(label().attribute("for", "int").text("Int"))
            .with(input()
                .id("int")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Int(input))
                })))
            .with(label().attribute("for", "wis").text("Wis"))
            .with(input()
                .id("wis")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Wis(input))
                })))
            .with(label().attribute("for", "cha").text("Cha"))
            .with(input()
                .id("cha")
                .attribute("type", "text")
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::Cha(input))
                })));

        let json_render = textarea()
            .rx_text("", rx.branch_filter_map(|ev| {
                if let Out::JsonRender(Some(render)) = ev {
                    Some(render.clone())
                } else {
                    None
                }
            }));
        div()
            .with(char_form)
            .with(json_render)
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

