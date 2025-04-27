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
    GenericPart, Manufacturer, NomParser,
    for_nom::{alnum_uppers, digits, lines3, uppers, year1_week2},
};

/// Winbond W24257S SRAM (SOP-28, 4.5-5.5V, 256 Kibit / 32 KiB)
///
/// Source:
///     "Winbond W24257 datasheet - 32K × 8 CMOS STATIC RAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::winbond::WINBOND_W24257S.parse("Winbond W24257S-70LL 046QB202858301AC").is_ok());
/// ```
pub static WINBOND_W24257S: NomParser<GenericPart> = NomParser {
    name: "Winbond W24257S",
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
        .map(|(_, kind, (date_code, _, _, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Winbond),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Winbond W24258S SRAM (SOP-28, 2.7-5.5V, 256 Kibit / 32 KiB)
///
///
/// Source:
///     "Winbond W24258 datasheet - 32K × 8 CMOS STATIC RAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::winbond::WINBOND_W24258S.parse("Winbond W24258S-70LE 011MH200254401AA").is_ok());
/// ```
pub static WINBOND_W24258S: NomParser<GenericPart> = NomParser {
    name: "Winbond W24258S",
    f: |input| {
        lines3(
            tag("Winbond"),
            recognize((
                tag("W24258"),
                package(Package::Sop28),
                char('-'),
                tag("70"), // speed
                tag("L"),  // power
                tag("E"),  // temperature rating
            )),
            (year1_week2, uppers(2), digits(9), uppers(2)),
        )
        .map(|(_, kind, (date_code, _, _, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Winbond),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Winbond W2465S SRAM (SOP-28, 4.5-5.5V, 64 Kibit / 8 KiB)
///
/// Source:
///     "Winbond W2465 datasheet - 8K × 8 CMOS STATIC RAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::winbond::WINBOND_W2465S.parse("Winbond W2465S-70LL 140SD21331480-II1RA").is_ok());
/// assert!(parser::winbond::WINBOND_W2465S.parse("Winbond W2465S-70LL 127AD21212050-811RA").is_ok());
/// ```
pub static WINBOND_W2465S: NomParser<GenericPart> = NomParser {
    name: "Winbond W2465S",
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
        .map(|(_, kind, (date_code, _, _, _, _, _, _))| GenericPart {
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
