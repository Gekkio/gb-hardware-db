// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{recognize, value},
    error::ParseError,
};

use super::{
    Manufacturer, NomParser,
    for_nom::{alnum_uppers, digits, lines3, uppers, year1_week2},
    sram::Ram,
};

/// Winbond W24257 (4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::winbond::WINBOND_W24257.parse("Winbond W24257S-70LL 046QB202858301AC").is_ok());
/// ```
pub static WINBOND_W24257: NomParser<Ram> = NomParser {
    name: "Winbond W24257",
    f: |input| {
        lines3(
            tag("Winbond"),
            recognize((
                tag("W24257"),
                package(Package::Sop28),
                char('-'),
                tag("70"), // speed
                tag("LL"), // power
            )),
            (year1_week2, uppers(2), digits(9), uppers(2)),
        )
        .map(|(_, kind, (date_code, _, _, _))| Ram {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Winbond),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Winbond W24258 (2.7-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::winbond::WINBOND_W24258.parse("Winbond W24258S-70LE 011MH200254401AA").is_ok());
/// ```
pub static WINBOND_W24258: NomParser<Ram> = NomParser {
    name: "Winbond W24258",
    f: |input| {
        lines3(
            tag("Winbond"),
            recognize((
                tag("W24258"),
                package(Package::Sop28),
                char('-'),
                tag("70"),
                tag("LE"),
            )),
            (year1_week2, uppers(2), digits(9), uppers(2)),
        )
        .map(|(_, kind, (date_code, _, _, _))| Ram {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Winbond),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Winbond W2465 (4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::winbond::WINBOND_W2465.parse("Winbond W2465S-70LL 140SD21331480-II1RA").is_ok());
/// assert!(parser::winbond::WINBOND_W2465.parse("Winbond W2465S-70LL 127AD21212050-811RA").is_ok());
/// ```
pub static WINBOND_W2465: NomParser<Ram> = NomParser {
    name: "Winbond W2465",
    f: |input| {
        lines3(
            tag("Winbond"),
            recognize((
                tag("W2465"),
                package(Package::Sop28),
                char('-'),
                tag("70"), // speed
                tag("LL"), // power
            )),
            (
                year1_week2,
                uppers(2),
                digits(8),
                char('-'),
                alnum_uppers(1),
                alnum_uppers(1),
                tag("1RA"),
            ),
        )
        .map(|(_, kind, (date_code, _, _, _, _, _, _))| Ram {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Winbond),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn package<'a, E: ParseError<&'a str>>(
    package: Package,
) -> impl Parser<&'a str, Output = Package, Error = E> {
    value(package, tag(package.code()))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Sop28,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Sop28 => "S",
        }
    }
}
