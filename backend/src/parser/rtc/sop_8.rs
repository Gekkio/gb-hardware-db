// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::Rtc;
use crate::{
    macros::single_parser,
    parser::{seiko_year1, ChipDateCode, LabelParser, Manufacturer},
};

/// Seiko S-3511A (SOP-8)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::sop_8::seiko_s3511a().parse("S3511 AV31 9812").is_ok());
/// assert!(parser::rtc::sop_8::seiko_s3511a().parse("S3511 AVEX 2753").is_ok());
/// ```
pub fn seiko_s3511a() -> &'static impl LabelParser<Rtc> {
    single_parser!(
        Rtc,
        r#"^S3511\ (?<revision>A)V(?<year>[[:alnum:]])[[:alnum:]]\ [0-9]{4}$"#,
        move |c| {
            Ok(Rtc {
                kind: format!("S-3511{revision}", revision = &c["revision"]),
                manufacturer: Some(Manufacturer::Seiko),
                date_code: Some(ChipDateCode::Year {
                    year: seiko_year1(&c["year"])?,
                }),
            })
        },
    )
}

/// Seiko S-3516AE (SOP-8)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::sop_8::seiko_s3516ae().parse("S3516 AEV42 7505").is_ok());
/// ```
pub fn seiko_s3516ae() -> &'static impl LabelParser<Rtc> {
    single_parser!(
        Rtc,
        r#"^S3516\ (?<revision>AE)V(?<year>[[:alnum:]])[[:alnum:]]\ [0-9]{4}$"#,
        move |c| {
            Ok(Rtc {
                kind: format!("S-3516{revision}", revision = &c["revision"]),
                manufacturer: Some(Manufacturer::Seiko),
                date_code: Some(ChipDateCode::Year {
                    year: seiko_year1(&c["year"])?,
                }),
            })
        },
    )
}
