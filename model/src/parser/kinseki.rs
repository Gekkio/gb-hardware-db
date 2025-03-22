// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _, branch::alt, bytes::streaming::tag, character::streaming::char,
    sequence::delimited,
};

use super::{
    Crystal, Manufacturer, NomParser,
    for_nom::{lines2, uppers, year1_month1_abc},
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kinseki::KINSEKI_4_MIHZ.parse("4194 KSS 0KF").is_ok());
/// assert!(parser::kinseki::KINSEKI_4_MIHZ.parse("4194 KSS1A").is_ok());
/// ```
pub static KINSEKI_4_MIHZ: NomParser<Crystal> = NomParser {
    name: "Kinseki 4 MiHz",
    f: |input| {
        lines2(
            tag("4194"),
            (
                tag("KSS"),
                alt((
                    delimited(char(' '), year1_month1_abc, uppers(1)),
                    year1_month1_abc,
                )),
            ),
        )
        .map(|(_, (_, date_code))| Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            frequency: Crystal::FREQ_4_MIHZ,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kinseki::KINSEKI_8_MIHZ.parse("8388 KSS 1CF").is_ok());
/// assert!(parser::kinseki::KINSEKI_8_MIHZ.parse("8388 KSS9J").is_ok());
/// ```
pub static KINSEKI_8_MIHZ: NomParser<Crystal> = NomParser {
    name: "Kinseki 8 MiHz",
    f: |input| {
        lines2(
            tag("8388"),
            (
                tag("KSS"),
                alt((
                    delimited(char(' '), year1_month1_abc, uppers(1)),
                    year1_month1_abc,
                )),
            ),
        )
        .map(|(_, (_, date_code))| Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            frequency: Crystal::FREQ_8_MIHZ,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kinseki::KINSEKI_20_MIHZ.parse("KSS20V 8A").is_ok());
/// ```
pub static KINSEKI_20_MIHZ: NomParser<Crystal> = NomParser {
    name: "Kinseki 20 MiHz",
    f: |input| {
        tag("KSS20V ")
            .and(year1_month1_abc)
            .map(|(_, date_code)| Crystal {
                manufacturer: Some(Manufacturer::Kinseki),
                frequency: Crystal::FREQ_20_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kinseki::KINSEKI_32_MIHZ.parse("33WKSS6DT").is_ok());
/// ```
pub static KINSEKI_32_MIHZ: NomParser<Crystal> = NomParser {
    name: "Kinseki 32 MiHz",
    f: |input| {
        tag("33WKSS")
            .and(year1_month1_abc.and(char('T')))
            .map(|(_, (date_code, _))| Crystal {
                manufacturer: Some(Manufacturer::Kinseki),
                frequency: Crystal::FREQ_32_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};
