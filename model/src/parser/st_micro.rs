// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{opt, recognize},
    sequence::{preceded, separated_pair},
};

use super::{
    GenericPart,
    for_nom::{alnum_uppers, lines4, uppers, year1_week2},
};
use crate::parser::{Manufacturer, NomParser};

/// STMicro M68AS128 (TSOP-I-48)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::st_micro::ST_MICRO_M68AS128.parse("M68AS128 DL70N6 AANFG F6 TWN 8B 414").is_ok());
/// ```
pub static ST_MICRO_M68AS128: NomParser<GenericPart> = NomParser {
    name: "STMicro M68AS128",
    f: |input| {
        preceded(
            opt(tag("E ")),
            lines4(
                tag("M68AS128"),
                recognize((
                    tag("DL"),
                    tag("70"), // speed
                    tag("N"),  // package
                    tag("6"),  // temperature
                )),
                separated_pair(uppers(5), char(' '), tag("F6")),
                (
                    tag("TWN"),
                    char(' '),
                    alnum_uppers(2),
                    char(' '),
                    year1_week2,
                ),
            ),
        )
        .map(|(kind, attrs, _, (_, _, _, _, date_code))| GenericPart {
            kind: format!("{kind}{attrs}"),
            manufacturer: Some(Manufacturer::StMicro),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
