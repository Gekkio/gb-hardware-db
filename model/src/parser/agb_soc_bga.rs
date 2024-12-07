// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, GenericPart, LabelParser};
use crate::{macros::single_parser, parser::PartDateCode};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::agb_soc_bga::agb_soc_bga().parse("0529 2m CPU AGB E Ⓜ © 2004 Nintendo JAPAN ARM").is_ok());
/// ```
pub fn agb_soc_bga() -> &'static impl LabelParser<GenericPart> {
    single_parser!(
        GenericPart,
        r#"^([0-9]{2})([0-9]{2})\ 2m\ (CPU\ AGB\ E)\ Ⓜ\ ©\ 2004\ Nintendo\ JAPAN\ ARM$"#,
        move |c| {
            Ok(GenericPart {
                kind: c[3].to_owned(),
                manufacturer: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[1])?,
                    week: week2(&c[2])?,
                }),
            })
        },
    )
}
