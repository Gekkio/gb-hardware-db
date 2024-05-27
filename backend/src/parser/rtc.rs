// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::{
    macros::{multi_parser, single_parser},
    parser::Manufacturer,
};

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

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::seiko_s3511().parse("S3511 AV31 9812").is_ok());
/// assert!(parser::rtc::seiko_s3511().parse("S3511 AVEX 2753").is_ok());
/// ```
pub fn seiko_s3511() -> &'static impl LabelParser<Rtc> {
    single_parser!(Rtc, r#"^S3511\ AV[[:alnum:]]{2}\ [0-9]{4}$"#, move |_| {
        Ok(Rtc {
            kind: "S-3511".to_owned(),
            manufacturer: Some(Manufacturer::Seiko),
            year: None,
            week: None,
        })
    },)
}

pub fn rtc() -> &'static impl LabelParser<Rtc> {
    multi_parser!(Rtc, toshiba_tc8521am(), seiko_s3511(),)
}
