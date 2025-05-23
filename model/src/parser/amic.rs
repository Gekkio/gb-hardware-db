// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
};

use super::{
    GenericPart,
    for_nom::{alnum_uppers, uppers},
};
use crate::parser::{Manufacturer, NomParser, for_nom::year2_week2};

/// AMIC LP62S16128 SRAM (TSOP-I-48, 2.7-3.6V, 2 Mibit / 256 KiB / 128x16)
///
/// Source:
///   "AMIC LP62S16128BW-T series - 128k x 16 bit low voltage CMOS SRAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::amic::AMIC_LP62S16128.parse("AMIC LP62S16128BW-70LLTF P4060473FB 0540A").is_ok());
/// ```
pub static AMIC_LP62S16128: NomParser<GenericPart> = NomParser {
    name: "AMIC LP62S16128",
    f: |input| {
        let package = Package::TsopI48;
        (
            tag("AMIC "),
            recognize((
                tag("LP62S16128")
                    .and(opt(one_of("ABC")))
                    .and(tag(package.code())),
                char('-'),
                tag("70"), // speed
                tag("LL"), // power
                tag("TF"),
            )),
            char(' '),
            alnum_uppers(10),
            char(' '),
            year2_week2.and(uppers(1)),
        )
            .map(|(_, kind, _, _, _, (date_code, _))| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Amic),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    TsopI48,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::TsopI48 => "W",
        }
        // V: TSOP-II-44
        // U: TFBGA-48
    }
}
