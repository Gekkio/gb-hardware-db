// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _, bytes::streaming::tag, character::streaming::one_of, combinator::recognize,
};

use super::{
    GenericPart, Manufacturer, NomParser,
    for_nom::{digits, lines3, uppers, year2_week2},
};

/// Analog ADXL202JQC accelerometer (14-lead CERPAK, 3-5.25V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::analog::ANALOG_ADXL202JQC.parse("2738109451 0028 ADXL202JQC").is_ok());
/// ```
pub static ANALOG_ADXL202JQC: NomParser<GenericPart> = NomParser {
    name: "Analog ADXL202JQC",
    f: |input| {
        lines3(
            digits(10),
            year2_week2,
            recognize((
                tag("ADXL202"),
                one_of("JA"), // temperature
                tag("QC"),    // package
            )),
        )
        .map(|(_, date_code, kind)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Analog),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Analog ADXL202JE accelerometer (8-lead LCC, 3-5.25V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::analog::ANALOG_ADXL202JE.parse("06245 202JE 0501A").is_ok());
/// ```
pub static ANALOG_ADXL202JE: NomParser<GenericPart> = NomParser {
    name: "Analog ADXL202JE",
    f: |input| {
        lines3(
            digits(5),
            recognize((
                tag("202"),
                one_of("JA"), // temperature
                tag("E"),     // package
            )),
            digits(4).and(uppers(1)),
        )
        .map(|(_, kind, _)| GenericPart {
            kind: format!("ADXL{kind}"),
            manufacturer: Some(Manufacturer::Analog),
            date_code: None,
        })
        .parse(input)
    },
};
