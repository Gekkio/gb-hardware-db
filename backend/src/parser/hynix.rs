// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize, value},
    error::ParseError,
    sequence::tuple,
    IResult, Parser,
};

use super::{
    for_nom::{uppers, year2_week2},
    sram::Ram,
    PartDateCode,
};
use crate::parser::{Manufacturer, NomParser};

/// Hynix HY62LF16206 (TSOP-I-48, 2.3-2.7V)
///
/// Source:
///   "hynix HY62LF16206A-LT12C 128kx16bit full CMOS SRAM"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_HY62LF16206.parse("Hynix KOREA HY62LF16206A 0223A LT12C").is_ok());
/// ```
pub static HYNIX_HY62LF16206: NomParser<Ram> = NomParser {
    name: "Hynix HY62LF16206",
    f: |input| {
        tuple((
            tag("Hynix KOREA "),
            recognize(tag("HY62LF16206").and(opt(one_of("AB")))),
            char(' '),
            date_code.and(process_code),
            char(' '),
            tuple((
                tag("L"),  // power
                tag("T"),  // package
                tag("12"), // speed
                tag("C"),  // temperature
            )),
        ))
        .map(
            |(_, kind, _, (date_code, _), _, (power, package, speed, temp))| Ram {
                kind: format!("{kind}-{power}{package}{speed}{temp}"),
                manufacturer: Some(Manufacturer::Hynix),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};

/// Hynix HY62WT08081 (SOP-28, 2.7-5.5V)
///
/// Source:
///   "hynix HY62WT08081E Series 32Kx8bit CMOS SRAM"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_HY62WT08081.parse("hynix 0231A HY62WT081ED70C KOREA").is_ok());
/// ```
pub static HYNIX_HY62WT08081: NomParser<Ram> = NomParser {
    name: "Hynix HY62WT08081",
    f: |input| {
        tuple((
            tag("hynix "),
            date_code.and(process_code),
            char(' '),
            tuple((
                recognize(value("HY62WT08081", tag("HY62WT081")).and(opt(one_of("ABCDE")))),
                alt((tag("L"), tag("D"))),           // power
                alt((tag("50"), tag("70"))),         // speed
                alt((tag("C"), tag("E"), tag("I"))), // temperature
            )),
            tag(" KOREA"),
        ))
        .map(
            |(_, (date_code, _), _, (kind, power, speed, temp), _)| Ram {
                kind: format!("{kind}{power}{speed}{temp}"),
                manufacturer: Some(Manufacturer::Hynix),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    year2_week2(input)
}

fn process_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &str, E> {
    uppers(1).parse(input)
}
