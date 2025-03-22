// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser as _, bytes::streaming::tag};

use super::{
    GenericPart, Manufacturer, NomParser,
    for_nom::{alnum_uppers, digits, year1},
};
use crate::parser::PartDateCode;

/// Mitsubishi M62021P
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::mitsubishi::MITSUBISHI_M62021P.parse("2021 7Z2").is_ok());
/// ```
pub static MITSUBISHI_M62021P: NomParser<GenericPart> = NomParser {
    name: "Mitsubishi M62021P",
    f: |input| {
        (tag("2021 "), (year1, alnum_uppers(1), digits(1)))
            .map(|(_, (year, _, _))| GenericPart {
                kind: "M62021P".to_owned(),
                manufacturer: Some(Manufacturer::Mitsubishi),
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};
