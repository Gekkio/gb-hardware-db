// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _,
    bytes::streaming::tag,
    character::streaming::one_of,
    combinator::{opt, recognize},
    sequence::delimited,
};

use super::{
    GenericPart, Manufacturer, NomParser, PartDateCode,
    for_nom::{alnum_uppers, digits, lines4, uppers, week2, year1, year1_week2},
};

/// Mosel-Vitelic LH52B256 (SOP-28, 5V, 256 Kibit / 32 KiB)
///
/// Probably Sharp LH52B256N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH52B256N.parse("LH52B256NA-10PLL MOSEL-VITELIC JAPAN N643 0T BB").is_ok());
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH52B256N.parse("LH52B256NZ-10PLL MOSEL-VITELIC JAPAN N636 06 CB").is_ok());
/// ```
pub static MOSEL_VITELIC_LH52B256N: NomParser<GenericPart> = NomParser {
    name: "Mosel-Vitelic LH52B256N",
    f: |input| {
        lines4(
            recognize((tag("LH52B256N"), one_of("AZ"), tag("-10PLL"))),
            tag("MOSEL-VITELIC"),
            tag("JAPAN"),
            delimited(
                tag("N"),
                year1_week2,
                (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(2)),
            ),
        )
        .map(|(kind, _, _, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::MoselVitelic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Mosel-Vitelic LH5168N (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// Probably Sharp LH5168N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH5168N.parse("LH5168N-10PL MOSEL-VITELIC JAPAN N745 1G BH").is_ok());
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH5168N.parse("LH5168N-10PL MOSEL-VITELIC JAPAN N7 34 22 BH").is_ok());
/// ```
pub static MOSEL_VITELIC_LH5168N: NomParser<GenericPart> = NomParser {
    name: "Mosel-Vitelic LH5168N",
    f: |input| {
        lines4(
            tag("LH5168N-10PL"),
            tag("MOSEL-VITELIC"),
            tag("JAPAN"),
            delimited(
                tag("N"),
                (year1, opt(tag(" ")), week2)
                    .map(|(year, _, week)| PartDateCode::YearWeek { year, week }),
                (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(2)),
            ),
        )
        .map(|(kind, _, _, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::MoselVitelic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Mosel-Vitelic LH5268AN (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// Probably Sharp LH5268AN manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH5268AN.parse("LH5268ANF-10PLL MOSEL-VITELIC JAPAN N526 0H BC").is_ok());
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH5268AN.parse("LH5268ANA-10PLL MOSEL-VITELIC JAPAN N527 02 BC").is_ok());
/// ```
pub static MOSEL_VITELIC_LH5268AN: NomParser<GenericPart> = NomParser {
    name: "Mosel-Vitelic LH5268AN",
    f: |input| {
        lines4(
            recognize((tag("LH5268AN"), one_of("AF"), tag("-10PLL"))),
            tag("MOSEL-VITELIC"),
            tag("JAPAN"),
            delimited(
                tag("N"),
                year1_week2,
                (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(2)),
            ),
        )
        .map(|(kind, _, _, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::MoselVitelic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Mosel-Vitelic LH52A64N (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// Probably Sharp LH52A64N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::mosel_vitelic::MOSEL_VITELIC_LH52A64N.parse("LH52A64N-PL MOSEL-VITELIC JAPAN N651 0F C").is_ok());
/// ```
pub static MOSEL_VITELIC_LH52A64N: NomParser<GenericPart> = NomParser {
    name: "Mosel-Vitelic LH52A64N",
    f: |input| {
        lines4(
            tag("LH52A64N-PL"),
            tag("MOSEL-VITELIC"),
            tag("JAPAN"),
            delimited(
                tag("N"),
                year1_week2,
                (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(1)),
            ),
        )
        .map(|(kind, _, _, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::MoselVitelic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
