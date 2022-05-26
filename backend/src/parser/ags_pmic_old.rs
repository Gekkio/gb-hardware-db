// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{year1, ChipYearWeek, LabelParser};
use crate::macros::{multi_parser, single_parser};

pub type AgsPmicOld = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ags_pmic_old::unknown().parse("S6403 CU4E0 9723").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<AgsPmicOld> {
    single_parser!(
        AgsPmicOld,
        r#"^S6403\ [[:alnum:]]{5}\ [0-9]{4}$"#,
        move |_| {
            Ok(AgsPmicOld {
                kind: "S6403".to_owned(),
                manufacturer: None,
                year: None,
                week: None,
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ags_pmic_old::unknown2().parse("9753 4862").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<AgsPmicOld> {
    single_parser!(
        AgsPmicOld,
        r#"^(9753)\ ([0-9])[[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(AgsPmicOld {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year1(&c[2])?),
                week: None,
            })
        }
    )
}

pub fn ags_pmic_old() -> &'static impl LabelParser<AgsPmicOld> {
    multi_parser!(AgsPmicOld, unknown(), unknown2())
}
