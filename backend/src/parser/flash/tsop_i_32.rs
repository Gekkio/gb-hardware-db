// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::opt,
    sequence::tuple,
    Parser as _,
};

use super::Flash;
use crate::parser::{
    for_nom::{alnum_uppers, digits, month1_alpha, uppers, year1, year2_week2},
    macronix, ChipDateCode, Manufacturer, NomParser,
};

/// Macronix MX29L010 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::MACRONIX_MX29L010.parse("B063857G MX29L010TC-15A1 1H4751").is_ok());
/// assert!(parser::flash::tsop_i_32::MACRONIX_MX29L010.parse("E032457 MX29L010TC-15A1 1E8980").is_ok());
/// assert!(parser::flash::tsop_i_32::MACRONIX_MX29L010.parse("E023057 MX29L010TC-15 1E0290").is_ok());
/// assert!(parser::flash::tsop_i_32::MACRONIX_MX29L010.parse("E040257 MX29L010TC-15A1 1F468900A0").is_ok());
/// ```
pub static MACRONIX_MX29L010: NomParser<Flash> = NomParser {
    name: "Macronix MX29L010",
    f: |input| {
        tuple((
            macronix::assembly_vendor_code,
            macronix::date_code,
            tag("57"),     // digits 3 and 4 of "product body" (?)
            opt(tag("G")), // green package?
            tag(" MX29L010TC-15"),
            opt(tag("A1")), // revision?
            char(' '),
            macronix::lot_code_new,
        ))
        .map(|(_, date_code, _, _, _, _, _, _)| Flash {
            kind: String::from("MX29L010TC-15"),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sanyo LE26FV10 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::SANYO_LE26FV10.parse("LE26FV10N1TS-10 3MU50").is_ok());
/// assert!(parser::flash::tsop_i_32::SANYO_LE26FV10.parse("LE26FV10N1TS-10 4DU2A").is_ok());
/// ```
pub static SANYO_LE26FV10: NomParser<Flash> = NomParser {
    name: "Sanyo LE26FV10",
    f: |input| {
        tuple((
            tag("LE26FV10N1"),
            tag("TS-10"), // package, speed
            char(' '),
            year1,
            month1_alpha,
            uppers(1),
            digits(1),
            alnum_uppers(1),
        ))
        .map(|(kind, attrs, _, year, month, _, _, _)| Flash {
            kind: format!("{kind}{attrs}"),
            manufacturer: Some(Manufacturer::Sanyo),
            date_code: Some(ChipDateCode::YearMonth { year, month }),
        })
        .parse(input)
    },
};

/// Atmel AT29LV512 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::ATMEL_AT29LV512.parse("AT29LV512 15TC 0114").is_ok());
/// ```
pub static ATMEL_AT29LV512: NomParser<Flash> = NomParser {
    name: "Atmel AT29LV512",
    f: |input| {
        tuple((
            tag("AT29LV512"),
            char(' '),
            tag("15"),    // speed
            char('T'),    // package
            one_of("CI"), // grade
            char(' '),
            year2_week2,
        ))
        .map(|(kind, _, speed, package, grade, _, date_code)| Flash {
            kind: format!("{kind}-{speed}{package}{grade}"),
            manufacturer: Some(Manufacturer::Atmel),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// SST SST39VF512 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::SST_SST39VF512.parse("39VF512 70-4C-WH 0216049-D").is_ok());
/// assert!(parser::flash::tsop_i_32::SST_SST39VF512.parse("39VF512 70-4C-WH 0350077-D").is_ok());
/// ```
pub static SST_SST39VF512: NomParser<Flash> = NomParser {
    name: "SST SST39VF512",
    f: |input| {
        tuple((
            tag("39VF512"),
            char(' '),
            tag("70-4C-WH"), // speed, durability, grade, package
            char(' '),
            year2_week2,
            digits(3),
            tag("-D"),
        ))
        .map(|(kind, _, attrs, _, date_code, _, _)| Flash {
            kind: format!("SST{kind}-{attrs}"),
            manufacturer: Some(Manufacturer::Sst),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
