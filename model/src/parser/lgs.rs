// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _,
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
};

use super::{GenericPart, Manufacturer, NomParser, for_nom::year2_week2};

/// LGS GM76C256 SRAM (SOP-28, 5V, 256 Kibit / 32 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lgs::LGS_GM76C256.parse("LGS GM76C256CLLFW70 0047 KOREA").is_ok());
/// ```
pub static LGS_GM76C256: NomParser<GenericPart> = NomParser {
    name: "LGS GM76C256",
    f: |input| {
        let package = Package::Sop;
        (
            tag("LGS "),
            (
                recognize(tag("GM76C256").and(opt(one_of("ABC")))),
                alt((tag("LL"), tag("L"))),             // power
                tag(package.code()),                    // package
                alt((tag("70"), tag("85"), tag("10"))), // speed
            ),
            char(' '),
            year2_week2,
            tag(" KOREA"),
        )
            .map(
                |(_, (kind, power, package, speed), _, date_code, _)| GenericPart {
                    kind: format!("{kind}{power}{package}{speed}"),
                    manufacturer: Some(Manufacturer::Lgs),
                    date_code: Some(date_code),
                },
            )
            .parse(input)
    },
};

/// Hyundai GM76C256 SRAM (SOP-28, 5V, 256 Kibit / 32 KiB)
///
/// Originally by LGS, acquired by Hyundai
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lgs::HYUNDAI_GM76C256.parse("HYUNDAI GM76C256CLLFW70 0047 KOREA").is_ok());
/// ```
pub static HYUNDAI_GM76C256: NomParser<GenericPart> = NomParser {
    name: "Hyundai GM76C256",
    f: |input| {
        let package = Package::Sop;
        (
            tag("HYUNDAI "),
            (
                recognize(tag("GM76C256").and(opt(one_of("ABC")))),
                alt((tag("LL"), tag("L"))),             // power
                tag(package.code()),                    // package
                alt((tag("70"), tag("85"), tag("10"))), // speed
            ),
            char(' '),
            year2_week2,
            tag(" KOREA"),
        )
            .map(
                |(_, (kind, power, package, speed), _, date_code, _)| GenericPart {
                    kind: format!("{kind}{power}{package}{speed}"),
                    manufacturer: Some(Manufacturer::Hyundai),
                    date_code: Some(date_code),
                },
            )
            .parse(input)
    },
};

/// Hyundai GM76V256 SRAM (SOP-28, 2.5-3.6V, 256 Kibit / 32 KiB)
///
/// Originally by LGS, acquired by Hyundai
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lgs::HYUNDAI_GM76V256.parse("HYUNDAI GM76V256CLLFW10 0115 KOREA").is_ok());
/// ```
pub static HYUNDAI_GM76V256: NomParser<GenericPart> = NomParser {
    name: "Hyundai GM76V256",
    f: |input| {
        let package = Package::Sop;
        (
            tag("HYUNDAI "),
            (
                recognize(tag("GM76V256").and(opt(one_of("ABC")))),
                alt((tag("LL"), tag("L"))), // power
                tag(package.code()),        // package
                tag("10"),                  // speed
            ),
            char(' '),
            year2_week2,
            tag(" KOREA"),
        )
            .map(
                |(_, (kind, power, package, speed), _, date_code, _)| GenericPart {
                    kind: format!("{kind}{power}{package}{speed}"),
                    manufacturer: Some(Manufacturer::Hyundai),
                    date_code: Some(date_code),
                },
            )
            .parse(input)
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Sop,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Sop => "FW",
        }
    }
}
