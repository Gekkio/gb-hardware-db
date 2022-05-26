// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, LabelParser, Year};
use crate::{macros::single_parser, time::Week};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbStamp {
    pub year: Option<Year>,
    pub week: Option<Week>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::cgb_stamp::cgb_stamp().parse("218-2221").is_ok());
/// ```
pub fn cgb_stamp() -> &'static impl LabelParser<CgbStamp> {
    single_parser!(
        CgbStamp,
        r#"^([0-9]{2})([0-9])[-\ .X]?[0-9]{2,4}Y?$"#,
        move |c| {
            Ok(CgbStamp {
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[1])?),
            })
        }
    )
}
