// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::{macros::single_parser, parser::Manufacturer};

pub type CgbSoc = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::cgb_soc::cpu_cgb().parse("CPU CGB B Ⓜ © 1998 Nintendo JAPAN 9842 I").is_ok());
/// ```
pub fn cpu_cgb() -> &'static impl LabelParser<CgbSoc> {
    single_parser!(
        CgbSoc,
        r#"^(CPU\ CGB(\ [A-E])?)\ Ⓜ\ ©\ (1998|2000)\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbSoc {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

pub fn cgb_soc() -> &'static impl LabelParser<CgbSoc> {
    cpu_cgb()
}
