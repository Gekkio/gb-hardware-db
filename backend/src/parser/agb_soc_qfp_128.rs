// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::macros::single_parser;

pub type AgbSoc = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_soc_qfp_128::agb_soc_qfp_128().parse("CPU AGB Ⓜ © 2000 Nintendo JAPAN ARM 0104 I").is_ok());
/// ```
pub fn agb_soc_qfp_128() -> &'static impl LabelParser<AgbSoc> {
    single_parser!(
        AgbSoc,
        r#"^(CPU\ AGB(\ A(\ E)?)?)\ Ⓜ\ ©\ 2000\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbSoc {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}
