// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::single_parser;

pub type Flash = ChipYearWeek;

/// Macronix MX29F008 flash
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::macronix_mx29f008().parse("E991012 29F008TC-14 21534 TAIWAN").is_ok());
/// ```
pub fn macronix_mx29f008() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}\ (29F008[A-Z]{2}-[0-9]{2})\ [0-9]{5}\ TAIWAN$"#,
        move |c| {
            Ok(Flash {
                kind: format!("MX{}", &c[3]),
                manufacturer: Some(Manufacturer::Macronix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn flash() -> &'static impl LabelParser<Flash> {
    macronix_mx29f008()
}
