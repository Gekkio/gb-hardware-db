// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

pub type MgbAmp = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mgb_amp::sharp_ir3r53n().parse("AMP MGB IR3R53N 9806 a").is_ok());
/// ```
pub fn sharp_ir3r53n() -> &'static impl LabelParser<MgbAmp> {
    single_parser!(
        MgbAmp,
        r#"^AMP\ MGB\ IR3R53N\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]$"#,
        move |c| {
            Ok(MgbAmp {
                kind: "IR3R53N".to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mgb_amp::sharp_ir3r56n().parse("AMP MGB IR3R56N 0040 C").is_ok());
/// ```
pub fn sharp_ir3r56n() -> &'static impl LabelParser<MgbAmp> {
    single_parser!(
        MgbAmp,
        r#"^AMP\ MGB\ IR3R56N\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]$"#,
        move |c| {
            Ok(MgbAmp {
                kind: "IR3R56N".to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn mgb_amp() -> &'static impl LabelParser<MgbAmp> {
    multi_parser!(MgbAmp, sharp_ir3r53n(), sharp_ir3r56n())
}
