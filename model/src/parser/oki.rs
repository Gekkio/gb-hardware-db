// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{consumed, value},
    error::ParseError,
    sequence::{terminated, tuple},
    Parser,
};

use super::{
    for_nom::{
        agb_rom_code, alnum_uppers, cgb_rom_code, digits, dmg_rom_code, satisfy_m_n_complete,
        year1_week2,
    },
    GameMaskRom, GameRomType, MaskCode, MaskRom,
};
use crate::parser::{Manufacturer, NomParser};

/// OKI unknown mask ROM (QFP-44, 5V, 512 Kibit / 64 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MASK_ROM_QFP_44_512_KIBIT.parse("DMG-QXA-0 OKI JAPAN B0 03 X0 02").is_ok());
/// ```
pub static OKI_MASK_ROM_QFP_44_512_KIBIT: NomParser<GameMaskRom> = NomParser {
    name: "OKI mask ROM",
    f: |input| {
        let rom_type = GameRomType::B0;
        tuple((
            dmg_rom_code(),
            tag(" OKI JAPAN "),
            tag(rom_type.as_str()),
            char(' '),
            digits(2),
            char(' '),
            alnum_uppers(2),
            char(' '),
            digits(2),
        ))
        .map(|(rom_id, _, _, _, _, _, _, _, _)| GameMaskRom {
            rom_id: String::from(rom_id),
            rom_type,
            manufacturer: Some(Manufacturer::Oki),
            chip_type: None,
            mask_code: None,
            date_code: None,
        })
        .parse(input)
    },
};

fn gb<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    chip_type: &'static str,
    rom_type: GameRomType,
) -> impl Parser<&'a str, GameMaskRom, E> {
    tuple((
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        tag(rom_type.as_str()),
        char(' '),
        consumed(terminated(tag(chip_type), char('-').and(alnum_uppers(2)))),
        char(' '),
        tuple((year1_week2, alnum_uppers(1), digits(2), alnum_uppers(1))),
    ))
    .map(
        move |(rom_id, _, _, _, (mask_code, kind), _, (date_code, _, _, _))| GameMaskRom {
            rom_id: String::from(rom_id),
            rom_type,
            manufacturer: Some(Manufacturer::Oki),
            chip_type: Some(format!("{prefix}{kind}")),
            mask_code: Some(MaskCode::Oki(String::from(mask_code))),
            date_code: Some(date_code),
        },
    )
}

/// OKI MSM534011 (SOP-32, 5V, 4 Mibit / 512 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MSM534011.parse("CGB-ADME-0 E1 M534011E-09 841232A").is_ok());
/// ```
pub static OKI_MSM534011: NomParser<GameMaskRom> = NomParser {
    name: "OKI MSM534011",
    f: |input| gb("MS", "M534011E", GameRomType::E1).parse(input),
};

/// OKI MSM538011 (SOP-32, 5V, 8 Mibit / 1 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MSM538011.parse("DMG-AM6J-0 F1 M538011E-36 9085401").is_ok());
/// assert!(parser::oki::OKI_MSM538011.parse("CGB-BJWP-0 F1 M538011E-4D 0475408").is_ok());
/// ```
pub static OKI_MSM538011: NomParser<GameMaskRom> = NomParser {
    name: "OKI MSM538011",
    f: |input| gb("MS", "M538011E", GameRomType::F1).parse(input),
};

/// OKI MR531614 (TSOP-II-44, 5V, 16 Mibit / 2 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR531614.parse("CGB-BPTE-0 G2 R531614G-44 044232E").is_ok());
/// ```
pub static OKI_MR531614: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR531614",
    f: |input| gb("M", "R531614G", GameRomType::G2).parse(input),
};

fn gba<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    chip_type: &'static str,
    rom_type: GameRomType,
) -> impl Parser<&'a str, GameMaskRom, E> {
    tuple((
        agb_rom_code(),
        char(' '),
        tag(rom_type.as_str()),
        char(' '),
        consumed(terminated(tag(chip_type), tag("-0").and(alnum_uppers(2)))),
        char(' '),
        tuple((
            year1_week2,
            satisfy_m_n_complete(4, 5, |c| c.is_ascii_uppercase() || c.is_ascii_digit()),
        )),
    ))
    .map(
        move |(rom_id, _, _, _, (mask_code, kind), _, (date_code, _))| GameMaskRom {
            rom_id: String::from(rom_id),
            rom_type,
            manufacturer: Some(Manufacturer::Oki),
            chip_type: Some(format!("{prefix}{kind}")),
            mask_code: Some(MaskCode::Oki(String::from(mask_code))),
            date_code: Some(date_code),
        },
    )
}

/// OKI MR26V3210 (TSOP-II-44, 3.3V, 32 Mibit / 4 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V3210.parse("AGB-TCHK-1 H2 R26V3210F-087 244A239").is_ok());
/// ```
pub static OKI_MR26V3210: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR26V3210",
    f: |input| gba("M", "R26V3210F", GameRomType::H2).parse(input),
};

/// OKI MR26V3211 (TSOP-II-44, 3.3V, 32 Mibit / 4 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V3211.parse("AGB-BR3P-0 H2 R26V3211F-0T6 442ABAJJ").is_ok());
/// ```
pub static OKI_MR26V3211: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR26V3211",
    f: |input| gba("M", "R26V3211F", GameRomType::H2).parse(input),
};

/// OKI MR26V6413 (TSOP-II-44, 3.3V, 64 Mibit / 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V6413.parse("AGB-A7HJ-0 I2 R26V6413G-0A9 242A273").is_ok());
/// ```
pub static OKI_MR26V6413: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR26V6413",
    f: |input| gba("M", "R26V6413G", GameRomType::I2).parse(input),
};

/// OKI MR26V6414 (TSOP-II-44, 3.3V, 64 Mibit / 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V6414.parse("AGB-AXVJ-0 I2 R26V6414G-0A7 243A262").is_ok());
/// ```
pub static OKI_MR26V6414: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR26V6414",
    f: |input| gba("M", "R26V6414G", GameRomType::I2).parse(input),
};

/// OKI MR26V6415 (TSOP-II-44, 3.3V, 64 Mibit / 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR26V6415.parse("AGB-BR4J-0 I2 R26V6415G-02L 427ABA3").is_ok());
/// ```
pub static OKI_MR26V6415: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR26V6415",
    f: |input| gba("M", "R26V6415G", GameRomType::I2).parse(input),
};

/// OKI MR27V810 (TSOP-II-44, 3.3V, 8 Mibit / 1 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR27V810.parse("AGB-FADP-0 F2 R27V810F-059 4475BB4J").is_ok());
/// ```
pub static OKI_MR27V810: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR27V810",
    f: |input| gba("M", "R27V810F", GameRomType::F2).parse(input),
};

/// OKI MR27V6416 (TSOP-II-44, 3.3V, 64 Mibit / 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR27V6416.parse("AGB-B2LP-0 I2 R27V6416M-0TB 6445BJ9J").is_ok());
/// ```
pub static OKI_MR27V6416: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR27V6416",
    f: |input| gba("M", "R27V6416M", GameRomType::I2).parse(input),
};

/// OKI MR27V12813 (TSOP-II-44, 3.3V, 128 Mibit / 16 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::oki::OKI_MR27V12813.parse("AGB-AXPS-1 J2 R27V12813M-0C7 6145BARJ").is_ok());
/// ```
pub static OKI_MR27V12813: NomParser<GameMaskRom> = NomParser {
    name: "OKI MR27V12813",
    f: |input| gba("M", "R27V12813M", GameRomType::J2).parse(input),
};

/// OKI SGB mask ROM, MSM534011 (SOP-32, 5V, 4 Mibit / 512 KiB)
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
            consumed(value("MSM534011E", tag("M534011E-05"))),
            char(' '),
            tuple((year1_week2, alnum_uppers(1), digits(2), alnum_uppers(1))),
        ))
        .map(
            move |(rom_id, _, (mask_code, kind), _, (date_code, _, _, _))| MaskRom {
                rom_id: String::from(rom_id),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(String::from(kind)),
                mask_code: Some(MaskCode::Oki(String::from(mask_code))),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};
