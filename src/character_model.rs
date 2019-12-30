use crate::common::CharError;
use crate::common::CharError::{InvalidPercentile, InvalidStrength};
use crate::common::Result;
use crate::utils;
use crate::utils::{build_form_field_input, build_form_field_select, input_error_handler};
use log::{error, info};
use mogwai::prelude::*;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
    CharClass(CharacterClass),
    Dex(Option<i32>),
    Con(Option<i32>),
    Int(Option<i32>),
    Wis(Option<i32>),
    Cha(Option<i32>),
    JsonRender(Option<String>),
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
            2 | 3 => Ok(-3),
            4 | 5 => Ok(-2),
            6 | 7 => Ok(-1),
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
            }
            19 | 20 => Ok(3),
            21 | 22 => Ok(4),
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
            4 | 5 => Ok(-1),
            6 | 7 => Ok(0),
            8 | 9 => Ok(0),
            10 | 11 => Ok(0),
            12 | 13 => Ok(0),
            14 | 15 => Ok(0),
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
                        i => Err(InvalidPercentile(Some(i))),
                    }
                } else {
                    Ok(2)
                }
            }
            19 => Ok(7),
            20 => Ok(8),
            21 => Ok(9),
            22 => Ok(10),
            23 => Ok(11),
            24 => Ok(12),
            25 => Ok(14),
            i => Err(InvalidStrength(i)),
        }
    }
    fn weight_allow(&self) -> Result<i32> {
        match self.str {
            1 => Ok(1),
            2 => Ok(1),
            3 => Ok(5),
            4 | 5 => Ok(10),
            6 | 7 => Ok(20),
            8 | 9 => Ok(35),
            10 | 11 => Ok(40),
            12 | 13 => Ok(45),
            14 | 15 => Ok(55),
            16 => Ok(70),
            17 => Ok(85),
            18 => {
                if let Some(p) = self.per {
                    match p {
                        1..=50 => Ok(135),
                        51..=75 => Ok(160),
                        76..=90 => Ok(185),
                        91..=99 => Ok(235),
                        100 => Ok(335),
                        i => Err(InvalidPercentile(Some(i))),
                    }
                } else {
                    Ok(110)
                }
            }
            19 => Ok(485),
            20 => Ok(535),
            21 => Ok(635),
            22 => Ok(785),
            23 => Ok(935),
            24 => Ok(1235),
            25 => Ok(1535),
            i => Err(InvalidStrength(i)),
        }
    }
    fn max_press(&self) -> Result<i32> {
        match self.str {
            1 => Ok(3),
            2 => Ok(5),
            3 => Ok(10),
            4 | 5 => Ok(25),
            6 | 7 => Ok(55),
            8 | 9 => Ok(90),
            10 | 11 => Ok(115),
            12 | 13 => Ok(140),
            14 | 15 => Ok(170),
            16 => Ok(195),
            17 => Ok(220),
            18 => {
                if let Some(p) = self.per {
                    match p {
                        1..=50 => Ok(280),
                        51..=75 => Ok(305),
                        76..=90 => Ok(330),
                        91..=99 => Ok(380),
                        100 => Ok(480),
                        i => Err(InvalidPercentile(Some(i))),
                    }
                } else {
                    Ok(255)
                }
            }
            19 => Ok(640),
            20 => Ok(700),
            21 => Ok(810),
            22 => Ok(970),
            23 => Ok(1130),
            24 => Ok(1440),
            25 => Ok(1750),
            i => Err(InvalidStrength(i)),
        }
    }
    fn open_doors(&self) -> Result<i32> {
        match self.str {
            1 => Ok(3),
            2 => Ok(5),
            3 => Ok(10),
            4 | 5 => Ok(25),
            6 | 7 => Ok(55),
            8 | 9 => Ok(90),
            10 | 11 => Ok(115),
            12 | 13 => Ok(140),
            14 | 15 => Ok(170),
            16 => Ok(195),
            17 => Ok(220),
            18 => {
                if let Some(p) = self.per {
                    match p {
                        1..=50 => Ok(280),
                        51..=75 => Ok(305),
                        76..=90 => Ok(330),
                        91..=99 => Ok(380),
                        100 => Ok(480),
                        i => Err(InvalidPercentile(Some(i))),
                    }
                } else {
                    Ok(255)
                }
            }
            19 => Ok(640),
            20 => Ok(700),
            21 => Ok(810),
            22 => Ok(970),
            23 => Ok(1130),
            24 => Ok(1440),
            25 => Ok(1750),
            i => Err(InvalidStrength(i)),
        }
    }
}

#[derive(Default, Serialize)]
pub struct Character {
    pub char_name: String,
    pub char_class: CharacterClass,
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
        let val = input
            .parse::<i32>()
            .map_err(|e| CharError::StrParseError(e))?;
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

        let val = input
            .parse::<i32>()
            .map_err(|e| CharError::PercentParseError(e))?;

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

    fn update(
        &mut self,
        msg: &Self::ModelMsg,
        tx_view: &Transmitter<Self::ViewMsg>,
        _sub: &Subscriber<Self::ModelMsg>,
    ) {
        match msg {
            In::CharName(input) => {
                self.char_name = input.clone();
            },
            In::CharClass(input) => {
                match input.parse::<CharacterClass>() {
                    Ok(cc) => {
                        self.char_class = cc;
                        tx_view.send(&Out::CharClass(cc));
                    },
                    Err(err) => {
                        error!("I'm not sure what to do with class:{} with error:{} so I'm just going to ignore it.", input, err)
                    }
                }
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

    fn builder(
        &self,
        tx: Transmitter<Self::ModelMsg>,
        rx: Receiver<Self::ViewMsg>,
    ) -> GizmoBuilder {
        // input field error handling
        rx.branch().respond(|msg| {
            match msg {
                Out::CharClass(_cc) => {
                    // do something
                }
                Out::StrPercentile(sp) => {
                    if let Some(e) = &sp.err {
                        match e {
                            CharError::InvalidStrength(_) | CharError::StrParseError(_) => {
                                input_error_handler("str", false);
                            }
                            CharError::PercentParseError(_) | CharError::InvalidPercentile(_) => {
                                input_error_handler("str_percentile", false);
                            }
                            _ => (),
                        }
                    } else {
                        input_error_handler("str", true);
                        input_error_handler("str_percentile", true);
                    }
                }
                Out::Dex(Some(_)) => input_error_handler("dex", true),
                Out::Dex(None) => input_error_handler("dex", false),
                Out::Con(Some(_)) => input_error_handler("con", true),
                Out::Con(None) => input_error_handler("con", false),
                Out::Int(Some(_)) => input_error_handler("int", true),
                Out::Int(None) => input_error_handler("int", false),
                Out::Wis(Some(_)) => input_error_handler("wis", true),
                Out::Wis(None) => input_error_handler("wis", false),
                Out::Cha(Some(_)) => input_error_handler("cha", true),
                Out::Cha(None) => input_error_handler("cha", false),
                Out::JsonRender(_) => {
                    // do nothing
                }
            }
        });

        // Character Name input field
        let char_name_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::CharName(input))
            }),
        );

        let class_select = select().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_select_value(ev)?;
                Some(In::CharClass(input))
            }),
        );

        // Str input field
        let str_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::Str(input))
            }),
        );

        // Str % input field
        let str_per_input = input()
            .rx_boolean_attribute(
                "disabled",
                false,
                rx.branch_filter_map(|ev| {
                    if let Out::StrPercentile(sp) = ev {
                        Some(18 != sp.str)
                    } else {
                        None
                    }
                }),
            )
            .tx_on(
                "input",
                tx.contra_filter_map(|ev: &Event| {
                    let input = utils::event_input_value(ev)?;
                    Some(In::StrPercentile(input))
                }),
            );

        let dex_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::Dex(input))
            }),
        );

        let con_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::Con(input))
            }),
        );

        let int_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::Int(input))
            }),
        );

        let wis_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::Wis(input))
            }),
        );

        let cha_input = input().tx_on(
            "input",
            tx.contra_filter_map(|ev: &Event| {
                let input = utils::event_input_value(ev)?;
                Some(In::Cha(input))
            }),
        );

        // Main form
        let char_form = form()
            .attribute("class", "pure-form pure-form-aligned")
            .with(
                fieldset()
                    // -- Character name --
                    .with(build_form_field_input(
                        char_name_input,
                        "character_name",
                        "Character",
                    ))
                    // -- Class --
                    .with(build_form_field_select(
                        class_select,
                        "character_class",
                        "Class / Kit",
                        vec![
                            "- Select One -",
                            CharacterClass::Fighter.to_string().as_str(),
                            CharacterClass::Cleric.to_string().as_str(),
                            CharacterClass::Wizard.to_string().as_str(),
                            CharacterClass::Rogue.to_string().as_str(),
                        ],
                    ))
                    // -- Str --
                    .with(build_form_field_input(str_input, "str", "Str"))
                    // -- Str Percentile --
                    .with(build_form_field_input(
                        str_per_input,
                        "str_percentile",
                        "Str %",
                    ))
                    // -- Dex --
                    .with(build_form_field_input(dex_input, "dex", "Dex"))
                    // -- Con --
                    .with(build_form_field_input(con_input, "con", "Con"))
                    // -- Int --
                    .with(build_form_field_input(int_input, "int", "Int"))
                    // -- Wis --
                    .with(build_form_field_input(wis_input, "wis", "Wis"))
                    // -- Cha --
                    .with(build_form_field_input(cha_input, "cha", "Cha")),
            );

        let json_render = textarea()
            .attribute("rows", "10")
            .attribute("cols", "50")
            .rx_text(
                "",
                rx.branch_filter_map(|ev| {
                    if let Out::JsonRender(Some(render)) = ev {
                        Some(render.clone())
                    } else {
                        None
                    }
                }),
            );

        let derived_str_scores = vec![
            p().rx_text(
                "Hit Adj",
                rx.branch_filter_map(|ev| match ev {
                    Out::StrPercentile(s) => match s.hit_adj() {
                        Ok(v) => Some(format!("Hit Adj: {}", v)),
                        Err(e) => Some(format!("Hit Adj: Err! {:?}", e)),
                    },
                    _ => None,
                }),
            ),
            p().rx_text(
                "Damage Adj",
                rx.branch_filter_map(|ev| match ev {
                    Out::StrPercentile(s) => match s.damage_adj() {
                        Ok(v) => Some(format!("Damage Adj: {}", v)),
                        Err(e) => Some(format!("Damage Adj: Err! {:?}", e)),
                    },
                    _ => None,
                }),
            ),
            p().rx_text(
                "Weight Allow",
                rx.branch_filter_map(|ev| match ev {
                    Out::StrPercentile(s) => match s.weight_allow() {
                        Ok(v) => Some(format!("Weight Allow: {}", v)),
                        Err(e) => Some(format!("Weight Allow: Err! {:?}", e)),
                    },
                    _ => None,
                }),
            ),
            p().rx_text(
                "Max Press",
                rx.branch_filter_map(|ev| match ev {
                    Out::StrPercentile(s) => match s.max_press() {
                        Ok(v) => Some(format!("Max Press: {}", v)),
                        Err(e) => Some(format!("Max Press: Err! {:?}", e)),
                    },
                    _ => None,
                }),
            ),
        ];

        let mut str_scores = div().attribute("class", "pure-g");
        for gb in derived_str_scores {
            str_scores = str_scores.with(div().attribute("class", "pure-u-sm-1-6").with(gb));
        }

        // -- main root --
        div()
            .with(
                div()
                    .attribute("class", "pure-g")
                    .with(div().attribute("class", "pure-u-lg-1-3").with(char_form))
                    .with(div().attribute("class", "pure-u-lg-1-3").with(str_scores)),
            )
            .with(json_render)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CharacterClass {
    Fighter,
    Cleric,
    Wizard,
    Rogue,
}

impl FromStr for CharacterClass {
    type Err = CharError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Fighter" => Ok(CharacterClass::Fighter),
            "Cleric" => Ok(CharacterClass::Cleric),
            "Wizard" => Ok(CharacterClass::Wizard),
            "Rogue" => Ok(CharacterClass::Rogue),
            _ => Err(CharError::CharacterClassParseError(String::from(s))),
        }
    }
}

impl Display for CharacterClass {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            CharacterClass::Fighter => write!(f, "Fighter"),
            CharacterClass::Cleric => write!(f, "Cleric"),
            CharacterClass::Wizard => write!(f, "Wizard"),
            CharacterClass::Rogue => write!(f, "Rogue"),
        }
    }
}

impl Default for CharacterClass {
    fn default() -> Self {
        CharacterClass::Fighter
    }
}
