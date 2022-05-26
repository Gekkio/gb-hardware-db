// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::macros::single_parser;

pub type GbsDol = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gbs_dol::unknown().parse("Nintendo GBS-DOL 011 0623L3001").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<GbsDol> {
    single_parser!(
        GbsDol,
        r#"^Nintendo\ GBS-DOL\ 011\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(GbsDol {
                kind: "GBS-DOL".to_owned(),
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn gbs_dol() -> &'static impl LabelParser<GbsDol> {
    unknown()
}
