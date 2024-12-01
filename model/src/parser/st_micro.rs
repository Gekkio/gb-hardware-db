// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{opt, recognize},
    sequence::tuple,
    Parser as _,
};

use super::{
    for_nom::{alnum_uppers, uppers, year1_week2},
    GenericPart,
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
        tuple((
            opt(tag("E ")),
            tag("M68AS128"),
            char(' '),
            recognize(tuple((
                tag("DL"),
                tag("70"), // speed
                tag("N"),  // package
                tag("6"),  // temperature
            ))),
            char(' '),
            uppers(5),
            tag(" F6 TWN "),
            alnum_uppers(2),
            char(' '),
            year1_week2,
        ))
        .map(
            |(_, kind, _, attrs, _, _, _, _, _, date_code)| GenericPart {
                kind: format!("{kind}{attrs}"),
                manufacturer: Some(Manufacturer::StMicro),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};
