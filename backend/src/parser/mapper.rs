// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, year2, LabelParser, Manufacturer, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mbc1Version {
    Original,
    A,
    B,
    B1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mbc2Version {
    Original,
    A,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mbc3Version {
    Original,
    A,
    B,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Huc1Version {
    Original,
    A,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapperType {
    Mbc1(Mbc1Version),
    Mbc2(Mbc2Version),
    Mbc3(Mbc3Version),
    Mbc30,
    Mbc5,
    Mbc6,
    Mbc7,
    Huc1(Huc1Version),
    Huc3,
    Mmm01,
}

impl MapperType {
    pub fn display_name(&self) -> &'static str {
        match self {
            MapperType::Mbc1(Mbc1Version::Original) => "MBC1",
            MapperType::Mbc1(Mbc1Version::A) => "MBC1A",
            MapperType::Mbc1(Mbc1Version::B) => "MBC1B",
            MapperType::Mbc1(Mbc1Version::B1) => "MBC1B1",
            MapperType::Mbc2(Mbc2Version::Original) => "MBC2",
            MapperType::Mbc2(Mbc2Version::A) => "MBC2A",
            MapperType::Mbc3(Mbc3Version::Original) => "MBC3",
            MapperType::Mbc3(Mbc3Version::A) => "MBC3A",
            MapperType::Mbc3(Mbc3Version::B) => "MBC3B",
            MapperType::Mbc30 => "MBC30",
            MapperType::Mbc5 => "MBC5",
            MapperType::Mbc6 => "MBC6",
            MapperType::Mbc7 => "MBC7",
            MapperType::Mmm01 => "MMM01",
            MapperType::Huc3 => "HuC-3",
            MapperType::Huc1(Huc1Version::Original) => "HuC-1",
            MapperType::Huc1(Huc1Version::A) => "HuC-1A",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mapper {
    pub mbc_type: MapperType,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

/// Sharp MBC1A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc1a().parse("DMG MBC1A Nintendo S 9025 1 A").is_ok());
/// ```
pub fn sharp_mbc1a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC1A\ Nintendo\ S\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::A),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sharp MBC1B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc1b().parse("DMG MBC1B Nintendo S 9107 5 A").is_ok());
/// ```
pub fn sharp_mbc1b() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC1B\ Nintendo\ S\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sharp MBC1B1
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc1b1().parse("DMG MBC1B1 Nintendo S 9838 5 A").is_ok());
/// ```
pub fn sharp_mbc1b1() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC1B1\ Nintendo\ S\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B1),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sharp MBC2A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc2a().parse("DMG MBC2A Nintendo S 9730 5 AB").is_ok());
/// ```
pub fn sharp_mbc2a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC2A\ Nintendo\ S\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc2(Mbc2Version::A),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sharp MBC3
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc3().parse("MBC3 LR385364 9743 A").is_ok());
/// ```
pub fn sharp_mbc3() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC3\ LR385364\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::Original),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sharp MBC3A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc3a().parse("MBC3 A LR38536B 9935 A").is_ok());
/// ```
pub fn sharp_mbc3a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC3\ A\ LR38536B\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::A),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Sharp MBC5
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::sharp_mbc5().parse("MBC5 LZ9GB31 AL23 A").is_ok());
/// ```
pub fn sharp_mbc5() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC5\ LZ9GB31\ ([[:alnum:]]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc5,
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// NEC MBC1B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::nec_mbc1b().parse("Nintendo DMG MBC1B N 9019BA012").is_ok());
/// ```
pub fn nec_mbc1b() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^Nintendo\ DMG\ MBC1B\ N\ ([0-9]{2})([0-9]{2})BA[0-9]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B),
                manufacturer: Some(Manufacturer::Nec),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// NEC MBC2A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::nec_mbc2a().parse("Nintendo DMG MBC2A N 9011CA005").is_ok());
/// ```
pub fn nec_mbc2a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^Nintendo\ DMG\ MBC2A\ N\ ([0-9]{2})([0-9]{2})CA[0-9]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc2(Mbc2Version::A),
                manufacturer: Some(Manufacturer::Nec),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC6 with NEC-like labeling
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::nec_like_mbc6().parse("Nintendo MBC6 0103XPO14").is_ok());
/// ```
pub fn nec_like_mbc6() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^Nintendo\ MBC6\ ([0-9]{2})([0-9]{2})XPO[0-9]{2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc6,
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC1B by some "P" company?
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::p_company_mbc1b().parse("DMG MBC1-B Nintendo P 0'D7").is_ok());
/// ```
pub fn p_company_mbc1b() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC1-B\ Nintendo\ P\ ([0-9])'[[:alnum:]][0-9]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: None,
            })
        },
    )
}

/// Unknown MBC2A by some "P" company?
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::p_company_mbc2a().parse("DMG MBC2-A Nintendo P 8'73").is_ok());
/// ```
pub fn p_company_mbc2a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC2-A\ Nintendo\ P\ ([0-9])'[[:alnum:]][0-9]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc2(Mbc2Version::A),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: None,
            })
        },
    )
}

/// Unknown MBC3A by some "P" company?
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::p_company_mbc3a().parse("MBC3 A P-2 834U4E").is_ok());
/// ```
pub fn p_company_mbc3a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC3\ A\ P-2\ ([0-9])([0-9]{2})U[0-9][A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::A),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC3B by some "P" company?
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::p_company_mbc3b().parse("MBC3 B P-2 134U2D").is_ok());
/// ```
pub fn p_company_mbc3b() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC3\ B\ P-2\ ([0-9])([0-9]{2})U[0-9][A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::B),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC30 by some "P" company?
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::p_company_mbc30().parse("MBC30 P 047U2M").is_ok());
/// ```
pub fn p_company_mbc30() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC30\ P\ ([0-9])([0-9]{2})[[:alnum:]][0-9][A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc30,
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC5 by some "P" company?
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::p_company_mbc5().parse("MBC5 P 041U7M").is_ok());
/// ```
pub fn p_company_mbc5() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC5\ P(-[0-9])?\ ([0-9])([0-9]{2})U[0-9][A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc5,
                manufacturer: None,
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Rohm MBC3
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::rohm_mbc3().parse("MBC3 BU3631K 802 127").is_ok());
/// ```
pub fn rohm_mbc3() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC3\ BU3631K\ ([0-9])([0-9]{2})\ [0-9]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::Original),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Rohm MBC3A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::rohm_mbc3a().parse("MBC-3 A BU3632K 004 H64").is_ok());
/// ```
pub fn rohm_mbc3a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC-3\ A\ BU3632K\ ([0-9])([0-9]{2})\ [[:alnum:]]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::A),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Rohm MBC3B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::rohm_mbc3b().parse("MBC-3 B BU3634K 135 H48").is_ok());
/// ```
pub fn rohm_mbc3b() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC-3\ B\ BU3634K\ ([0-9])([0-9]{2})\ H[0-9]{2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc3(Mbc3Version::B),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Rohm MBC30
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::rohm_mbc30().parse("MBC-30 BU3633AK 046 175").is_ok());
/// ```
pub fn rohm_mbc30() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC-30\ BU3633AK\ ([0-9])([0-9]{2})\ [0-9]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc30,
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Rohm MBC5
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::rohm_mbc5().parse("MBC5 BU3650K 229 H51").is_ok());
/// assert!(parser::mapper::rohm_mbc5().parse("MBC-5 BU3650K 049 186").is_ok());
/// ```
pub fn rohm_mbc5() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC-?5\ BU3650K\ ([0-9])([0-9]{2})\ [[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc5,
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Rohm MBC7
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::rohm_mbc7().parse("MBC-7 BU3667KS 041 170").is_ok());
/// ```
pub fn rohm_mbc7() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MBC-7\ BU3667KS\ ([0-9])([0-9]{2})\ [0-9]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc7,
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Texas Instruments MBC5
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::texas_instruments_mbc5().parse("11CH8VT MBC5 2417").is_ok());
/// ```
pub fn texas_instruments_mbc5() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^([0-9])[[:alnum:]][A-Z][[:alnum:]]{3}T\ MBC5\ 2417$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc5,
                manufacturer: Some(Manufacturer::TexasInstruments),
                year: Some(year1(&c[1])?),
                week: None,
            })
        },
    )
}

/// Motorola MBC1B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::unknown_mbc1b().parse("DMG MBC1B Nintendo J9130BR").is_ok());
/// ```
pub fn unknown_mbc1b() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^DMG\ MBC1B\ Nintendo\ J([0-9]{2})([0-9]{2})BR$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B),
                manufacturer: Some(Manufacturer::Motorola),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC1B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::unknown_mbc1b_2().parse("Nintendo DMG MBC1B 8940AJ").is_ok());
/// ```
pub fn unknown_mbc1b_2() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^Nintendo\ DMG\ MBC1B\ ([0-9]{2})([0-9]{2})AJ$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B),
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Unknown MBC1B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::unknown_mbc1b_3().parse("Nintendo DMG MBC1B N9542B3004").is_ok());
/// ```
pub fn unknown_mbc1b_3() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^Nintendo\ DMG\ MBC1B\ N([0-9]{2})([0-9]{2})B[0-9]{4}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mbc1(Mbc1Version::B),
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Hudson HuC-1
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::huc1().parse("HuC-1 © HUDSON Nintendo 9752 A").is_ok());
/// ```
pub fn huc1() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^HuC-1\ ©\ HUDSON\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Huc1(Huc1Version::Original),
                manufacturer: Some(Manufacturer::Hudson),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Hudson HuC-1A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::huc1a().parse("HuC1A © HUDSON Nintendo 9845 A").is_ok());
/// ```
pub fn huc1a() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^HuC1A\ ©\ HUDSON\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Huc1(Huc1Version::A),
                manufacturer: Some(Manufacturer::Hudson),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Hudson HuC-3
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::huc3().parse("HuC-3 © HUDSON Nintendo 9943 A").is_ok());
/// ```
pub fn huc3() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^HuC-3\ ©\ HUDSON\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Huc3,
                manufacturer: Some(Manufacturer::Hudson),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// MMM01
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mapper::mmm01().parse("MMM01 645 113").is_ok());
/// ```
pub fn mmm01() -> &'static impl LabelParser<Mapper> {
    single_parser!(
        Mapper,
        r#"^MMM01\ ([0-9])([0-9]{2})\ [0-9]{3}$"#,
        move |c| {
            Ok(Mapper {
                mbc_type: MapperType::Mmm01,
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

pub fn mapper() -> &'static impl LabelParser<Mapper> {
    multi_parser!(
        Mapper,
        sharp_mbc1a(),
        sharp_mbc1b(),
        sharp_mbc1b1(),
        sharp_mbc2a(),
        sharp_mbc3(),
        sharp_mbc3a(),
        sharp_mbc5(),
        nec_mbc1b(),
        nec_mbc2a(),
        nec_like_mbc6(),
        p_company_mbc1b(),
        p_company_mbc2a(),
        p_company_mbc3a(),
        p_company_mbc3b(),
        p_company_mbc30(),
        p_company_mbc5(),
        rohm_mbc3(),
        rohm_mbc3a(),
        rohm_mbc3b(),
        rohm_mbc30(),
        rohm_mbc5(),
        rohm_mbc7(),
        texas_instruments_mbc5(),
        unknown_mbc1b(),
        unknown_mbc1b_2(),
        unknown_mbc1b_3(),
        huc1(),
        huc1a(),
        huc3(),
        mmm01(),
    )
}
