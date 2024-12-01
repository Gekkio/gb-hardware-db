// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::recognize,
    sequence::{preceded, tuple},
    Parser as _,
};

use super::{
    for_nom::{alnum_uppers, digits, month1_123abc, year1, year1_week2},
    Eeprom, GenericPart, Manufacturer, NomParser, PartDateCode,
};

/// ROHM ??9853 EEPROM (SOP-8)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9853.parse("9853 2A46").is_ok());
/// assert!(parser::rohm::ROHM_9853.parse("9853 6912").is_ok());
/// ```
pub static ROHM_9853: NomParser<Eeprom> = NomParser {
    name: "ROHM 9853",
    f: |input| {
        tuple((tag("9853"), char(' '), year1, month1_123abc, digits(2)))
            .map(|(kind, _, year, month, _)| Eeprom {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM ??9854 EEPROM (SOP-8)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9854.parse("9854 5S95W").is_ok());
/// ```
pub static ROHM_9854: NomParser<Eeprom> = NomParser {
    name: "ROHM 9854",
    f: |input| {
        tuple((
            tag("9854"),
            char(' '),
            year1,
            alnum_uppers(1),
            digits(2),
            char('W'),
        ))
        .map(|(kind, _, year, _, _, _)| Eeprom {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(PartDateCode::Year { year }),
        })
        .parse(input)
    },
};

/// ROHM BA6129
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BA6129.parse("6129 4803").is_ok());
/// assert!(parser::rohm::ROHM_BA6129.parse("6129A 6194").is_ok());
/// ```
pub static ROHM_BA6129: NomParser<GenericPart> = NomParser {
    name: "ROHM BA6129",
    f: |input| {
        tuple((
            alt((tag("6129A"), tag("6129"))),
            char(' '),
            year1,
            alnum_uppers(1),
            digits(2),
        ))
        .map(|(kind, _, year, _, _)| GenericPart {
            kind: format!("BA{kind}"),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(PartDateCode::Year { year }),
        })
        .parse(input)
    },
};

/// ROHM BA6735
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BA6735.parse("6735 8C19").is_ok());
/// ```
pub static ROHM_BA6735: NomParser<GenericPart> = NomParser {
    name: "ROHM BA6735",
    f: |input| {
        tuple((tag("6735"), char(' '), year1, month1_123abc, digits(2)))
            .map(|(_, _, year, month, _)| GenericPart {
                kind: "BA6735".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM ??9750
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9750.parse("9750A 1581").is_ok());
/// assert!(parser::rohm::ROHM_9750.parse("9750B 2A69").is_ok());
/// ```
pub static ROHM_9750: NomParser<GenericPart> = NomParser {
    name: "ROHM 9750",
    f: |input| {
        tuple((
            recognize(tag("9750").and(one_of("AB"))),
            char(' '),
            year1,
            month1_123abc,
            digits(2),
        ))
        .map(|(kind, _, year, month, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(PartDateCode::YearMonth { year, month }),
        })
        .parse(input)
    },
};

/// ROHM ??9753
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9753.parse("9753 4862").is_ok());
/// ```
pub static ROHM_9753: NomParser<GenericPart> = NomParser {
    name: "ROHM 9753",
    f: |input| {
        tuple((tag("9753"), char(' '), year1, month1_123abc, digits(2)))
            .map(|(kind, _, year, month, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM BH7835AFS AGB amplifier
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BH7835AFS.parse("BH7835AFS 337 T22").is_ok());
/// ```
pub static ROHM_BH7835AFS: NomParser<GenericPart> = NomParser {
    name: "ROHM BH7835AFS",
    f: |input| {
        tuple((
            tag("BH7835AFS"),
            char(' '),
            year1_week2,
            char(' '),
            alnum_uppers(1),
            digits(2),
        ))
        .map(|(kind, _, date_code, _, _, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM ICD2-R
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_ICD2_R.parse("Nintendo ICD2-R 435 179").is_ok());
/// ```
pub static ROHM_ICD2_R: NomParser<GenericPart> = NomParser {
    name: "ROHM ICD2_R",
    f: |input| {
        tuple((
            preceded(tag("Nintendo "), tag("ICD2-R")),
            char(' '),
            year1_week2,
            char(' '),
            alnum_uppers(1),
            digits(2),
        ))
        .map(|(kind, _, date_code, _, _, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
