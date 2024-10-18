// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::fmt;

pub mod config;
pub mod hash;
pub mod input;
pub mod parser;
pub mod time;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Console {
    Dmg,
    Sgb,
    Mgb,
    Mgl,
    Sgb2,
    Cgb,
    Agb,
    Ags,
    Gbs,
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
    macro_rules! single_parser {
        ($t:ty, $re:literal, $f:expr $(,)?) => {{
            static PARSER: std::sync::OnceLock<crate::parser::SingleParser<$t>> =
                std::sync::OnceLock::new();
            PARSER.get_or_init(|| crate::parser::SingleParser::compile($re, $f))
        }};
    }
    pub(crate) use single_parser;

    macro_rules! multi_parser {
        ($t:ty, $($m:expr),+ $(,)?) => {{
            static PARSER: std::sync::OnceLock<crate::parser::MultiParser<$t>> =
                std::sync::OnceLock::new();
            PARSER.get_or_init(|| {
                use crate::parser::LabelParser;
                let parsers: Vec<&'static dyn LabelParser<$t>> = vec![$($m),+];
                let parsers: Vec<&'static crate::parser::SingleParser<$t>> = parsers.into_iter().flat_map(|p| p.parsers()).collect();
                crate::parser::MultiParser::compile(parsers)
            })
        }};
    }
    pub(crate) use multi_parser;
}
