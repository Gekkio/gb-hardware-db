// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

pub type Flash = ChipYearWeek;

/// Macronix MX29F008 flash
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::macronix_mx29f008().parse("E991012 29F008TC-14 21534 TAIWAN").is_ok());
/// ```
pub fn macronix_mx29f008() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}\ (29F008[A-Z]{2}-[0-9]{2})\ [0-9]{5}\ TAIWAN$"#,
        move |c| {
            Ok(Flash {
                kind: format!("MX{}", &c[3]),
                manufacturer: Some(Manufacturer::Macronix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Macronix MX29L010 flash
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::macronix_mx29l010().parse("B063857G MX29L010TC-15A1 1H4751").is_ok());
/// assert!(parser::flash::macronix_mx29l010().parse("E032457 MX29L010TC-15A1 1E8980").is_ok());
/// ```
pub fn macronix_mx29l010() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}G?\ (MX29L010[A-Z]{2}-[0-9]{2}[A-Z]?[0-9]?)\ [[:alnum:]]{6}$"#,
        move |c| {
            Ok(Flash {
                kind: String::from(&c[3]),
                manufacturer: Some(Manufacturer::Macronix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sanyo LE26FV10 flash
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::sanyo_le26fv10().parse("LE26FV10N1TS-10 3MU50").is_ok());
/// assert!(parser::flash::sanyo_le26fv10().parse("LE26FV10N1TS-10 4DU2A").is_ok());
/// ```
pub fn sanyo_le26fv10() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^(LE26FV10N1TS-10)\ ([0-9])[A-Z]{2}[0-9][[:alnum:]]$"#,
        move |c| {
            Ok(Flash {
                kind: String::from(&c[1]),
                manufacturer: Some(Manufacturer::Sanyo),
                year: Some(year2(&c[2])?),
                week: None,
            })
        },
    )
}

pub fn flash() -> &'static impl LabelParser<Flash> {
    multi_parser!(
        Flash,
        macronix_mx29f008(),
        macronix_mx29l010(),
        sanyo_le26fv10(),
    )
}
