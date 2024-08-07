// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, ChipYearWeek, LabelParser};
use crate::macros::{multi_parser, single_parser};

pub type Eeprom = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::eeprom::lcs5().parse("LCS5 040").is_ok());
/// ```
pub fn lcs5() -> &'static impl LabelParser<Eeprom> {
    single_parser!(
        Eeprom,
        r#"^LCS5\ ([0-9])([0-9]{2})(\ [0-9]{2})?$"#,
        move |c| {
            Ok(Eeprom {
                kind: "LCS5".to_owned(),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::eeprom::lc56().parse("LC56 W617 08").is_ok());
/// ```
pub fn lc56() -> &'static impl LabelParser<Eeprom> {
    single_parser!(Eeprom, r#"^LC56\ [A-Z][0-9]{3}\ [0-9]{2}$"#, move |_| {
        Ok(Eeprom {
            kind: "LC56".to_owned(),
            manufacturer: None,
            year: None,
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::eeprom::unknown_9853().parse("9853 2A46").is_ok());
/// assert!(parser::eeprom::unknown_9853().parse("9853 6912").is_ok());
/// ```
pub fn unknown_9853() -> &'static impl LabelParser<Eeprom> {
    single_parser!(Eeprom, r#"^9853\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(Eeprom {
            kind: "9853".to_owned(),
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::eeprom::unknown_9854().parse("9854 5S95W").is_ok());
/// ```
pub fn unknown_9854() -> &'static impl LabelParser<Eeprom> {
    single_parser!(
        Eeprom,
        r#"^9854\ ([0-9])[[:alnum:]][0-9]{2}[A-Z]$"#,
        move |c| {
            Ok(Eeprom {
                kind: "9854".to_owned(),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}
pub fn eeprom() -> &'static impl LabelParser<Eeprom> {
    multi_parser!(Eeprom, lcs5(), lc56(), unknown_9853(), unknown_9854())
}
