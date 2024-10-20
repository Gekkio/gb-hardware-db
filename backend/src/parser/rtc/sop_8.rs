// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    bytes::streaming::tag,
    character::streaming::{char, satisfy},
    multi::count,
    sequence::tuple,
    Parser,
};

use super::Rtc;
use crate::parser::{alnum_upper, seiko_year1, ChipDateCode, Manufacturer, NomFnParser};

/// Seiko S-3511A (SOP-8)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::sop_8::SEIKO_S3511A.parse("S3511 AV31 9812").is_ok());
/// assert!(parser::rtc::sop_8::SEIKO_S3511A.parse("S3511 AVEX 2753").is_ok());
/// ```
pub static SEIKO_S3511A: NomFnParser<Rtc> = NomFnParser {
    name: "Seiko S-3511A",
    f: |input| {
        tuple((
            tag("S3511 AV"),
            seiko_year1,
            alnum_upper,
            char(' '),
            count(satisfy(|c| c.is_ascii_digit()), 4),
        ))
        .map(|(_, year, _, _, _)| Rtc {
            kind: String::from("S-3511A"),
            manufacturer: Some(Manufacturer::Seiko),
            date_code: Some(ChipDateCode::Year { year }),
        })
        .parse(input)
    },
};

/// Seiko S-3516AE (SOP-8)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::rtc::sop_8::SEIKO_S3516AE.parse("S3516 AEV42 7505").is_ok());
/// ```
pub static SEIKO_S3516AE: NomFnParser<Rtc> = NomFnParser {
    name: "Seiko S-3516AE",
    f: |input| {
        tuple((
            tag("S3516 AEV"),
            seiko_year1,
            alnum_upper,
            char(' '),
            count(satisfy(|c| c.is_ascii_digit()), 4),
        ))
        .map(|(_, year, _, _, _)| Rtc {
            kind: String::from("S-3516AE"),
            manufacturer: Some(Manufacturer::Seiko),
            date_code: Some(ChipDateCode::Year { year }),
        })
        .parse(input)
    },
};
