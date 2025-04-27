// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::recognize,
};

use super::{GenericPart, for_nom::lines3};
use crate::parser::{Manufacturer, NomParser, for_nom::year2_week2};

/// Atmel AT29LV512 flash (TSOP-I-32, 3.0-3.6V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::atmel::ATMEL_AT29LV512.parse("AT29LV512 15TC 0114").is_ok());
/// ```
pub static ATMEL_AT29LV512: NomParser<GenericPart> = NomParser {
    name: "Atmel AT29LV512",
    f: |input| {
        lines3(
            tag("AT29LV512"),
            recognize((
                tag("15"),    // speed
                char('T'),    // package
                one_of("CI"), // grade
            )),
            year2_week2,
        )
        .map(|(kind, attrs, date_code)| GenericPart {
            kind: format!("{kind}-{attrs}"),
            manufacturer: Some(Manufacturer::Atmel),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
