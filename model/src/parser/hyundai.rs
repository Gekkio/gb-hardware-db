// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    IResult, Parser, branch::alt, bytes::streaming::tag, character::streaming::char,
    combinator::value, error::ParseError,
};

use super::{
    PartDateCode,
    for_nom::{uppers, year2_week2},
    sram::Ram,
};
use crate::parser::{Manufacturer, NomParser};

/// Hyundai HY628100 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hyundai::HYUNDAI_HY628100.parse("HYUNDAI KOREA HY628100B 0041A LLG-70").is_ok());
/// ```
pub static HYUNDAI_HY628100: NomParser<Ram> = NomParser {
    name: "Hyundai HY628100",
    f: |input| {
        (
            tag("HYUNDAI KOREA "),
            alt((tag("HY628100A"), tag("HY628100B"), tag("HY628100"))),
            char(' '),
            date_code.and(process_code),
            char(' '),
            (
                alt((tag("LL"), tag("L"))), // power
                package(Package::Sop32),    // package
                char('-'),
                alt((tag("50"), tag("55"), tag("70"), tag("85"))), // speed
            ),
        )
            .map(
                |(_, kind, _, (date_code, _), _, (power, package, _, speed))| Ram {
                    kind: format!("{kind}{power}{package}-{speed}", package = package.code()),
                    manufacturer: Some(Manufacturer::Hyundai),
                    date_code: Some(date_code),
                },
            )
            .parse(input)
    },
};

/// Hyundai HY6264 (SOP-28, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hyundai::HYUNDAI_HY6264.parse("HYUNDAI HY6264ALLJ-10 9327B KOREA").is_ok());
/// assert!(parser::hyundai::HYUNDAI_HY6264.parse("HY6264A LLJ-10 9902B KOREA").is_ok());
/// ```
pub static HYUNDAI_HY6264: NomParser<Ram> = NomParser {
    name: "Hyundai HY6264",
    f: |input| {
        // 1992-1994
        let old_format = (
            tag("HYUNDAI "),
            (
                alt((tag("HY6264A"), tag("HY6264"))),
                alt((tag("LL"), tag("L"))), // power
                package(Package::Sop28),    // package
                char('-'),
                alt((tag("70"), tag("85"), tag("10"), tag("12"), tag("15"))), // speed
            ),
            char(' '),
            date_code.and(process_code),
            tag(" KOREA"),
        )
            .map(
                |(_, (kind, power, package, _, speed), _, (date_code, _), _)| Ram {
                    kind: format!("{kind}{power}{package}-{speed}", package = package.code()),
                    manufacturer: Some(Manufacturer::Hyundai),
                    date_code: Some(date_code),
                },
            );
        // 1994-
        let new_format = (
            alt((tag("HY6264A"), tag("HY6264"))),
            char(' '),
            (
                alt((tag("LL"), tag("L"))), // power
                package(Package::Sop28),    // package
                char('-'),
                alt((tag("70"), tag("85"), tag("10"), tag("12"), tag("15"))), // speed
            ),
            char(' '),
            date_code.and(process_code),
            tag(" KOREA"),
        )
            .map(
                |(kind, _, (power, package, _, speed), _, (date_code, _), _)| Ram {
                    kind: format!("{kind}{power}{package}-{speed}", package = package.code()),
                    manufacturer: Some(Manufacturer::Hyundai),
                    date_code: Some(date_code),
                },
            );
        alt((new_format, old_format)).parse(input)
    },
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    year2_week2(input)
}

fn process_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    uppers(1).parse(input)
}

fn package<'a, E: ParseError<&'a str>>(
    package: Package,
) -> impl Parser<&'a str, Output = Package, Error = E> {
    value(package, tag(package.code()))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Sop28,
    Sop32,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Sop28 => "J",
            Package::Sop32 => "G",
        }
    }
}
