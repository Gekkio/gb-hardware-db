// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    bytes::streaming::tag, character::streaming::char, combinator::opt, sequence::tuple,
    Parser as _,
};

use super::{
    for_nom::{digits, uppers, year1_week2},
    GenericPart, LabelParser, NomParser,
};
use crate::{macros::multi_parser, parser::rohm};

pub type Eeprom = GenericPart;

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::eeprom::LCS5.parse("LCS5 040").is_ok());
/// assert!(parser::eeprom::LCS5.parse("LCS5 435 09").is_ok());
/// ```
pub static LCS5: NomParser<Eeprom> = NomParser {
    name: "LCS5",
    f: |input| {
        tuple((
            tag("LCS5 "),
            year1_week2,
            opt(tuple((nom::character::complete::char(' '), digits(2)))),
        ))
        .map(|(_, date_code, _)| Eeprom {
            kind: "LC56".to_owned(),
            manufacturer: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::eeprom::LC56.parse("LC56 W617 08").is_ok());
/// ```
pub static LC56: NomParser<Eeprom> = NomParser {
    name: "LC56",
    f: |input| {
        tuple((tag("LC56 "), uppers(1).and(digits(3)), char(' '), digits(2)))
            .map(|(_, _, _, _)| Eeprom {
                kind: "LC56".to_owned(),
                manufacturer: None,
                date_code: None,
            })
            .parse(input)
    },
};

pub fn eeprom() -> &'static impl LabelParser<Eeprom> {
    multi_parser!(Eeprom, &LCS5, &LC56, &rohm::ROHM_9853, &rohm::ROHM_9854)
}
