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
/// assert!(parser::oxy_u5::unknown().parse("CP6465 B 02 KOR0531 635963").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<GenericPart> {
    single_parser!(
        GenericPart,
        r#"^CP6465\ B\ 0[0-9]\ KOR([0-9]{2})([0-9]{2})\ [0-9]{6}$"#,
        move |c| {
            Ok(GenericPart {
                kind: "CP6465".to_owned(),
                manufacturer: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[1])?,
                    week: week2(&c[2])?,
                }),
            })
        },
    )
}

pub fn oxy_u5() -> &'static impl LabelParser<GenericPart> {
    unknown()
}
