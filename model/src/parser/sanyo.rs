// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    IResult, Parser as _,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
    error::ParseError,
    sequence::separated_pair,
};

use super::{
    GenericPart,
    for_nom::{lines2, lines3},
};
use crate::parser::{
    Manufacturer, NomParser, PartDateCode,
    for_nom::{alnum_uppers, digits, month1_abc, uppers, year1},
};

/// Sanyo LE26FV10 flash (TSOP-I-32, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sanyo::SANYO_LE26FV10.parse("LE26FV10N1TS -10 3MU50").is_ok());
/// assert!(parser::sanyo::SANYO_LE26FV10.parse("LE26FV10N1TS -10 4DU2A").is_ok());
/// ```
pub static SANYO_LE26FV10: NomParser<GenericPart> = NomParser {
    name: "Sanyo LE26FV10",
    f: |input| {
        lines2(
            recognize(tag("LE26FV10N1").and(tag("TS"))),
            separated_pair(
                tag("-10"), // speed
                char(' '),
                (date_code, uppers(1), digits(1), alnum_uppers(1)),
            ),
        )
        .map(|(kind, (speed, (date_code, _, _, _)))| GenericPart {
            kind: format!("{kind}{speed}"),
            manufacturer: Some(Manufacturer::Sanyo),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sanyo LC35256 SRAM (SOP-28, 2.7-5.5V, 256 Kibit / 32 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sanyo::SANYO_LC35256.parse("SANYO LC35256DM-70W JAPAN 0EUPG").is_ok());
/// assert!(parser::sanyo::SANYO_LC35256.parse("SANYO LC35256FM-70U JAPAN 0LK5G").is_ok());
/// ```
pub static SANYO_LC35256: NomParser<GenericPart> = NomParser {
    name: "Sanyo LC35256",
    f: |input| {
        lines3(
            tag("SANYO"),
            (
                recognize(tag("LC35256").and(opt(one_of("ABCDEF")))),
                char('M'),
                char('-'),
                tag("70"),
                alnum_uppers(1),
            ),
            separated_pair(tag("JAPAN"), char(' '), date_code.and(alnum_uppers(3))),
        )
        .map(
            |(_, (kind, package, _, speed, _), (_, (date_code, _)))| GenericPart {
                kind: format!("{kind}{package}-{speed}"),
                manufacturer: Some(Manufacturer::Sanyo),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};

/// Sanyo LC3564 SRAM (SOP-28, 2.7-5.5V, 64 Kibit / 8 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sanyo::SANYO_LC3564.parse("SANYO LC3564BM-70 JAPAN 9MUBG").is_ok());
/// ```
pub static SANYO_LC3564: NomParser<GenericPart> = NomParser {
    name: "Sanyo LC3564",
    f: |input| {
        lines3(
            tag("SANYO"),
            (
                recognize(tag("LC3564").and(opt(one_of("AB")))),
                char('M'),
                char('-'),
                tag("70"),
            ),
            separated_pair(tag("JAPAN"), char(' '), date_code.and(alnum_uppers(3))),
        )
        .map(
            |(_, (kind, package, _, speed), (_, (date_code, _)))| GenericPart {
                kind: format!("{kind}{package}-{speed}"),
                manufacturer: Some(Manufacturer::Sanyo),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    year1
        .and(month1_abc)
        .map(|(year, month)| PartDateCode::YearMonth { year, month })
        .parse(input)
}
