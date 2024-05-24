// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, LabelParser, ParsedData, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Gen1SocKind {
    Dmg0,
    DmgA,
    DmgB,
    DmgC,
    DmgBlobB,
    DmgBlobC,
    Sgb,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gen1Soc {
    pub kind: Gen1SocKind,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

impl ParsedData for Gen1Soc {}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gen1_soc::dmg_cpu_lr35902().parse("DMG-CPU LR35902 8907 D").is_ok());
/// ```
pub fn dmg_cpu_lr35902() -> &'static impl LabelParser<Gen1Soc> {
    single_parser!(
        Gen1Soc,
        r#"^DMG-CPU\ LR35902\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(Gen1Soc {
                kind: Gen1SocKind::Dmg0,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gen1_soc::dmg_cpu().parse("DMG-CPU © 1989 Nintendo JAPAN 8913 D").is_ok());
/// assert!(parser::gen1_soc::dmg_cpu().parse("DMG-CPU A © 1989 Nintendo JAPAN 8937 D").is_ok());
/// assert!(parser::gen1_soc::dmg_cpu().parse("DMG-CPU B © 1989 Nintendo JAPAN 9207 D").is_ok());
/// assert!(parser::gen1_soc::dmg_cpu().parse("DMG-CPU C © 1989 Nintendo JAPAN 9835 D").is_ok());
/// ```
pub fn dmg_cpu() -> &'static impl LabelParser<Gen1Soc> {
    single_parser!(
        Gen1Soc,
        r#"^DMG-CPU(\ [ABC])?\ ©\ 1989\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Gen1Soc {
                kind: (match c.get(1).map(|m| m.as_str()) {
                    Some(" A") => Ok(Gen1SocKind::DmgA),
                    Some(" B") => Ok(Gen1SocKind::DmgB),
                    Some(" C") => Ok(Gen1SocKind::DmgC),
                    Some(text) => Err(format!("Invalid DMG-CPU part name: {}", text)),
                    _ => Ok(Gen1SocKind::Dmg0),
                })?,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

pub fn dmg_cpu_deprecated() -> &'static impl LabelParser<Gen1Soc> {
    single_parser!(
        Gen1Soc,
        r#"^DMG-CPU(\ [A-B])?\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Gen1Soc {
                kind: (match c.get(1).map(|m| m.as_str()) {
                    Some(" A") => Ok(Gen1SocKind::DmgA),
                    Some(" B") => Ok(Gen1SocKind::DmgB),
                    Some(text) => Err(format!("Invalid DMG-CPU part name: {}", text)),
                    _ => Ok(Gen1SocKind::Dmg0),
                })?,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gen1_soc::dmg_cpu_blob().parse("B").is_ok());
/// assert!(parser::gen1_soc::dmg_cpu_blob().parse("C").is_ok());
/// ```
pub fn dmg_cpu_blob() -> &'static impl LabelParser<Gen1Soc> {
    single_parser!(Gen1Soc, r#"^[BC]$"#, move |c| {
        Ok(Gen1Soc {
            kind: (match &c[0] {
                "B" => Ok(Gen1SocKind::DmgBlobB),
                "C" => Ok(Gen1SocKind::DmgBlobC),
                text => Err(format!("Invalid DMG-CPU part name: {}", text)),
            })?,
            year: None,
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gen1_soc::sgb_cpu().parse("SGB-CPU 01 © 1994 Nintendo Ⓜ 1989 Nintendo JAPAN 9434 7 D").is_ok());
/// ```
pub fn sgb_cpu() -> &'static impl LabelParser<Gen1Soc> {
    single_parser!(
        Gen1Soc,
        r#"^SGB-CPU\ 01\ ©\ 1994\ Nintendo\ Ⓜ\ 1989\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Gen1Soc {
                kind: Gen1SocKind::Sgb,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn gen1_soc() -> &'static impl LabelParser<Gen1Soc> {
    multi_parser!(
        Gen1Soc,
        dmg_cpu(),
        dmg_cpu_blob(),
        dmg_cpu_lr35902(),
        dmg_cpu_deprecated(),
        sgb_cpu(),
    )
}
