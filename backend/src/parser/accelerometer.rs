// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, character::streaming::char, sequence::tuple, Parser as _};

use super::{
    for_nom::{digits, uppers, year2_week2},
    GenericPart, LabelParser, Manufacturer, NomParser,
};
use crate::macros::multi_parser;

pub type Accelerometer = GenericPart;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::accelerometer::ANALOG_ADXL202JQC.parse("2738109451 0028 ADXL202JQC").is_ok());
/// ```
pub static ANALOG_ADXL202JQC: NomParser<Accelerometer> = NomParser {
    name: "Analog ADXL202JQC",
    f: |input| {
        tuple((
            digits(10),
            char(' '),
            year2_week2,
            char(' '),
            tag("ADXL202JQC"),
        ))
        .map(|(_, _, date_code, _, kind)| Accelerometer {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Analog),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::accelerometer::ANALOG_ADXL202JE.parse("06245 202JE 0501A").is_ok());
/// ```
pub static ANALOG_ADXL202JE: NomParser<Accelerometer> = NomParser {
    name: "Analog ADXL202JE",
    f: |input| {
        tuple((
            digits(5),
            char(' '),
            tag("202JE"),
            char(' '),
            digits(4),
            uppers(1),
        ))
        .map(|(_, _, kind, _, _, _)| Accelerometer {
            kind: format!("ADXL{kind}"),
            manufacturer: Some(Manufacturer::Analog),
            date_code: None,
        })
        .parse(input)
    },
};

pub fn accelerometer() -> &'static impl LabelParser<Accelerometer> {
    multi_parser!(Accelerometer, &ANALOG_ADXL202JQC, &ANALOG_ADXL202JE)
}
