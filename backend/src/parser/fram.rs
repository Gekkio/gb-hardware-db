// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::{
    macros::multi_parser,
    parser::{GenericChip, LabelParser},
};

pub mod sop_28 {
    use super::Fram;
    use crate::{
        macros::single_parser,
        parser::{week2, year2, ChipDateCode, LabelParser, Manufacturer},
    };

    /// Fujitsu MB85R256 (SOP-28)
    ///
    /// ```
    /// use gbhwdb_backend::parser::{self, LabelParser};
    /// assert!(parser::fram::sop_28::fujitsu_mb85r256().parse("JAPAN MB85R256A 0412 M88").is_ok());
    /// assert!(parser::fram::sop_28::fujitsu_mb85r256().parse("JAPAN MB85R256S 0511 M22 E1").is_ok());
    /// ```
    pub fn fujitsu_mb85r256() -> &'static impl LabelParser<Fram> {
        single_parser!(
            Fram,
            r#"^JAPAN\ (?<kind>MB85R256(A|S))\ (?<year>[0-9]{2})(?<week>[0-9]{2})\ [A-Z][0-9]{2}(\ E1)?$"#,
            move |c| {
                Ok(Fram {
                    kind: c["kind"].to_owned(),
                    manufacturer: Some(Manufacturer::Fujitsu),
                    date_code: Some(ChipDateCode::YearWeek {
                        year: year2(&c["year"])?,
                        week: week2(&c["week"])?,
                    }),
                })
            },
        )
    }
}

pub type Fram = GenericChip;

pub fn fram_sop_28() -> &'static impl LabelParser<Fram> {
    multi_parser!(Fram, sop_28::fujitsu_mb85r256(),)
}
