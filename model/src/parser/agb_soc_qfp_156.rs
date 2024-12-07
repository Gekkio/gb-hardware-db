// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, LabelParser};
use crate::{
    macros::single_parser,
    parser::{GenericPart, PartDateCode},
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::agb_soc_qfp_156::agb_soc_qfp_156().parse("CPU AGB B E Ⓜ © 2002 Nintendo JAPAN ARM 0602 UB").is_ok());
/// ```
pub fn agb_soc_qfp_156() -> &'static impl LabelParser<GenericPart> {
    single_parser!(
        GenericPart,
        r#"^(CPU\ AGB\ B(\ E)?)\ Ⓜ\ ©\ 2002\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(GenericPart {
                kind: c[1].to_owned(),
                manufacturer: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[3])?,
                    week: week2(&c[4])?,
                }),
            })
        },
    )
}
