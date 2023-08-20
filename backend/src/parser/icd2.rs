// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, year2, ChipYearWeek, LabelParser};
use crate::macros::{multi_parser, single_parser};

pub type Icd2 = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::icd2::unknown().parse("Nintendo ICD2-R 435 129").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<Icd2> {
    single_parser!(
        Icd2,
        r#"^Nintendo\ (ICD2-[NR])\ ([0-9])([0-9]{2})\ [0-9]{3}$"#,
        move |c| {
            Ok(Icd2 {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::icd2::unknown2().parse("Nintendo ICD2-N 9415KX226 D93115").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<Icd2> {
    single_parser!(
        Icd2,
        r#"^Nintendo\ (ICD2-[NR])\ ([0-9]{2})\ ?([0-9]{2})[A-Z]{2}[0-9]{3}\ (D93115|D93128)$"#,
        move |c| {
            Ok(Icd2 {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

pub fn icd2() -> &'static impl LabelParser<Icd2> {
    multi_parser!(Icd2, unknown(), unknown2())
}
