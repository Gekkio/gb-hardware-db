// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::{macros::single_parser, parser::Manufacturer};

pub type CgbReg = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::cgb_reg::sharp_ir3e06n().parse("CGB-REG IR3E06N 9839 C").is_ok());
/// ```
pub fn sharp_ir3e06n() -> &'static impl LabelParser<CgbReg> {
    single_parser!(
        CgbReg,
        r#"^CGB-REG\ IR3E06N\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbReg {
                kind: "IR3E06N".to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

pub fn cgb_reg() -> &'static impl LabelParser<CgbReg> {
    sharp_ir3e06n()
}
