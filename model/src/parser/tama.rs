// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, GameMaskRom, GenericPart, LabelParser};
use crate::{
    macros::single_parser,
    parser::{GameRomType, Mapper, MapperType, PartDateCode},
};

/// TAMA5
///
/// ```
/// # use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::tama::tama5().parse("TAMA5 9726 EAD1").is_ok());
/// ```
pub fn tama5() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^TAMA5\ ([0-9]{2})([0-9]{2})\ EA[A-Z]1$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Tama5,
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// TAMA6
///
/// ```
/// # use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::tama::tama6().parse("TAMA6 JAPAN 47C243M FV61 9751H").is_ok());
/// ```
pub fn tama6() -> &'static impl LabelParser<GenericPart> {
    single_parser!(
        GenericPart,
        r#"^TAMA6\ JAPAN\ 47C243M\ FV61\ ([0-9]{2})([0-9]{2})H$"#,
        move |c| {
            Ok(GenericPart {
                kind: String::from("TAMA6"),
                manufacturer: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[1])?,
                    week: week2(&c[2])?,
                }),
            })
        },
    )
}

/// TAMA7 ROM
///
/// ```
/// # use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::tama::tama7().parse("TAMA7 B9748 43913A TAIWAN").is_ok());
/// ```
pub fn tama7() -> &'static impl LabelParser<GameMaskRom> {
    single_parser!(
        GameMaskRom,
        r#"^TAMA7\ [A-Z]([0-9]{2})([0-9]{2})\ [0-9]{5}[A-Z]\ TAIWAN$"#,
        move |c| {
            Ok(GameMaskRom {
                rom_id: String::from("DMG-AOMJ-0"),
                rom_type: GameRomType::E1,
                manufacturer: None,
                chip_type: None,
                mask_code: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[1])?,
                    week: week2(&c[2])?,
                }),
            })
        },
    )
}
