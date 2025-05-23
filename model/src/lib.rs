// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};

pub mod config;
pub mod hash;
pub mod input;
pub mod parser;
pub mod time;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Console {
    #[serde(rename = "dmg")]
    Dmg,
    #[serde(rename = "sgb")]
    Sgb,
    #[serde(rename = "mgb")]
    Mgb,
    #[serde(rename = "mgl")]
    Mgl,
    #[serde(rename = "sgb2")]
    Sgb2,
    #[serde(rename = "cgb")]
    Cgb,
    #[serde(rename = "agb")]
    Agb,
    #[serde(rename = "ags")]
    Ags,
    #[serde(rename = "gbs")]
    Gbs,
    #[serde(rename = "oxy")]
    Oxy,
}

impl Console {
    pub const ALL: [Console; 10] = [
        Console::Dmg,
        Console::Sgb,
        Console::Mgb,
        Console::Mgl,
        Console::Sgb2,
        Console::Cgb,
        Console::Agb,
        Console::Ags,
        Console::Gbs,
        Console::Oxy,
    ];
    pub const fn id(&self) -> &'static str {
        match self {
            Console::Dmg => "dmg",
            Console::Sgb => "sgb",
            Console::Mgb => "mgb",
            Console::Mgl => "mgl",
            Console::Sgb2 => "sgb2",
            Console::Cgb => "cgb",
            Console::Agb => "agb",
            Console::Ags => "ags",
            Console::Gbs => "gbs",
            Console::Oxy => "oxy",
        }
    }
    pub const fn code(&self) -> &'static str {
        match self {
            Console::Dmg => "DMG",
            Console::Sgb => "SGB",
            Console::Mgb => "MGB",
            Console::Mgl => "MGL",
            Console::Sgb2 => "SGB2",
            Console::Cgb => "CGB",
            Console::Agb => "AGB",
            Console::Ags => "AGS",
            Console::Gbs => "GBS",
            Console::Oxy => "OXY",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            Console::Dmg => "Game Boy",
            Console::Sgb => "Super Game Boy",
            Console::Mgb => "Game Boy Pocket",
            Console::Mgl => "Game Boy Light",
            Console::Sgb2 => "Super Game Boy 2",
            Console::Cgb => "Game Boy Color",
            Console::Agb => "Game Boy Advance",
            Console::Ags => "Game Boy Advance SP",
            Console::Gbs => "Game Boy Player",
            Console::Oxy => "Game Boy Micro",
        }
    }
}

impl fmt::Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

#[macro_use]
pub(crate) mod macros {
    macro_rules! multi_parser {
        ($t:ty, $($m:expr),* $(,)?) => {{
            static PARSERS: std::sync::OnceLock<Vec<&'static dyn crate::parser::LabelParser<$t>>> =
                std::sync::OnceLock::new();
            static PARSER: std::sync::OnceLock<crate::parser::MultiParser<$t>> =
                std::sync::OnceLock::new();
            PARSER.get_or_init(|| {
                crate::parser::MultiParser::new(&PARSERS.get_or_init(|| vec![$($m),*]))
            })
        }};
    }
    pub(crate) use multi_parser;
}

pub trait SubmissionMetadata {
    type PhotoKind;

    fn contributor(&self) -> &str;
    fn slug(&self) -> &str;
    fn identifier(&self) -> SubmissionIdentifier;

    fn set_contributor(&mut self, contributor: &str);
    fn update_identifier(&mut self, contributor_slug: &str, index: u16);
}

pub enum SubmissionIdentifier<'a> {
    Serial(&'a str),
    Index(u16),
}

impl<'a> SubmissionIdentifier<'a> {
    pub fn new(serial: &'a str, index: Option<u16>) -> Self {
        if serial.is_empty() {
            Self::Index(index.unwrap_or(1))
        } else {
            Self::Serial(serial)
        }
    }
}
