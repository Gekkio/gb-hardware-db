// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{month2, year1, LabelParser, ParsedData, Year};
use crate::{macros::single_parser, time::Month};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgStamp {
    pub year: Option<Year>,
    pub month: Option<Month>,
}

impl ParsedData for DmgStamp {}

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::dmg_stamp::dmg_stamp().parse("010 23").is_ok());
/// assert!(parser::dmg_stamp::dmg_stamp().parse("903-22").is_ok());
/// assert!(parser::dmg_stamp::dmg_stamp().parse("709.3901").is_ok());
/// assert!(parser::dmg_stamp::dmg_stamp().parse("202-0007").is_ok());
/// ```
pub fn dmg_stamp() -> &'static impl LabelParser<DmgStamp> {
    single_parser!(
        DmgStamp,
        r#"^([0-9])([0-9]{2})[-\ .][0-9-]{2,4}Y?$"#,
        move |c| {
            Ok(DmgStamp {
                year: Some(year1(&c[1])?),
                month: Some(month2(&c[2])?),
            })
        }
    )
}
