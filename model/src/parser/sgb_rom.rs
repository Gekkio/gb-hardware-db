// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, LabelParser};
use crate::{
    macros::{multi_parser, single_parser},
    parser::{fujitsu, nec, oki, sharp, toshiba, MaskRom, PartDateCode},
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::unknown2().parse("SYS-SGB-2 © 1994 Nintendo 9423 E").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[3])?,
                    week: week2(&c[4])?,
                }),
            })
        },
    )
}

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::unknown3().parse("SYS-SGB-2 JAPAN © 1994 Nintendo 427A2 A04 NND").is_ok());
/// ```
pub fn unknown3() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^(SYS-SGB-(NT|2))\ JAPAN\ ©\ 1994\ Nintendo\ [[:alnum:]]{5}\ [[:alnum:]]{3}\ [A-Z]{3}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                date_code: None,
            })
        },
    )
}

pub fn sgb_rom() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        &toshiba::TOSHIBA_SGB_ROM,
        &sharp::SHARP_SGB_ROM,
        &sharp::SHARP_SGB2_ROM,
        &oki::OKI_SGB_ROM,
        &fujitsu::FUJITSU_SGB_ROM,
        unknown2(),
        unknown3(),
        &nec::NEC_SGB_ROM,
    )
}
