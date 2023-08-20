// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::single_parser;

pub type Accelerometer = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::accelerometer::accelerometer().parse("2738109451 0028 ADXL202JQC").is_ok());
/// ```
pub fn analog_adxl202jqc() -> &'static impl LabelParser<ChipYearWeek> {
    single_parser!(
        ChipYearWeek,
        r#"^[0-9]{10}\ ([0-9]{2})([0-9]{2})\ ADXL202JQC$"#,
        move |c| {
            Ok(ChipYearWeek {
                kind: "ADXL202JQC".to_owned(),
                manufacturer: Some(Manufacturer::Analog),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

pub fn accelerometer() -> &'static impl LabelParser<Accelerometer> {
    analog_adxl202jqc()
}
