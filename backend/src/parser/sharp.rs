// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{opt, value},
    error::ParseError,
    sequence::tuple,
    Parser,
};

use super::{
    for_nom::{alphas, year2_week2},
    GenericPart, Manufacturer, NomParser,
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3E02.parse("DMG-REG IR3E02 9527 CB").is_ok());
/// assert!(parser::sharp::SHARP_IR3E02.parse("DMG-REG IR3E02 9820 n").is_ok());
/// assert!(parser::sharp::SHARP_IR3E02.parse("DMG-REG IR3E02 9024 J").is_ok());
/// ```
pub static SHARP_IR3E02: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3E02",
    f: |input| ir3_old("DMG-REG", "IR3E02").parse(input),
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3E06.parse("CGB-REG IR3E06N 9839 C").is_ok());
/// assert!(parser::sharp::SHARP_IR3E06.parse("CGB-REG IR3E06N 0046 A").is_ok());
/// ```
pub static SHARP_IR3E06: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3E06",
    f: |input| ir3("CGB-REG", "IR3E06", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0104 C").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0141 K").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0204 d").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N AA24 A").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0223 B").is_ok());
/// ```
pub static SHARP_IR3E09: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3E09",
    f: |input| ir3("AGB-REG", "IR3E09", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R40.parse("DMG-AMP IR3R40 9222 AA").is_ok());
/// assert!(parser::sharp::SHARP_IR3R40.parse("DMG-AMP IR3R40 8909 A").is_ok());
/// ```
pub static SHARP_IR3R40: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R40",
    f: |input| ir3_old("DMG-AMP", "IR3R40").parse(input),
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R53.parse("AMP MGB IR3R53N 9806 a").is_ok());
/// assert!(parser::sharp::SHARP_IR3R53.parse("AMP MGB IR3R53N 9724 C").is_ok());
/// ```
pub static SHARP_IR3R53: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R53",
    f: |input| ir3("AMP MGB", "IR3R53", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R56.parse("AMP MGB IR3R56N 0046 A").is_ok());
/// assert!(parser::sharp::SHARP_IR3R56.parse("AMP MGB IR3R56N 0040 C").is_ok());
/// ```
pub static SHARP_IR3R56: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R56",
    f: |input| ir3("AMP MGB", "IR3R56", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R60.parse("AMP AGB IR3R60N 0103 a").is_ok());
/// assert!(parser::sharp::SHARP_IR3R60.parse("AMP AGB IR3R60N 0240 N").is_ok());
/// ```
pub static SHARP_IR3R60: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R60",
    f: |input| ir3("AMP AGB", "IR3R60", Package::Ssop18).parse(input),
};

fn ir3<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    kind: &'static str,
    pkg: Package,
) -> impl Parser<&'a str, GenericPart, E> {
    tuple((
        tag(prefix),
        char(' '),
        tag(kind),
        package(pkg),
        char(' '),
        year2_week2,
        char(' '),
        alphas(1),
    ))
    .map(|(_, _, kind, package, _, date_code, _, _)| GenericPart {
        kind: format!("{kind}{package}", package = package.code()),
        manufacturer: Some(Manufacturer::Sharp),
        date_code: Some(date_code),
    })
}

fn ir3_old<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    kind: &'static str,
) -> impl Parser<&'a str, GenericPart, E> {
    tuple((
        tag(prefix),
        char(' '),
        tag(kind),
        char(' '),
        year2_week2,
        char(' '),
        alphas(1),
        opt(nom::character::complete::satisfy(|c| {
            c.is_ascii_uppercase()
        })),
    ))
    .map(|(_, _, kind, _, date_code, _, _, _)| GenericPart {
        kind: String::from(kind),
        manufacturer: Some(Manufacturer::Sharp),
        date_code: Some(date_code),
    })
}

fn package<'a, E: ParseError<&'a str>>(package: Package) -> impl Parser<&'a str, Package, E> {
    value(package, tag(package.code()))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Ssop18,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Ssop18 => "N",
        }
    }
}
