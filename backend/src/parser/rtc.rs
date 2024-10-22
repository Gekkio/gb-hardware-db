// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, error::ParseError, sequence::tuple, Parser};

use super::{for_nom::year2_week2, toshiba, GenericChip, LabelParser, Manufacturer};
use crate::macros::multi_parser;

pub mod sop_8 {
    use nom::{bytes::streaming::tag, character::streaming::char, sequence::tuple, Parser};

    use super::Rtc;
    use crate::parser::{for_nom::alnum_uppers, seiko, ChipDateCode, Manufacturer, NomParser};

    /// Seiko S-3511A (SOP-8)
    ///
    /// ```
    /// use gbhwdb_backend::parser::{self, LabelParser};
    /// assert!(parser::rtc::sop_8::SEIKO_S3511A.parse("S3511 AV31 9812").is_ok());
    /// assert!(parser::rtc::sop_8::SEIKO_S3511A.parse("S3511 AVEX 2753").is_ok());
    /// ```
    pub static SEIKO_S3511A: NomParser<Rtc> = NomParser {
        name: "Seiko S-3511A",
        f: |input| {
            tuple((
                tag("S3511 AV"),
                seiko::year1,
                alnum_uppers(1),
                char(' '),
                seiko::lot_code,
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
    pub static SEIKO_S3516AE: NomParser<Rtc> = NomParser {
        name: "Seiko S-3516AE",
        f: |input| {
            tuple((
                tag("S3516 AEV"),
                seiko::year1,
                alnum_uppers(1),
                char(' '),
                seiko::lot_code,
            ))
            .map(|(_, year, _, _, _)| Rtc {
                kind: String::from("S-3516AE"),
                manufacturer: Some(Manufacturer::Seiko),
                date_code: Some(ChipDateCode::Year { year }),
            })
            .parse(input)
        },
    };
}

pub mod sop_20 {
    use nom::Parser as _;

    use super::{toshiba_tc8521a, Rtc};
    use crate::parser::{toshiba, NomParser};

    /// Toshiba TC8521AM (SOP-20)
    ///
    /// Source:
    ///   "TOSHIBA TC8521AP, TC8521AM (Real Time Clock II)"
    ///
    /// ```
    /// use gbhwdb_backend::parser::{self, LabelParser};
    /// assert!(parser::rtc::sop_20::TOSHIBA_TC8521AM.parse("T 9722HB 8521AM").is_ok());
    /// ```
    pub static TOSHIBA_TC8521AM: NomParser<Rtc> = NomParser {
        name: "Toshiba TC8521AM",
        f: |input| toshiba_tc8521a(toshiba::Package::SOP).parse(input),
    };
}

fn toshiba_tc8521a<'a, E: ParseError<&'a str>>(
    package: toshiba::Package,
) -> impl Parser<&'a str, Rtc, E> {
    tuple((
        tag("T "),
        year2_week2,
        tag("HB 8521A"),
        nom::character::streaming::char(package.code_char()),
    ))
    .map(move |(_, date_code, _, package_code)| Rtc {
        kind: format!("TC8521A{package_code}"),
        manufacturer: Some(Manufacturer::Toshiba),
        date_code: Some(date_code),
    })
}

pub type Rtc = GenericChip;

pub fn rtc_sop_8() -> &'static impl LabelParser<Rtc> {
    multi_parser!(Rtc, &sop_8::SEIKO_S3511A, &sop_8::SEIKO_S3516AE)
}

pub fn rtc_sop_20() -> &'static impl LabelParser<Rtc> {
    multi_parser!(Rtc, &sop_20::TOSHIBA_TC8521AM)
}
