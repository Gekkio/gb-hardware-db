// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, character::streaming::char, sequence::tuple, Parser as _};

use super::{
    for_nom::{alnum_uppers, month1_123abc, year1},
    GenericPart, Manufacturer, NomParser,
};
use crate::parser::PartDateCode;

/// TI SN74LV2416
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ti::TI_SN74LV2416.parse("LV2416 17M A23D").is_ok());
/// assert!(parser::ti::TI_SN74LV2416.parse("LV2416 13M A8R3").is_ok());
/// assert!(parser::ti::TI_SN74LV2416.parse("LV2416 0CM A73E").is_ok());
/// ```
pub static TI_SN74LV2416: NomParser<GenericPart> = NomParser {
    name: "TI SN74LV2416",
    f: |input| {
        tuple((
            tag("LV2416 "),
            tuple((year1, month1_123abc, tag("M"))),
            char(' '),
            tag("A").and(alnum_uppers(3)),
        ))
        .map(|(_, (year, month, _), _, _)| GenericPart {
            kind: "SN74LV2416".to_owned(),
            manufacturer: Some(Manufacturer::TexasInstruments),
            date_code: Some(PartDateCode::YearMonth { year, month }),
        })
        .parse(input)
    },
};
