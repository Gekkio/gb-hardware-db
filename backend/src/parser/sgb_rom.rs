// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, year2, LabelParser, Manufacturer, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SgbRom {
    pub rom_code: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::unknown().parse("SYS-SGB-2 © 1994 Nintendo 9429 R77").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::unknown2().parse("SYS-SGB-2 © 1994 Nintendo 9423 E").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::unknown3().parse("SYS-SGB-2 JAPAN © 1994 Nintendo 427A2 A04 NND").is_ok());
/// ```
pub fn unknown3() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^(SYS-SGB-(NT|2))\ JAPAN\ ©\ 1994\ Nintendo\ [[:alnum:]]{5}\ [[:alnum:]]{3}\ [A-Z]{3}$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                year: None,
                week: None,
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::unknown4().parse("© 1994 Nintendo SYS-SGB-NT N-2001EGW-J56 9414X9013").is_ok());
/// ```
pub fn unknown4() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^©\ 1994\ Nintendo\ (SYS-SGB-(NT|2))\ (N-[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Toshiba SGB ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::toshiba().parse("SYS-SGB-2 © 1994 Nintendo TC532000BF-N807 JAPAN 9431EAI").is_ok());
/// ```
pub fn toshiba() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ (TC53[0-9]{4}[A-Z]{2})-[A-Z][0-9]{3}\ JAPAN\ ([0-9]{2})([0-9]{2})EAI$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Toshiba),
                chip_type: (Some(c[3].to_owned())),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Sharp SGB ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::sharp_sgb().parse("SYS-SGB-2 © 1994 Nintendo LH532M0M 9432 E").is_ok());
/// ```
pub fn sharp_sgb() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^(SYS-SGB-NT|SYS-SGB-2)\ ©\ 1994\ Nintendo\ (LH[[:alnum:]]{4})[[:alnum:]]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(c[2].to_owned()),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Sharp SGB2 ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::sharp_sgb2().parse("© 1998 Nintendo SYS-SGB2-10 LH5S4RY4 0003 D").is_ok());
/// ```
pub fn sharp_sgb2() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^©\ 1998\ Nintendo\ (SYS-SGB2-10)\ (LH[[:alnum:]]{4})[[:alnum:]]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(c[2].to_owned()),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// OKI Semiconductor SGB/SGB2 ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sgb_rom::oki().parse("SYS-SGB2-10 © 1998 Nintendo M534011E-05 8012354").is_ok());
/// ```
pub fn oki() -> &'static impl LabelParser<SgbRom> {
    single_parser!(
        SgbRom,
        r#"^(SYS-SGB-NT|SYS-SGB-2|SYS-SGB2-10)\ ©\ 1998\ Nintendo\ (M534011E)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(c[2].to_owned()),
                year: Some(year1(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

pub fn sgb_rom() -> &'static impl LabelParser<SgbRom> {
    multi_parser!(
        SgbRom,
        toshiba(),
        sharp_sgb(),
        sharp_sgb2(),
        oki(),
        unknown(),
        unknown2(),
        unknown3(),
        unknown4(),
    )
}
