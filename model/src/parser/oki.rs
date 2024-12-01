// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt, bytes::streaming::tag, character::streaming::char, error::ParseError,
    sequence::tuple, Parser,
};

use super::{
    for_nom::{
        agb_rom_code, alnum_uppers, cgb_rom_code, digits, dmg_rom_code, satisfy_m_n_complete,
        uppers, year1_week2,
    },
    MaskRom,
};
use crate::parser::{Manufacturer, NomParser};

/// OKI old mask ROM chip (QFP-44, 5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_OLD_MASK_ROM.parse("DMG-QXA-0 OKI JAPAN B0 03 X0 02").is_ok());
/// ```
pub static OKI_OLD_MASK_ROM: NomParser<MaskRom> = NomParser {
    name: "OKI old mask ROM",
    f: |input| {
        tuple((
            dmg_rom_code(),
            tag(" OKI JAPAN "),
            alnum_uppers(2),
            char(' '),
            digits(2),
            char(' '),
            alnum_uppers(2),
            char(' '),
            digits(2),
        ))
        .map(|(rom_id, _, _, _, _, _, _, _, _)| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Oki),
            chip_type: None,
            date_code: None,
        })
        .parse(input)
    },
};

fn gb<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    chip_type: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        uppers(1).and(digits(1)),
        char(' '),
        tag(chip_type).and(char('-').and(alnum_uppers(2))),
        char(' '),
        tuple((year1_week2, alnum_uppers(1), digits(2), alnum_uppers(1))),
    ))
    .map(
        move |(rom_id, _, _, _, (kind, _), _, (date_code, _, _, _))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Oki),
            chip_type: Some(format!("{prefix}{kind}")),
            date_code: Some(date_code),
        },
    )
}

/// OKI MSM534011 (SOP-32, 5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MSM534011.parse("CGB-ADME-0 E1 M534011E-09 841232A").is_ok());
/// ```
pub static OKI_MSM534011: NomParser<MaskRom> = NomParser {
    name: "OKI MSM534011",
    f: |input| gb("MS", "M534011E").parse(input),
};

/// OKI MSM538011 (SOP-32, 5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MSM538011.parse("DMG-AM6J-0 F1 M538011E-36 9085401").is_ok());
/// assert!(parser::oki::OKI_MSM538011.parse("CGB-BJWP-0 F1 M538011E-4D 0475408").is_ok());
/// ```
pub static OKI_MSM538011: NomParser<MaskRom> = NomParser {
    name: "OKI MSM538011",
    f: |input| gb("MS", "M538011E").parse(input),
};

/// OKI MR531614 (TSOP-II-44, 5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR531614.parse("CGB-BPTE-0 G2 R531614G-44 044232E").is_ok());
/// ```
pub static OKI_MR531614: NomParser<MaskRom> = NomParser {
    name: "OKI MR531614",
    f: |input| gb("M", "R531614G").parse(input),
};

fn gba<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    chip_type: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        agb_rom_code(),
        char(' '),
        uppers(1).and(digits(1)),
        char(' '),
        tag(chip_type).and(char('-').and(char('0').and(alnum_uppers(2)))),
        char(' '),
        tuple((
            year1_week2,
            satisfy_m_n_complete(4, 5, |c| c.is_ascii_uppercase() || c.is_ascii_digit()),
        )),
    ))
    .map(
        move |(rom_id, _, _, _, (kind, _), _, (date_code, _))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Oki),
            chip_type: Some(format!("{prefix}{kind}")),
            date_code: Some(date_code),
        },
    )
}

/// OKI MR26V3210 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V3210.parse("AGB-TCHK-1 H2 R26V3210F-087 244A239").is_ok());
/// ```
pub static OKI_MR26V3210: NomParser<MaskRom> = NomParser {
    name: "OKI MR26V3210",
    f: |input| gba("M", "R26V3210F").parse(input),
};

/// OKI MR26V3211 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V3211.parse("AGB-BR3P-0 H2 R26V3211F-0T6 442ABAJJ").is_ok());
/// ```
pub static OKI_MR26V3211: NomParser<MaskRom> = NomParser {
    name: "OKI MR26V3211",
    f: |input| gba("M", "R26V3211F").parse(input),
};

/// OKI MR26V6413 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V6413.parse("AGB-A7HJ-0 I2 R26V6413G-0A9 242A273").is_ok());
/// ```
pub static OKI_MR26V6413: NomParser<MaskRom> = NomParser {
    name: "OKI MR26V6413",
    f: |input| gba("M", "R26V6413G").parse(input),
};

/// OKI MR26V6414 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V6414.parse("AGB-AXVJ-0 I2 R26V6414G-0A7 243A262").is_ok());
/// ```
pub static OKI_MR26V6414: NomParser<MaskRom> = NomParser {
    name: "OKI MR26V6414",
    f: |input| gba("M", "R26V6414G").parse(input),
};

/// OKI MR26V6415 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V6415.parse("AGB-BR4J-0 I2 R26V6415G-02L 427ABA3").is_ok());
/// ```
pub static OKI_MR26V6415: NomParser<MaskRom> = NomParser {
    name: "OKI MR26V6415",
    f: |input| gba("M", "R26V6415G").parse(input),
};

/// OKI MR27V810 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR27V810.parse("AGB-FADP-0 F2 R27V810F-059 4475BB4J").is_ok());
/// ```
pub static OKI_MR27V810: NomParser<MaskRom> = NomParser {
    name: "OKI MR27V810",
    f: |input| gba("M", "R27V810F").parse(input),
};

/// OKI MR27V12813 (TSOP-II-44, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR27V12813.parse("AGB-AXPS-1 J2 R27V12813M-0C7 6145BARJ").is_ok());
/// ```
pub static OKI_MR27V12813: NomParser<MaskRom> = NomParser {
    name: "OKI MR27V12813",
    f: |input| gba("M", "R27V12813M").parse(input),
};

/// OKI SGB mask ROM
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_SGB_ROM.parse("SYS-SGB2-10 © 1998 Nintendo M534011E-05 8012354").is_ok());
/// ```
pub static OKI_SGB_ROM: NomParser<MaskRom> = NomParser {
    name: "OKI SGB ROM",
    f: |input| {
        tuple((
            tag("SYS-SGB2-10"),
            tag(" © 1998 Nintendo "),
            tag("M534011E").and(char('-').and(alnum_uppers(2))),
            char(' '),
            tuple((year1_week2, alnum_uppers(1), digits(2), alnum_uppers(1))),
        ))
        .map(
            move |(rom_id, _, (kind, _), _, (date_code, _, _, _))| MaskRom {
                rom_id: String::from(rom_id),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("MS{kind}")),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};
