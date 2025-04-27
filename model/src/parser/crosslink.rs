// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    IResult, Parser as _, bytes::streaming::tag, combinator::opt, error::ParseError,
    sequence::terminated,
};

use super::{
    GenericPart, Manufacturer, NomParser, PartDateCode,
    for_nom::{alnum_uppers, digits, lines4, uppers, week2, year1},
};

/// Crosslink LH52A64N (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::crosslink::CROSSLINK_LH52A64N.parse("LH52A64N-YL Xlink JAPAN H432 0U C").is_ok());
/// ```
pub static CROSSLINK_LH52A64N: NomParser<GenericPart> = NomParser {
    name: "Crosslink LH52A64N",
    f: |input| {
        lines4(
            tag("LH52A64N-YL"),
            tag("Xlink"),
            tag("JAPAN"),
            terminated(
                date_code,
                (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(1)),
            ),
        )
        .map(|(kind, _, _, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Crosslink),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Crosslink LH5268AN (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::crosslink::CROSSLINK_LH5268AN.parse("LH5268ANF-10YLL Xlink JAPAN H429 0Y BB").is_ok());
/// ```
pub static CROSSLINK_LH5268AN: NomParser<GenericPart> = NomParser {
    name: "Crosslink LH5268AN",
    f: |input| {
        lines4(
            tag("LH5268ANF-10YLL"),
            tag("Xlink"),
            tag("JAPAN"),
            terminated(
                date_code,
                (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(2)),
            ),
        )
        .map(|(kind, _, _, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Crosslink),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    (tag("H"), year1, opt(tag(" ")), week2)
        .map(|(_, year, _, week)| PartDateCode::YearWeek { year, week })
        .parse(input)
}
