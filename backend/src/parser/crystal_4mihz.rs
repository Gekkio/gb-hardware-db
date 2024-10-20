// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{kds_month1, week2, year1, year2, Crystal, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

const FREQUENCY: u32 = 4_194_304;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_4mihz::kds_4194().parse("KDS 9803 4.194").is_ok());
/// assert!(parser::crystal_4mihz::kds_4194().parse("KDS9807 4.194").is_ok());
/// assert!(parser::crystal_4mihz::kds_4194().parse("KDSI 0549 4.194").is_ok());
/// ```
pub fn kds_4194() -> &'static impl LabelParser<Crystal> {
    single_parser!(
        Crystal,
        r#"^KDSI?\ ?([0-9]{2})([0-9]{2})\ 4\.194$"#,
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
/// assert!(parser::crystal_4mihz::kds_4194_short().parse("KDS 6F 4.194").is_ok());
/// ```
pub fn kds_4194_short() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^KDS\ ([0-9])([A-Z])\ 4\.194$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: Some(kds_month1(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_4mihz::kds_d419().parse("D419A2").is_ok());
/// assert!(parser::crystal_4mihz::kds_d419().parse("D419J3I").is_ok());
/// ```
pub fn kds_d419() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^D419([A-Z])([0-9])[A-Z]?$"#, move |c| {
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
/// assert!(parser::crystal_4mihz::kinseki_4194().parse("4194 KSS 0KF").is_ok());
/// assert!(parser::crystal_4mihz::kinseki_4194().parse("4194 KSS1A").is_ok());
/// ```
pub fn kinseki_4194() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4194\ KSS\ ?([0-9])([A-Z])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: Some(kds_month1(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_4mihz::unknown2().parse("4.19C59").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.19C([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_4mihz::unknown_41943().parse("4.1943 9752").is_ok());
/// ```
pub fn unknown_41943() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.1943\ ([0-9]{2})([0-9]{2})$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            frequency: FREQUENCY,
            year: Some(year2(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_4mihz::unknown_41943_2().parse("4.1943 RVR 841").is_ok());
/// ```
pub fn unknown_41943_2() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.1943\ RVR\ ([0-9])([0-9]{2})$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_4mihz::unknown2().parse("4.19C59").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.19C([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

pub fn crystal_4mihz() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        kds_d419(),
        unknown(),
        kinseki_4194(),
        kds_4194(),
        kds_4194_short(),
        unknown_41943(),
        unknown_41943_2(),
    )
}
