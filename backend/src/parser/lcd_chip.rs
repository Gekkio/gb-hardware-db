// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{month2, week2, year1, LabelParser, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::{Month, Week},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LcdChip {
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub week: Option<Week>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::lcd_chip::lcd_chip_old().parse("110").is_ok());
/// ```
pub fn lcd_chip_old() -> &'static impl LabelParser<LcdChip> {
    single_parser!(LcdChip, r#"^([0-9])([0-9]{2})$"#, move |c| {
        Ok(LcdChip {
            year: Some(year1(&c[1])?),
            month: Some(month2(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::lcd_chip::lcd_chip_new().parse("5341").is_ok());
/// ```
pub fn lcd_chip_new() -> &'static impl LabelParser<LcdChip> {
    single_parser!(LcdChip, r#"^([0-9])([0-9]{2})[0-9]$"#, move |c| {
        Ok(LcdChip {
            year: Some(year1(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn lcd_chip() -> &'static impl LabelParser<LcdChip> {
    multi_parser!(LcdChip, lcd_chip_old(), lcd_chip_new())
}
