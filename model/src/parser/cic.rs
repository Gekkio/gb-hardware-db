// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, GenericPart, LabelParser};
use crate::{
    macros::single_parser,
    parser::{Manufacturer, PartDateCode},
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::cic::sharp_cic().parse("F411A © 1990 Nintendo 9428 a").is_ok());
/// ```
pub fn sharp_cic() -> &'static impl LabelParser<GenericPart> {
    single_parser!(
        GenericPart,
        r#"^(F411A|F411B|F413A|F413B)\ ©\ (1990|1992)\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Za-z]?$"#,
        move |c| {
            Ok(GenericPart {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[3])?,
                    week: week2(&c[4])?,
                }),
            })
        }
    )
}

pub fn cic() -> &'static impl LabelParser<GenericPart> {
    sharp_cic()
}
