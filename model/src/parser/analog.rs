// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, Parser as _};

use super::{
    for_nom::{digits, lines3, uppers, year2_week2},
    GenericPart, Manufacturer, NomParser,
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::analog::ANALOG_ADXL202JQC.parse("2738109451 0028 ADXL202JQC").is_ok());
/// ```
pub static ANALOG_ADXL202JQC: NomParser<GenericPart> = NomParser {
    name: "Analog ADXL202JQC",
    f: |input| {
        lines3(digits(10), year2_week2, tag("ADXL202JQC"))
            .map(|(_, date_code, kind)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Analog),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::analog::ANALOG_ADXL202JE.parse("06245 202JE 0501A").is_ok());
/// ```
pub static ANALOG_ADXL202JE: NomParser<GenericPart> = NomParser {
    name: "Analog ADXL202JE",
    f: |input| {
        lines3(digits(5), tag("202JE"), digits(4).and(uppers(1)))
            .map(|(_, kind, _)| GenericPart {
                kind: format!("ADXL{kind}"),
                manufacturer: Some(Manufacturer::Analog),
                date_code: None,
            })
            .parse(input)
    },
};
