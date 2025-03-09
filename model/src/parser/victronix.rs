// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, combinator::recognize, Parser};

use super::{
    for_nom::{alnum_uppers, digits, lines3, year2_week2},
    GenericPart, Manufacturer, NomParser,
};

/// Victronix VN4464 (SOP-28, 5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::victronix::VICTRONIX_VN4464.parse("Victronix VN4464S-08LL 95103B029").is_ok());
/// ```
pub static VICTRONIX_VN4464: NomParser<GenericPart> = NomParser {
    name: "Victronix VN4464",
    f: |input| {
        lines3(
            tag("Victronix"),
            recognize(tag("VN4464").and(tag("S-08LL"))),
            (year2_week2, digits(1), alnum_uppers(1), digits(3)),
        )
        .map(|(_, kind, (date_code, _, _, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Victronix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
