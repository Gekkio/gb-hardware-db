// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::single_parser;

pub type OxyPmic = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::oxy_pmic::mitsumi_pm_c().parse("MITSUMI JAPAN 528A PM C").is_ok());
/// ```
pub fn mitsumi_pm_c() -> &'static impl LabelParser<OxyPmic> {
    single_parser!(
        OxyPmic,
        r#"^MITSUMI\ JAPAN\ ([0-9])([0-9]{2})\ ?[A-Z]\ PM\ C$"#,
        move |c| {
            Ok(OxyPmic {
                kind: "PM C".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn oxy_pmic() -> &'static impl LabelParser<OxyPmic> {
    mitsumi_pm_c()
}
