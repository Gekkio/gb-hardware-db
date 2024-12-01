// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, GenericPart, LabelParser};
use crate::{
    macros::{multi_parser, single_parser},
    parser::{mitsumi, PartDateCode},
};

pub type AgsChargeController = GenericPart;

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::ags_charge_ctrl::unknown().parse("2253B 3129").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<AgsChargeController> {
    single_parser!(
        AgsChargeController,
        r#"^2253B\ ([0-9])([0-9]{2})[0-9]$"#,
        move |c| {
            Ok(AgsChargeController {
                kind: "2253B".to_owned(),
                manufacturer: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[1])?,
                    week: week2(&c[2])?,
                }),
            })
        }
    )
}

pub fn ags_charge_ctrl() -> &'static impl LabelParser<AgsChargeController> {
    multi_parser!(AgsChargeController, &mitsumi::MITSUMI_MM1581A, unknown())
}
