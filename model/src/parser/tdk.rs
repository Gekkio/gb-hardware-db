// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser, bytes::streaming::tag};

use super::{
    GenericPart, Manufacturer, NomParser,
    for_nom::{alphas, lines3, uppers},
};

/// TDK ZJY-M4A
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::tdk::TDK_ZJY_M4A.parse("TDK ZJY-M4A N").is_ok());
/// ```
pub static TDK_ZJY_M4A: NomParser<GenericPart> = NomParser {
    name: "TDK ZJY-M4A",
    f: |input| {
        lines3(tag("TDK"), tag("ZJY-M4A"), uppers(1))
            .map(|(_, kind, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Tdk),
                date_code: None,
            })
            .parse(input)
    },
};

/// TDK ZJY-M4PA
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::tdk::TDK_ZJY_M4PA.parse("TDK ZJY-M4PA n").is_ok());
/// ```
pub static TDK_ZJY_M4PA: NomParser<GenericPart> = NomParser {
    name: "TDK ZJY-M4PA",
    f: |input| {
        lines3(tag("TDK"), tag("ZJY-M4PA"), alphas(1))
            .map(|(_, kind, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Tdk),
                date_code: None,
            })
            .parse(input)
    },
};
