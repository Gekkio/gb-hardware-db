// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
    sequence::tuple,
    Parser as _,
};

use super::{
    for_nom::{alnum_uppers, digits, uppers},
    GenericPart,
};
use crate::parser::{for_nom::year2_week2, Manufacturer, NomParser};

/// BSI BS62LV256 (SOP-28, 2.4V-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::bsi::BSI_BS62LV256.parse("BSI BS62LV256SC-70 S2827V52155 A0106 TAIWAN").is_ok());
/// assert!(parser::bsi::BSI_BS62LV256.parse("BSI BS62LV256SC-70 S2828W11075.1 F0231 TAIWAN").is_ok());
/// assert!(parser::bsi::BSI_BS62LV256.parse("BSI BS62LV256SCG70 S2828CA30125.A D05502 TAIWAN").is_ok());
/// assert!(parser::bsi::BSI_BS62LV256.parse("BSI BS62LV256SC-70 S2828W13088.1N F0318 TAIWAN").is_ok());
/// ```
pub static BSI_BS62LV256: NomParser<GenericPart> = NomParser {
    name: "BSI BS62LV256",
    f: |input| {
        tuple((
            tag("BSI "),
            recognize(tuple((
                tag("BS62LV256"),
                tag("S"),                    // package
                one_of("CI"),                // temperature
                one_of("-GP"),               // material
                alt((tag("55"), tag("70"))), // speed
            ))),
            char(' '),
            tuple((
                alt((tag("S2827"), tag("S2828"))),
                opt(alt((tag("CA"), uppers(1)))),
                digits(5),
                opt(char('.').and(alnum_uppers(1)).and(opt(uppers(1)))),
            )),
            char(' '),
            tuple((uppers(1), year2_week2, opt(digits(1)))),
            tag(" TAIWAN"),
        ))
        .map(|(_, kind, _, _, _, (_, date_code, _), _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Bsi),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// BSI BS616LV2018 (TSOP-I-48, 2.4-3.6V)
///
/// Source:
///   "BSI BS616LV2018 - Very Low Power/Voltage CMOS SRAM 128k x 16 bit"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::bsi::BSI_BS616LV2018.parse("BSI BS616LV2018TC-70 S31686-2FY24092.1 L0314 TAIWAN").is_ok());
/// assert!(parser::bsi::BSI_BS616LV2018.parse("BSI BS616LV2018TC-70 S31686-2FY10121.1 L0230 TAIWAN").is_ok());
/// ```
pub static BSI_BS616LV2018: NomParser<GenericPart> = NomParser {
    name: "BSI BS616LV2018",
    f: |input| {
        tuple((
            tag("BSI "),
            recognize(tuple((
                tag("BS616LV2018"),
                tag("T"),      // package
                one_of("CI"),  // temperature
                one_of("-GP"), // material
                tag("70"),     // speed
            ))),
            char(' '),
            tuple((tag("S31686-2FY"), digits(5), tag(".1"))),
            char(' '),
            tuple((uppers(1), year2_week2)),
            tag(" TAIWAN"),
        ))
        .map(|(_, kind, _, _, _, (_, date_code), _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Bsi),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// BSI BS616LV2019 (TSOP-I-48, 2.4-3.6V)
///
/// Source:
///   "BSI BS616LV2019 - Very Low Power CMOS SRAM 128k x 16 bit"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::bsi::BSI_BS616LV2019.parse("BSI BS616LV2019TC-70 S31687FZ26013.1 L0335 TAIWAN").is_ok());
/// assert!(parser::bsi::BSI_BS616LV2019.parse("BSI BS616LV2019TC-70 S31687FZ27050.1 L0336 TAIWAN").is_ok());
/// assert!(parser::bsi::BSI_BS616LV2019.parse("BSI BS616LV2019TC-70 S31687FZ31012.1 L0410 TAIWAN").is_ok());
/// ```
pub static BSI_BS616LV2019: NomParser<GenericPart> = NomParser {
    name: "BSI BS616LV2019",
    f: |input| {
        tuple((
            tag("BSI "),
            recognize(tuple((
                tag("BS616LV2019"),
                tag("T"),                    // package
                one_of("CI"),                // temperature
                one_of("-GP"),               // material
                alt((tag("55"), tag("70"))), // speed
            ))),
            char(' '),
            tuple((tag("S31687FZ"), digits(5), tag(".1"))),
            char(' '),
            tuple((uppers(1), year2_week2)),
            tag(" TAIWAN"),
        ))
        .map(|(_, kind, _, _, _, (_, date_code), _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Bsi),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
