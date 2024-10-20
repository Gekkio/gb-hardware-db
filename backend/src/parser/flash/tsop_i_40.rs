// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::Flash;
use crate::{
    macros::single_parser,
    parser::{week2, year2, ChipDateCode, LabelParser, Manufacturer},
};

/// Macronix MX29F008 (TSOP-I-40)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_40::macronix_mx29f008().parse("E991012 29F008TC-14 21534 TAIWAN").is_ok());
/// ```
pub fn macronix_mx29f008() -> &'static impl LabelParser<Flash> {
    single_parser!(
        Flash,
        r#"^[A-Z](?<year>[0-9]{2})(?<week>[0-9]{2})[0-9]{2}\ (?<kind>29F008)(?<package>T)(?<grade>C)-(?<speed>14)\ [0-9]{5}\ TAIWAN$"#,
        move |c| {
            Ok(Flash {
                kind: format!("MX{}", &c["kind"]),
                manufacturer: Some(Manufacturer::Macronix),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}
