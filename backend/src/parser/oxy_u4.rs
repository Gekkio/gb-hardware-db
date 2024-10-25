// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, GenericPart, LabelParser};
use crate::{macros::single_parser, parser::PartDateCode};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::oxy_u4::unknown().parse("AKV 522").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<GenericPart> {
    single_parser!(GenericPart, r#"^AKV\ ([0-9])([0-9]{2})$"#, move |c| {
        Ok(GenericPart {
            kind: "AKV".to_owned(),
            manufacturer: None,
            date_code: Some(PartDateCode::YearWeek {
                year: year1(&c[1])?,
                week: week2(&c[2])?,
            }),
        })
    })
}

pub fn oxy_u4() -> &'static impl LabelParser<GenericPart> {
    unknown()
}
