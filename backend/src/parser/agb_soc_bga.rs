// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::macros::single_parser;

pub type AgbSoc = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_soc_bga::agb_soc_bga().parse("0529 2m CPU AGB E Ⓜ © 2004 Nintendo JAPAN ARM").is_ok());
/// ```
pub fn agb_soc_bga() -> &'static impl LabelParser<AgbSoc> {
    single_parser!(
        AgbSoc,
        r#"^([0-9]{2})([0-9]{2})\ 2m\ (CPU\ AGB\ E)\ Ⓜ\ ©\ 2004\ Nintendo\ JAPAN\ ARM$"#,
        move |c| {
            Ok(AgbSoc {
                kind: c[3].to_owned(),
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}
