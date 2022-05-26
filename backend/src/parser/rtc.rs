// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::{macros::single_parser, parser::Manufacturer};

pub type Rtc = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::toshiba_tc8521am().parse("T 9722HB 8521AM").is_ok());
/// ```
pub fn toshiba_tc8521am() -> &'static impl LabelParser<Rtc> {
    single_parser!(Rtc, r#"^T\ ([0-9]{2})([0-9]{2})HB\ 8521AM$"#, move |c| {
        Ok(Rtc {
            kind: "TC8521AM".to_owned(),
            manufacturer: Some(Manufacturer::Toshiba),
            year: Some(year2(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    },)
}

pub fn rtc() -> &'static impl LabelParser<Rtc> {
    toshiba_tc8521am()
}
