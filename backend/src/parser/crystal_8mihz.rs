// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{kds_month1, week2, year1, year2, Crystal, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

const FREQUENCY: u32 = 8_388_608;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_8mihz::kds_8388().parse("KDS 9841 8.388").is_ok());
/// ```
pub fn kds_8388() -> &'static impl LabelParser<Crystal> {
    single_parser!(
        Crystal,
        r#"^KDS\ ([0-9]{2})([0-9]{2})\ 8\.388$"#,
        move |c| {
            Ok(Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: FREQUENCY,
                year: Some(year2(&c[1])?),
                month: None,
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_8mihz::kds_d838().parse("D838K0I").is_ok());
/// ```
pub fn kds_d838() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^D838([A-Z])([0-9])[A-Z]$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            frequency: FREQUENCY,
            year: Some(year1(&c[2])?),
            month: Some(kds_month1(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_8mihz::kinseki_8388().parse("8388 KSS 1CF").is_ok());
/// assert!(parser::crystal_8mihz::kinseki_8388().parse("8388 KSS 9J").is_ok());
/// ```
pub fn kinseki_8388() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^8388\ KSS\ ([0-9])([A-Z])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: Some(kds_month1(&c[2])?),
            week: None,
        })
    })
}
pub fn crystal_8mihz() -> &'static impl LabelParser<Crystal> {
    multi_parser!(Crystal, kds_8388(), kds_d838(), kinseki_8388())
}
