// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt, bytes::streaming::tag, character::streaming::char, combinator::opt,
    sequence::tuple, Parser as _,
};

use super::{
    for_nom::{alnum_uppers, uppers, year1, year1_week2},
    GenericPart, Manufacturer, NomParser,
};
use crate::parser::PartDateCode;

/// Mitsumi MM1026A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_MM1026A.parse("843 26A").is_ok());
/// assert!(parser::mitsumi::MITSUMI_MM1026A.parse("1L51 26A").is_ok());
/// ```
pub static MITSUMI_MM1026A: NomParser<GenericPart> = NomParser {
    name: "Mitsumi MM1026A",
    f: |input| {
        tuple((year1, alt((alnum_uppers(3), alnum_uppers(2))), tag(" 26A")))
            .map(|(year, _, _)| GenericPart {
                kind: "MM1026A".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};

/// Mitsumi MM1134A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_MM1134A.parse("939 134A").is_ok());
/// ```
pub static MITSUMI_MM1134A: NomParser<GenericPart> = NomParser {
    name: "Mitsumi MM1134A",
    f: |input| {
        tuple((year1_week2, tag(" 134A")))
            .map(|(date_code, _)| GenericPart {
                kind: "MM1134A".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Mitsumi MM1514X
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_MM1514X.parse("105 514X").is_ok());
/// assert!(parser::mitsumi::MITSUMI_MM1514X.parse("081 514X").is_ok());
/// ```
pub static MITSUMI_MM1514X: NomParser<GenericPart> = NomParser {
    name: "Mitsumi MM1514X",
    f: |input| {
        tuple((year1, alnum_uppers(2), tag(" 514X")))
            .map(|(year, _, _)| GenericPart {
                kind: "MM1514X".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};

/// Mitsumi MM1581A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_MM1581A.parse("422 1581A").is_ok());
/// ```
pub static MITSUMI_MM1581A: NomParser<GenericPart> = NomParser {
    name: "Mitsumi MM1581A",
    f: |input| {
        tuple((year1_week2, tag(" 1581A")))
            .map(|(date_code, _)| GenericPart {
                kind: "MM1581A".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Mitsumi MM1592F
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_MM1592F.parse("548 592F").is_ok());
/// ```
pub static MITSUMI_MM1592F: NomParser<GenericPart> = NomParser {
    name: "Mitsumi MM1592F",
    f: |input| {
        tuple((year1_week2, tag(" 592F")))
            .map(|(date_code, _)| GenericPart {
                kind: "MM1592F".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Mitsumi PM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_PM.parse("MITSUMI JAPAN 528A PM C").is_ok());
/// ```
pub static MITSUMI_PM: NomParser<GenericPart> = NomParser {
    name: "Mitsumi PM",
    f: |input| {
        tuple((
            tag("MITSUMI JAPAN "),
            year1_week2,
            opt(char(' ')),
            uppers(1),
            char(' '),
            alt((tag("PM B3"), tag("PM B4"), tag("PM C"))),
        ))
        .map(|(_, date_code, _, _, _, kind)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Mitsumi),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// # use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mitsumi::MITSUMI_MGL_TRANSFORMER.parse("82Y7").is_ok());
/// assert!(parser::mitsumi::MITSUMI_MGL_TRANSFORMER.parse("84Z7").is_ok());
/// ```
pub static MITSUMI_MGL_TRANSFORMER: NomParser<GenericPart> = NomParser {
    name: "Mitsumi MGL transformer",
    f: |input| {
        alt((tag("82Y7"), tag("84Z7")))
            .map(|kind| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Mitsumi),
                date_code: None,
            })
            .parse(input)
    },
};
