// SPDX-FileCopyrightText: 2024 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::Ram;
use crate::{
    macros::single_parser,
    parser::{week2, year1, year2, LabelParser, Manufacturer, PartDateCode},
};

/// Rohm BR62256F (SOP-28, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::rohm_br62256f().parse("BR62256F-70LL 006 169NA").is_ok());
/// ```
pub fn rohm_br62256f() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>BR62256[AB]?F)-(?<speed>70)(?<power>LL)\ (?<year>[0-9])(?<week>[0-9]{2})\ [0-9]{3}[A-Z]{0,2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}-{speed}{power}",
                    kind = &c["kind"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Rohm BR6265 (SOP-28, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::rohm_br6265().parse("BR6265BF-10SL 111 120N").is_ok());
/// ```
pub fn rohm_br6265() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>BR6265[AB]?F)-(?<speed>10)(?<power>SL)\ (?<year>[0-9])(?<week>[0-9]{2})\ [0-9]{3}[A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}-{speed}{power}",
                    kind = &c["kind"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Rohm XLJ6265
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::rohm_xlj6265().parse("XLJ6265BF-10SL 640 173N").is_ok());
/// ```
pub fn rohm_xlj6265() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>XLJ6265[AB]?F)-(?<speed>10)(?<power>SL)\ (?<year>[0-9])(?<week>[0-9]{2})\ [0-9]{3}[A-Z]{0,2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}-{speed}{power}",
                    kind = &c["kind"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Sharp LH52256
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::sharp_lh52256().parse("LH52256CN-10LL SHARP JAPAN 9832 1 SN").is_ok());
/// assert!(parser::sram::sop_28::sharp_lh52256().parse("LH52256CN-10LL SHARP A9802 3 EC").is_ok());
/// ```
pub fn sharp_lh52256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>LH52256C?)(?<package>N)-(?<speed>10)(?<power>LL)\ SHARP\ ((JAPAN\ )|A)(?<year>[0-9]{2})(?<week>[0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}-{speed}{power}",
                    kind = &c["kind"],
                    package = &c["package"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}
