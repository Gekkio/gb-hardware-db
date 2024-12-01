// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, character::streaming::char, sequence::tuple, Parser};

use super::{
    for_nom::{alnum_uppers, digits, year2_week2},
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
        tuple((
            tag("Victronix "),
            tuple((tag("VN4464"), tag("S-08LL"))),
            char(' '),
            tuple((year2_week2, digits(1), alnum_uppers(1), digits(3))),
        ))
        .map(|(_, (kind, attrs), _, (date_code, _, _, _))| GenericPart {
            kind: format!("{kind}{attrs}"),
            manufacturer: Some(Manufacturer::Victronix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
