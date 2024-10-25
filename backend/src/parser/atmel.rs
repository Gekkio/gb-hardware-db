// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    sequence::tuple,
    Parser as _,
};

use super::GenericPart;
use crate::parser::{for_nom::year2_week2, Manufacturer, NomParser};

/// Atmel AT29LV512 (TSOP-I-32, 3.0-3.6V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::atmel::ATMEL_AT29LV512.parse("AT29LV512 15TC 0114").is_ok());
/// ```
pub static ATMEL_AT29LV512: NomParser<GenericPart> = NomParser {
    name: "Atmel AT29LV512",
    f: |input| {
        tuple((
            tag("AT29LV512"),
            char(' '),
            tuple((
                tag("15"),    // speed
                char('T'),    // package
                one_of("CI"), // grade
            )),
            char(' '),
            year2_week2,
        ))
        .map(
            |(kind, _, (speed, package, grade), _, date_code)| GenericPart {
                kind: format!("{kind}-{speed}{package}{grade}"),
                manufacturer: Some(Manufacturer::Atmel),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};
