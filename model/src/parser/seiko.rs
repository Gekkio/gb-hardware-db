// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    IResult, Parser,
    bytes::streaming::tag,
    character::{
        complete::one_of,
        streaming::{anychar, char},
    },
    combinator::{map_opt, recognize},
    error::ParseError,
    sequence::preceded,
};

use super::{
    GenericPart, Manufacturer, NomParser, PartDateCode, Year,
    for_nom::{alnum_uppers, digits, lines3, month1_123xyz},
};

/// Seiko S-3511A RTC (SOP-8)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::seiko::SEIKO_S3511A.parse("S3511 AV31 9812").is_ok());
/// assert!(parser::seiko::SEIKO_S3511A.parse("S3511 AVEX 2753").is_ok());
/// ```
pub static SEIKO_S3511A: NomParser<GenericPart> = NomParser {
    name: "Seiko S-3511A",
    f: |input| {
        lines3(tag("S3511"), preceded(tag("AV"), date_code), lot_code)
            .map(|(_, date_code, _)| GenericPart {
                kind: String::from("S-3511A"),
                manufacturer: Some(Manufacturer::Seiko),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Seiko S-3516AE RTC (SOP-8)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::seiko::SEIKO_S3516AE.parse("S3516 AEV42 7505").is_ok());
/// ```
pub static SEIKO_S3516AE: NomParser<GenericPart> = NomParser {
    name: "Seiko S-3516AE",
    f: |input| {
        lines3(tag("S3516"), preceded(tag("AEV"), date_code), lot_code)
            .map(|(_, date_code, _)| GenericPart {
                kind: String::from("S-3516AE"),
                manufacturer: Some(Manufacturer::Seiko),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Seiko S-6403
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::seiko::SEIKO_S6403.parse("S6403 CU4E0 9723").is_ok());
/// ```
pub static SEIKO_S6403: NomParser<GenericPart> = NomParser {
    name: "Seiko S-6403",
    f: |input| {
        lines3(
            tag("S6403"),
            (one_of("AC"), char('U'), year1, alnum_uppers(1), digits(1)),
            lot_code,
        )
        .map(|(_, (revision, _, year, _, _), _)| GenericPart {
            kind: format!("S-6403{revision}"),
            manufacturer: Some(Manufacturer::Seiko),
            date_code: Some(PartDateCode::Year { year }),
        })
        .parse(input)
    },
};

/// Seiko S-6960E
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::seiko::SEIKO_S6960E.parse("S6960 E-U2Z C700").is_ok());
/// assert!(parser::seiko::SEIKO_S6960E.parse("S6960 E-U2X C410").is_ok());
/// ```
pub static SEIKO_S6960E: NomParser<GenericPart> = NomParser {
    name: "Seiko S-6960E",
    f: |input| {
        lines3(tag("S6960"), preceded(tag("E-U"), date_code), lot_code)
            .map(|(_, date_code, _)| GenericPart {
                kind: String::from("S-6960E"),
                manufacturer: Some(Manufacturer::Seiko),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    year1
        .and(month1_123xyz)
        .map(|(year, month)| PartDateCode::YearMonth { year, month })
        .parse(input)
}

fn year1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Year, E> {
    map_opt(anychar, |ch| match ch {
        '0' => Some(Year::Partial(0)),
        '1' | 'A' => Some(Year::Partial(1)),
        '2' | 'B' => Some(Year::Partial(2)),
        '3' | 'C' => Some(Year::Partial(3)),
        '4' | 'D' => Some(Year::Partial(4)),
        '5' | 'E' => Some(Year::Partial(5)),
        '6' | 'F' => Some(Year::Partial(6)),
        '7' | 'G' => Some(Year::Partial(7)),
        '8' | 'H' => Some(Year::Partial(8)),
        '9' | 'J' => Some(Year::Partial(9)),
        _ => None,
    })
    .parse(input)
}

fn lot_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(alnum_uppers(1).and(digits(3))).parse(input)
}
