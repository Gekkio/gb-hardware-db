// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, character::streaming::char, sequence::tuple, Parser as _};

use crate::parser::{
    for_nom::{digits, year2_week2},
    Manufacturer, NomParser,
};

use super::GenericPart;

/// SST SST39VF512 (TSOP-I-32, 2.7-3.6V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sst::SST_SST39VF512.parse("39VF512 70-4C-WH 0216049-D").is_ok());
/// assert!(parser::sst::SST_SST39VF512.parse("39VF512 70-4C-WH 0350077-D").is_ok());
/// ```
pub static SST_SST39VF512: NomParser<GenericPart> = NomParser {
    name: "SST SST39VF512",
    f: |input| {
        tuple((
            tag("39VF512"),
            char(' '),
            tag("70-4C-WH"), // speed, durability, grade, package
            char(' '),
            tuple((year2_week2, digits(3), tag("-D"))),
        ))
        .map(|(kind, _, attrs, _, (date_code, _, _))| GenericPart {
            kind: format!("SST{kind}-{attrs}"),
            manufacturer: Some(Manufacturer::Sst),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
