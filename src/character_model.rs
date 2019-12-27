use mogwai::prelude::*;
use mogwai::utils as mogwaiutils;
use log::{info, error};
use serde::Serialize;
use crate::utils;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::num::ParseIntError;
use crate::character_model::CharError::{InvalidStrength, InvalidPercentile};

type Result<T> = std::result::Result<T, CharError>;

#[derive(Debug, Clone)]
pub enum CharError {
    ParseError(ParseIntError),
    InvalidStrength(i32),
    InvalidPercentile(Option<i32>),
}

impl Display for CharError {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            CharError::ParseError(pe) => pe.fmt(f),
            CharError::InvalidStrength(i) => write!(f, "invalid strength:{}", i),
            CharError::InvalidPercentile(i) => write!(f, "invalid strength percentile:{:?}", i),
        }
    }
}

impl Error for CharError {}

#[derive(Debug, Clone)]
pub enum In {
    CharName(String),
    CharClass(String),
    Str(String),
    StrPercentile(String),
    Dex(String),
    Con(String),
    Int(String),
    Wis(String),
    Cha(String),
}

#[derive(Debug, Clone)]
pub enum Out {
    StrPercentile(StrengthPercentile),
    Dex(Option<i32>),
    Con(Option<i32>),
    Int(Option<i32>),
    Wis(Option<i32>),
    Cha(Option<i32>),
    JsonRender(Option<String>)
}

#[derive(Debug, Clone)]
pub struct StrengthPercentile {
    str: i32,
    per: Option<i32>,
    err: Option<CharError>,
}

impl StrengthPercentile {
    fn hit_adj(&self) -> Result<i32> {
        match self.str {
            1 => Ok(-5),
            2|3 => Ok(-3),
            4|5 => Ok(-2),
            6|7 => Ok(-1),
            8..=16 => Ok(0),
            17 => Ok(1),
            18 => {
                if let Some(p) = self.per {
                    match p {
                        1..=50 => Ok(1),
                        51..=99 => Ok(2),
                        100 => Ok(3),
                        err => Err(CharError::InvalidPercentile(Some(err))),
                    }
                } else {
                    Ok(1)
                }
            },
            19|20 => Ok(3),
            21|22 => Ok(4),
            23 => Ok(5),
            24 => Ok(6),
            25 => Ok(7),
            err => Err(CharError::InvalidStrength(err)),
        }
    }
    fn damage_adj(&self) -> Result<i32> {
        match self.str {
            1 => Ok(-4),
            2 => Ok(-2),
            3 => Ok(-1),
            4|5 => Ok(-1),
            6|7 => Ok(0),
            8|9 => Ok(0),
            10|11 => Ok(0),
            12|13 => Ok(0),
            14|15 => Ok(0),
            16 => Ok(1),
            17 => Ok(1),
            18 => {
                if let Some(p) = self.per {
                    match p {
                        1..=50 => Ok(3),
                        51..=75 => Ok(3),
                        76..=90 => Ok(4),
                        91..=99 => Ok(5),
                        100 => Ok(6),
                        i => Err(InvalidPercentile(Some(i)))
                    }
                } else {
                    Ok(2)
                }
            },
            19 => Ok(7),
            20 => Ok(8),
            21 => Ok(9),
            22 => Ok(10),
            23 => Ok(11),
            24 => Ok(12),
            25 => Ok(14),
            i => Err(InvalidStrength(i))
        }
    }
}

#[derive(Default, Serialize)]
pub struct Character {
    pub char_name: String,
    pub char_class: String,
    pub str: i32,
    pub str_percentile: Option<i32>,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}

impl Character {


    fn to_json_string(&self) -> Option<String> {
        Some(serde_json::to_string_pretty(self).ok()?)
    }

    fn handle_str_update(&mut self, input: &str) -> Result<i32> {
        let val = input.parse::<i32>()
            .map_err(|e| { CharError::ParseError(e)} )?;
        if (1..=25).contains(&val) {
            self.str = val;
            Ok(val)
        } else {
            Err(CharError::InvalidStrength(val))
        }
    }

    fn handle_str_percentile_update(&mut self, input: &str) -> Result<Option<i32>> {
        if input == "" {
            self.str_percentile = None;
            return Ok(None);
        }

        let val = input.parse::<i32>()
            .map_err(|e| { CharError::ParseError(e)} )?;

        self.str_percentile = Some(val);

        if (1..=100).contains(&val) {
            Ok(Some(val))
        } else {
            Err(CharError::InvalidPercentile(Some(val)))
        }
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
                match self.handle_str_update(input.as_str()) {
                    Ok(input) => {
                        info!("updated str to {}", input);
                        tx_view.send(&Out::StrPercentile(StrengthPercentile{str: self.str, per: self.str_percentile, err: None}))
                    },
                    Err(e) => {
                        tx_view.send(&Out::StrPercentile(StrengthPercentile{str: self.str, per: self.str_percentile, err: Some(e)}))
                    }
                }
            },
            In::StrPercentile(input) => {
                match self.handle_str_percentile_update(input.as_str()) {
                    Ok(input) => {
                        info!("updated str_percentile to {:?}", input);
                        tx_view.send(&Out::StrPercentile(StrengthPercentile{str: self.str, per: self.str_percentile, err: None}))
                    },
                    Err(e) => {
                        tx_view.send(&Out::StrPercentile(StrengthPercentile{str: self.str, per: self.str_percentile, err: Some(e)}))
                    }
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
                Out::StrPercentile(sp) => {
                    if let Some(e) = &sp.err {
                        match e {
                            CharError::InvalidStrength(_) => {
                                input_error_handler("str", false);
                            },
                            CharError::InvalidPercentile(_) => {
                                input_error_handler("str_percentile", false);
                            },
                            _ => {},
                        }
                    } else {
                        input_error_handler("str", true);
                        input_error_handler("str_percentile", true);
                    }
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
            // -- Str Percentile --
            .with(label().attribute("for", "str_percentile").text("Str %"))
            .with(input()
                .id("str_percentile")
                .attribute("type", "text")
                .rx_boolean_attribute("disabled", false, rx.branch_filter_map(|ev| {
                    if let Out::StrPercentile(sp) = ev {
                        Some(18 != sp.str)
                    } else {
                        None
                    }
                }))
                .tx_on("input", tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::StrPercentile(input))
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

        let derived_scores = div()
            .with(p().rx_text("Hit Adj", rx.branch_filter_map(|ev| {
                match ev {
                    Out::StrPercentile(s) => {
                        match s.hit_adj() {
                            Ok(v) => Some(format!("Hit Adj: {}", v)),
                            Err(e) => Some(format!("Hit Adj: Err! {:?}", e))
                        }
                    },
                    _ => None
                }
            })))
            .with(p().rx_text("Damage Adj", rx.branch_filter_map(|ev| {
                match ev {
                    Out::StrPercentile(s) => {
                        match s.damage_adj() {
                            Ok(da) => Some(format!("Damage Adj: {}", da)),
                            Err(e) => Some(format!("Damage Adj: Error:{:?}", e)),
                        }
                    }
                    _ => None
                }
            })));
        div()
            .with(char_form)
            .with(derived_scores)
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


