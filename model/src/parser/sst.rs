// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser as _, bytes::streaming::tag};

use crate::parser::{
    Manufacturer, NomParser,
    for_nom::{digits, year2_week2},
};

use super::{GenericPart, for_nom::lines3};

/// SST SST39VF512 (TSOP-I-32, 2.7-3.6V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sst::SST_SST39VF512.parse("39VF512 70-4C-WH 0216049-D").is_ok());
/// assert!(parser::sst::SST_SST39VF512.parse("39VF512 70-4C-WH 0350077-D").is_ok());
/// ```
pub static SST_SST39VF512: NomParser<GenericPart> = NomParser {
    name: "SST SST39VF512",
    f: |input| {
        lines3(
            tag("39VF512"),
            tag("70-4C-WH"), // speed, durability, grade, package
            (year2_week2, digits(3), tag("-D")),
        )
        .map(|(kind, attrs, (date_code, _, _))| GenericPart {
            kind: format!("SST{kind}-{attrs}"),
            manufacturer: Some(Manufacturer::Sst),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
