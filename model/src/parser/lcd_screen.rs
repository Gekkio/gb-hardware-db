// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{LabelParser, Year, month2, year1};
use crate::{
    macros::{multi_parser, single_parser},
    parser::year2,
    time::Month,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LcdScreen {
    pub year: Option<Year>,
    pub month: Option<Month>,
}

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lcd_screen::lcd_screen1().parse("S890220").is_ok());
/// ```
pub fn lcd_screen1() -> &'static impl LabelParser<LcdScreen> {
    single_parser!(LcdScreen, r#"^.*([0-9]{2})([0-9]{2})[0-9]{2}$"#, move |c| {
        Ok(LcdScreen {
            year: Some(year2(&c[1])?),
            month: Some(month2(&c[2])?),
        })
    })
}

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lcd_screen::lcd_screen2().parse("T61102S T61104").is_ok());
/// ```
pub fn lcd_screen2() -> &'static impl LabelParser<LcdScreen> {
    single_parser!(
        LcdScreen,
        r#"^(.*[^0-9])?([0-9])([0-9]{2})[0-9]{2}$"#,
        move |c| {
            Ok(LcdScreen {
                year: Some(year1(&c[2])?),
                month: Some(month2(&c[3])?),
            })
        }
    )
}

pub fn lcd_screen() -> &'static impl LabelParser<LcdScreen> {
    multi_parser!(LcdScreen, lcd_screen1(), lcd_screen2())
}
