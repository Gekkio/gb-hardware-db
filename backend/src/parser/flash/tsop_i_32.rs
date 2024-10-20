// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::Flash;
use crate::{
    macros::single_parser,
    parser::{month1_alpha, week2, year1, year2, ChipDateCode, LabelParser, Manufacturer},
};

/// Macronix MX29L010 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::macronix_mx29l010().parse("B063857G MX29L010TC-15A1 1H4751").is_ok());
/// assert!(parser::flash::tsop_i_32::macronix_mx29l010().parse("E032457 MX29L010TC-15A1 1E8980").is_ok());
/// assert!(parser::flash::tsop_i_32::macronix_mx29l010().parse("E023057 MX29L010TC-15 1E0290").is_ok());
/// ```
pub fn macronix_mx29l010() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^[A-Z](?<year>[0-9]{2})(?<week>[0-9]{2})[0-9]{2}G?\ (?<kind>MX29L010)(?<package>T)(?<grade>C)-(?<speed>15)([A-Z][0-9])?\ [0-9][A-Z][0-9]{4}$"#,
        move |c| {
            Ok(Flash {
                kind: String::from(&c["kind"]),
                manufacturer: Some(Manufacturer::Macronix),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Sanyo LE26FV10 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::sanyo_le26fv10().parse("LE26FV10N1TS-10 3MU50").is_ok());
/// assert!(parser::flash::tsop_i_32::sanyo_le26fv10().parse("LE26FV10N1TS-10 4DU2A").is_ok());
/// ```
pub fn sanyo_le26fv10() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^(?<kind>LE26FV10N1)(?<package>TS)-(?<speed>10)\ (?<year>[0-9])(?<month>[A-Z])[A-Z][0-9][[:alnum:]]$"#,
        move |c| {
            Ok(Flash {
                kind: String::from(&c["kind"]),
                manufacturer: Some(Manufacturer::Sanyo),
                date_code: Some(ChipDateCode::YearMonth {
                    year: year1(&c["year"])?,
                    month: month1_alpha(&c["month"])?,
                }),
            })
        },
    )
}

/// Atmel AT29LV512 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::atmel_at29lv512().parse("AT29LV512 15TC 0114").is_ok());
/// ```
pub fn atmel_at29lv512() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^(?<kind>AT29LV512)\ (?<speed>[12][05])(?<package>T)(?<grade>[CI])\ (?<year>[0-9]{2})(?<week>[0-9]{2})$"#,
        move |c| {
            Ok(Flash {
                kind: String::from(&c["kind"]),
                manufacturer: Some(Manufacturer::Atmel),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// SST SST39VF512 (TSOP-I-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_32::sst_sst39vf512().parse("39VF512 70-4C-WH 0216049-D").is_ok());
/// assert!(parser::flash::tsop_i_32::sst_sst39vf512().parse("39VF512 70-4C-WH 0350077-D").is_ok());
/// ```
pub fn sst_sst39vf512() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^(?<kind>39VF512)\ (?<speed>70)-(?<durability>4)(?<grade>[CI])-(?<package>WH)\ (?<year>[0-9]{2})(?<week>[0-9]{2})[0-9]{3}-[A-Z]$"#,
        move |c| {
            Ok(Flash {
                kind: format!("SST{}", &c["kind"]),
                manufacturer: Some(Manufacturer::Sst),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}
