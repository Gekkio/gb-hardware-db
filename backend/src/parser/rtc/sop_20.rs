// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::Rtc;
use crate::{
    macros::single_parser,
    parser::{week2, year2, ChipDateCode, LabelParser, Manufacturer},
};

/// Toshiba TC8521AM (SOP-20)
///
/// Source:
///   "TOSHIBA TC8521AP, TC8521AM (Real Time Clock II)"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::sop_20::toshiba_tc8521am().parse("T 9722HB 8521AM").is_ok());
/// ```
pub fn toshiba_tc8521am() -> &'static impl LabelParser<Rtc> {
    single_parser!(
        Rtc,
        r#"^T\ (?<year>[0-9]{2})(?<week>[0-9]{2})HB\ (?<kind>8521A)(?<package>M)$"#,
        move |c| {
            Ok(Rtc {
                kind: format!("TC{}", &c["kind"]),
                manufacturer: Some(Manufacturer::Toshiba),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}
