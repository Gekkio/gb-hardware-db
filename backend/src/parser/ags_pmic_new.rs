// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, AgbAmp, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::single_parser;

pub type AgsPmicNew = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ags_pmic_new::mitsumi_pm().parse("MITSUMI JAPAN 602E PM B3").is_ok());
/// ```
pub fn mitsumi_pm() -> &'static impl LabelParser<AgsPmicNew> {
    // FIXME: Not really an amplifier
    single_parser!(
        AgbAmp,
        r#"^MITSUMI\ JAPAN\ ([0-9])([0-9]{2})[A-Z]\ (PM\ B[0-9])$"#,
        move |c| {
            Ok(AgbAmp {
                kind: c[3].to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn ags_pmic_new() -> &'static impl LabelParser<AgsPmicNew> {
    mitsumi_pm()
}
