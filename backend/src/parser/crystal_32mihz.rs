// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{kds_month, year1, Crystal, LabelParser, Manufacturer};
use crate::macros::single_parser;

const FREQUENCY: u32 = 33_554_432;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_32mihz::kinseki_kss30().parse("33WKSS6DT").is_ok());
/// ```
pub fn kinseki_kss30() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^33WKSS([0-9])([A-Z])T$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}
pub fn crystal_32mihz() -> &'static impl LabelParser<Crystal> {
    kinseki_kss30()
}
