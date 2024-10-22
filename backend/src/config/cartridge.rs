// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    any::Any,
    collections::{BTreeMap, HashMap},
    fmt,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    sync::OnceLock,
};

use crate::{
    hash::{Crc32, Md5, Sha1, Sha256},
    parser::{
        accelerometer::accelerometer,
        crystal_32kihz::crystal_32kihz,
        eeprom::eeprom,
        flash::{flash_tsop_i_32, flash_tsop_i_40},
        fram::fram_sop_28,
        hex_inverter::hex_inverter,
        line_decoder::line_decoder,
        mapper,
        mask_rom::{
            agb_mask_rom_tsop_ii_44, mask_rom_glop_top_28, mask_rom_qfp_44, mask_rom_sop_32,
            mask_rom_tsop_i_32, mask_rom_tsop_ii_44,
        },
        rtc::{rtc_sop_20, rtc_sop_8},
        sram::{sram_sop_28, sram_sop_32, sram_tsop_i_28},
        supervisor_reset::supervisor_reset,
        tama::tama,
        unknown_chip, LabelParser, ParsedData,
    },
};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameConfig {
    #[serde(skip, default)]
    pub rom_id: String,
    pub name: String,
    pub rom_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32: Option<Crc32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<Md5>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<Sha1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<Sha256>,
    pub platform: GamePlatform,
    pub no_intro_id: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum GamePlatform {
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "gbc")]
    Gbc,
    #[serde(rename = "gba")]
    Gba,
}

impl GamePlatform {
    pub const ALL: [GamePlatform; 3] = [GamePlatform::Gb, GamePlatform::Gbc, GamePlatform::Gba];
    pub const fn id(&self) -> &'static str {
        match self {
            GamePlatform::Gb => "gb",
            GamePlatform::Gbc => "gbc",
            GamePlatform::Gba => "gba",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            GamePlatform::Gb => "Game Boy",
            GamePlatform::Gbc => "Game Boy Color",
            GamePlatform::Gba => "Game Boy Advance",
        }
    }
    pub const fn has_mappers(&self) -> bool {
        match self {
            GamePlatform::Gb | GamePlatform::Gbc => true,
            GamePlatform::Gba => false,
        }
    }
}

impl fmt::Display for GamePlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GamePlatform::Gb => write!(f, "GB"),
            GamePlatform::Gbc => write!(f, "GBC"),
            GamePlatform::Gba => write!(f, "GBA"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BoardConfig {
    AgbArc,
    AgbE01,
    AgbE02,
    AgbE03,
    AgbE05,
    AgbE06,
    AgbE11,
    AgbE18,
    AgbE24,
    AgbY11,
    Tama,
    Aaac,
    CgbA32,
    DmgA02,
    DmgA03,
    DmgA04,
    DmgA06,
    DmgA07,
    DmgA08,
    DmgA09,
    DmgA11,
    DmgA14,
    DmgA15,
    DmgA16,
    DmgA40,
    DmgA47,
    DmgAaa,
    DmgBba,
    DmgBca,
    DmgBean,
    DmgBfan,
    DmgDecn,
    DmgDedn,
    DmgDgcu,
    DmgGdan,
    DmgKecn,
    DmgKfcn,
    DmgKfdn,
    DmgKgdu,
    DmgLfdn,
    DmgMBfan,
    DmgMcDfcn,
    DmgMcSfcn,
    DmgMheu,
    DmgTedn,
    DmgTfdn,
    DmgUedt,
    DmgUfdt,
    DmgUgdu,
    DmgZ02,
    DmgZ03,
    DmgZ04,
}

impl BoardConfig {
    pub fn part(&self, designator: PartDesignator) -> Option<BoardPart> {
        use PartDesignator as D;

        fn part<T: ParsedData + 'static>(
            role: PartRole,
            parser: &'static impl LabelParser<T>,
        ) -> Option<BoardPart> {
            Some(BoardPart {
                role,
                parser: Box::new(move |input| {
                    let value = parser.parse(input)?;
                    Ok(Box::new(value))
                }),
                parse_any: Box::new(move |input| {
                    let value = parser.parse(input)?;
                    Ok(Box::new(value))
                }),
            })
        }

        match self {
            BoardConfig::AgbArc => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // SOP-28 FRAM
                D::U2 => part(PartRole::Ram, fram_sop_28()),
                _ => None,
            },
            BoardConfig::AgbE01 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                _ => None,
            },
            BoardConfig::AgbE02 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // TSOP-I-32 Flash
                D::U2 => part(PartRole::Flash, flash_tsop_i_32()),
                _ => None,
            },
            BoardConfig::AgbE03 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // SOP-8 EEPROM
                D::U2 => part(PartRole::Eeprom, eeprom()),
                _ => None,
            },
            BoardConfig::AgbE05 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // TSOP-I-32 Flash
                D::U2 => part(PartRole::Flash, flash_tsop_i_32()),
                // SOP-8 RTC
                D::U3 => part(PartRole::Rtc, rtc_sop_8()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::AgbE06 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // SOP-28 RAM
                D::U2 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 BU9803F
                D::U3 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::AgbE11 | BoardConfig::AgbY11 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // SOP-28 FRAM
                D::U2 => part(PartRole::Ram, fram_sop_28()),
                _ => None,
            },
            BoardConfig::AgbE18 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // SOP-8 EEPROM
                D::U2 => part(PartRole::Eeprom, eeprom()),
                // SOP-8 RTC
                D::U3 => part(PartRole::Rtc, rtc_sop_8()),
                D::U4 => part(PartRole::Unknown, unknown_chip()),
                D::U5 => part(PartRole::Unknown, unknown_chip()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::AgbE24 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, agb_mask_rom_tsop_ii_44()),
                // SOP-8 EEPROM
                D::U2 => part(PartRole::Eeprom, eeprom()),
                _ => None,
            },
            BoardConfig::Tama => match designator {
                // SOP-32 TAMA7
                D::U1 => part(PartRole::Rom, tama()),
                // SOP-28 TAMA5
                D::U2 => part(PartRole::Mapper, tama()),
                // SOP-28 TAMA6
                D::U3 => part(PartRole::Mcu, tama()),
                // SOP-20
                D::U4 => part(PartRole::Rtc, rtc_sop_20()),
                // SOP-8 M62021P
                D::U5 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::Aaac => match designator {
                // glop top ROM, 28 pads
                D::U1 => part(PartRole::Rom, mask_rom_glop_top_28()),
                _ => None,
            },
            BoardConfig::CgbA32 => match designator {
                // QFP-64 MBC6
                D::U1 => part(PartRole::Mapper, mapper::mbc6_qfp64()),
                // SOP-32 ROM
                D::U2 => part(PartRole::Rom, mask_rom_sop_32()),
                // TSOP-I-40 Flash
                D::U3 => part(PartRole::Flash, flash_tsop_i_40()),
                // SOP-28 RAM
                D::U4 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134
                D::U5 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA02 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA03 | BoardConfig::DmgA08 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA04 => match designator {
                // TSOP-I-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_i_32()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // MOT1 => motor
                _ => None,
            },
            BoardConfig::DmgA06 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA07 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                _ => None,
            },
            BoardConfig::DmgA09 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                _ => None,
            },
            BoardConfig::DmgA11 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // MOT1 => motor
                _ => None,
            },
            BoardConfig::DmgA14 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-32 RAM
                D::U3 => part(PartRole::Ram, sram_sop_32()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA15 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSOP-II-44 ROM
                D::U5 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // SSOP-8
                D::U6 => part(PartRole::LineDecoder, line_decoder()),
                _ => None,
            },
            BoardConfig::DmgA16 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-32 RAM
                D::U3 => part(PartRole::Ram, sram_sop_32()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA40 => match designator {
                // QFP-56 MBC7
                D::U1 => part(PartRole::Mapper, mapper::mbc7_qfp56()),
                // SOP-32 ROM
                D::U2 => part(PartRole::Rom, mask_rom_sop_32()),
                // TSSOP-8 EEPROM
                D::U3 => part(PartRole::Eeprom, eeprom()),
                // QC-14 accelerometer
                D::U4 => part(PartRole::Accelerometer, accelerometer()),
                _ => None,
            },
            BoardConfig::DmgA47 => match designator {
                // QFP-56 MBC7
                D::U1 => part(PartRole::Mapper, mapper::mbc7_qfp56()),
                // TSOP-II-44 ROM
                D::U2 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // TSSOP-8 EEPROM
                D::U3 => part(PartRole::Eeprom, eeprom()),
                // QC-14 accelerometer
                D::U4 => part(PartRole::Accelerometer, accelerometer()),
                _ => None,
            },
            BoardConfig::DmgAaa => match designator {
                // QFP-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_qfp_44()),
                _ => None,
            },
            BoardConfig::DmgBba | BoardConfig::DmgBca => match designator {
                // QFP-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_qfp_44()),
                // SOP-24 MBC1
                D::U2 => part(PartRole::Mapper, mapper::mbc1_sop24()),
                _ => None,
            },
            BoardConfig::DmgBean | BoardConfig::DmgBfan | BoardConfig::DmgMBfan => match designator
            {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // SOP-24 MBC1
                D::U2 => part(PartRole::Mapper, mapper::mbc1_sop24()),
                _ => None,
            },
            BoardConfig::DmgDecn | BoardConfig::DmgDedn | BoardConfig::DmgMcDfcn => {
                match designator {
                    // SOP-32 ROM
                    D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                    // SOP-24 MBC1
                    D::U2 => part(PartRole::Mapper, mapper::mbc1_sop24()),
                    // SOP-28 RAM
                    D::U3 => part(PartRole::Ram, sram_sop_28()),
                    // SOP-8 26A
                    D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                    _ => None,
                }
            }
            BoardConfig::DmgDgcu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // SOP-24 MBC1
                D::U2 => part(PartRole::Mapper, mapper::mbc1_sop24()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgGdan => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // SOP-28 MBC2
                D::U2 => part(PartRole::Mapper, mapper::mbc2_sop28()),
                // SOP-8 26A
                D::U3 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgKecn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper::mbc3_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 26A / MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgKfcn | BoardConfig::DmgKfdn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper::mbc3_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgKgdu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper::mbc3_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgLfdn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper::mbc3_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgMcSfcn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MMM01
                D::U2 => part(PartRole::Mapper, mapper::mmm01_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 26A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgMheu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC30
                D::U2 => part(PartRole::Mapper, mapper::mbc30_qfp32()),
                // SOP-32 RAM
                D::U3 => part(PartRole::Ram, sram_sop_32()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgTedn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 HuC-1
                D::U2 => part(PartRole::Mapper, mapper::huc1_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 26A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgTfdn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 HuC-1
                D::U2 => part(PartRole::Mapper, mapper::huc1_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgUedt => match designator {
                // TSOP-I-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_i_32()),
                // QFP-48 HuC-3
                D::U2 => part(PartRole::Mapper, mapper::huc3_qfp48()),
                // TSOP-I-28 RAM
                D::U3 => part(PartRole::Ram, sram_tsop_i_28()),
                // SOP-8 26A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSSOP-14
                D::U5 => part(PartRole::HexInverter, hex_inverter()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgUfdt => match designator {
                // TSOP-I-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_i_32()),
                // QFP-48 HuC-3
                D::U2 => part(PartRole::Mapper, mapper::huc3_qfp48()),
                // TSOP-I-28 RAM
                D::U3 => part(PartRole::Ram, sram_tsop_i_28()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSSOP-14
                D::U5 => part(PartRole::HexInverter, hex_inverter()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgUgdu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-48 HuC-3
                D::U2 => part(PartRole::Mapper, mapper::huc3_qfp48()),
                // TSOP-I-28 RAM
                D::U3 => part(PartRole::Ram, sram_tsop_i_28()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSSOP-14
                D::U5 => part(PartRole::HexInverter, hex_inverter()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgZ02 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom_sop_32()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgZ03 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgZ04 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom_tsop_ii_44()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper::mbc5_qfp32()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, sram_sop_28()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // MOT1 => motor
                _ => None,
            },
        }
    }
    pub fn parts(&self) -> impl Iterator<Item = (PartDesignator, BoardPart)> + '_ {
        PartDesignator::ALL.into_iter().filter_map(|designator| {
            let part = self.part(designator)?;
            Some((designator, part))
        })
    }
}

pub struct BoardPart {
    pub role: PartRole,
    pub parser: Box<dyn Fn(&str) -> Result<Box<dyn ParsedData>, String> + Send + Sync>,
    pub parse_any: Box<dyn Fn(&str) -> Result<Box<dyn Any>, String> + Send + Sync>,
}

fn create_map() -> HashMap<&'static str, BoardConfig> {
    let mut m = HashMap::new();
    m.insert("AGB-ARC", BoardConfig::AgbArc);
    m.insert("AGB-E01", BoardConfig::AgbE01);
    m.insert("AGB-E02", BoardConfig::AgbE02);
    m.insert("AGB-E03", BoardConfig::AgbE03);
    m.insert("AGB-E05", BoardConfig::AgbE05);
    m.insert("AGB-E06", BoardConfig::AgbE06);
    m.insert("AGB-E11", BoardConfig::AgbE11);
    m.insert("AGB-E18", BoardConfig::AgbE18);
    m.insert("AGB-E24", BoardConfig::AgbE24);
    m.insert("AGB-Y11", BoardConfig::AgbY11);
    m.insert("0200309E4-01", BoardConfig::Tama);
    m.insert("AAAC S", BoardConfig::Aaac);
    m.insert("CGB-A32", BoardConfig::CgbA32);
    m.insert("DMG-A02", BoardConfig::DmgA02);
    m.insert("DMG-A03", BoardConfig::DmgA03);
    m.insert("DMG-A04", BoardConfig::DmgA04);
    m.insert("DMG-A06", BoardConfig::DmgA06);
    m.insert("DMG-A07", BoardConfig::DmgA07);
    m.insert("DMG-A08", BoardConfig::DmgA08);
    m.insert("DMG-A09", BoardConfig::DmgA09);
    m.insert("DMG-A11", BoardConfig::DmgA11);
    m.insert("DMG-A14", BoardConfig::DmgA14);
    m.insert("DMG-A15", BoardConfig::DmgA15);
    m.insert("DMG-A16", BoardConfig::DmgA16);
    m.insert("DMG-A40", BoardConfig::DmgA40);
    m.insert("DMG-A47", BoardConfig::DmgA47);
    m.insert("DMG-AAA", BoardConfig::DmgAaa);
    m.insert("DMG-BBA", BoardConfig::DmgBba);
    m.insert("DMG-BCA", BoardConfig::DmgBca);
    m.insert("DMG-BEAN", BoardConfig::DmgBean);
    m.insert("DMG-BEAN(K)", BoardConfig::DmgBean);
    m.insert("DMG-BFAN", BoardConfig::DmgBfan);
    m.insert("DMG-DECN", BoardConfig::DmgDecn);
    m.insert("DMG-DECN(K)", BoardConfig::DmgDecn);
    m.insert("DMG-DEDN", BoardConfig::DmgDedn);
    m.insert("DMG-DGCU", BoardConfig::DmgDgcu);
    m.insert("DMG-GDAN", BoardConfig::DmgGdan);
    m.insert("DMG-KECN", BoardConfig::DmgKecn);
    m.insert("DMG-KFCN", BoardConfig::DmgKfcn);
    m.insert("DMG-KFDN", BoardConfig::DmgKfdn);
    m.insert("DMG-KGDU", BoardConfig::DmgKgdu);
    m.insert("DMG-LFDN", BoardConfig::DmgLfdn);
    m.insert("DMG-M-BFAN", BoardConfig::DmgMBfan);
    m.insert("DMG-MC-DFCN", BoardConfig::DmgMcDfcn);
    m.insert("DMG-MC-SFCN", BoardConfig::DmgMcSfcn);
    m.insert("DMG-MHEU", BoardConfig::DmgMheu);
    m.insert("DMG-TEDN", BoardConfig::DmgTedn);
    m.insert("DMG-TFDN", BoardConfig::DmgTfdn);
    m.insert("DMG-UEDT", BoardConfig::DmgUedt);
    m.insert("DMG-UFDT", BoardConfig::DmgUfdt);
    m.insert("DMG-UGDU", BoardConfig::DmgUgdu);
    m.insert("DMG-Z02", BoardConfig::DmgZ02);
    m.insert("DMG-Z03", BoardConfig::DmgZ03);
    m.insert("DMG-Z04", BoardConfig::DmgZ04);
    m
}

impl BoardConfig {
    pub fn from_label(label: &str) -> Option<BoardConfig> {
        static MAP: OnceLock<HashMap<&'static str, BoardConfig>> = OnceLock::new();
        let map = MAP.get_or_init(create_map);
        label
            .rfind(|c: char| c == '-')
            .map(|pos| label.split_at(pos).0)
            .and_then(|key| map.get(key).cloned())
            .or_else(|| map.get(label).cloned())
    }
}

pub fn load_cfgs<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, GameConfig>, Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let mut cfgs: BTreeMap<String, GameConfig> = serde_json::from_reader(file)?;
    for (rom_id, cfg) in cfgs.iter_mut() {
        cfg.rom_id.clone_from(rom_id);
    }
    Ok(cfgs)
}

pub fn write_cfgs<P: AsRef<Path>>(
    path: P,
    cfgs: &BTreeMap<String, GameConfig>,
) -> Result<(), Error> {
    let file = File::create(path)?;
    let file = BufWriter::new(file);
    serde_json::to_writer_pretty(file, cfgs)?;
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum PartRole {
    Unknown,
    Rom,
    Mapper,
    Ram,
    SupervisorReset,
    Crystal,
    Flash,
    Eeprom,
    Accelerometer,
    LineDecoder,
    HexInverter,
    Mcu,
    Rtc,
}

impl PartRole {
    pub fn display(&self) -> &'static str {
        match self {
            PartRole::Unknown => "Unknown",
            PartRole::Rom => "ROM",
            PartRole::Mapper => "Mapper",
            PartRole::Ram => "RAM",
            PartRole::SupervisorReset => "Supervisor & Reset",
            PartRole::Crystal => "Crystal",
            PartRole::Flash => "Flash",
            PartRole::Eeprom => "EEPROM",
            PartRole::Accelerometer => "Accelerometer",
            PartRole::LineDecoder => "Line decoder",
            PartRole::HexInverter => "Hex inverter",
            PartRole::Mcu => "Microcontroller",
            PartRole::Rtc => "RTC",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartDesignator {
    U1,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    X1,
}

impl PartDesignator {
    const ALL: [PartDesignator; 8] = [
        PartDesignator::U1,
        PartDesignator::U2,
        PartDesignator::U3,
        PartDesignator::U4,
        PartDesignator::U5,
        PartDesignator::U6,
        PartDesignator::U7,
        PartDesignator::X1,
    ];
    pub fn as_str(&self) -> &'static str {
        match self {
            PartDesignator::U1 => "U1",
            PartDesignator::U2 => "U2",
            PartDesignator::U3 => "U3",
            PartDesignator::U4 => "U4",
            PartDesignator::U5 => "U5",
            PartDesignator::U6 => "U6",
            PartDesignator::U7 => "U7",
            PartDesignator::X1 => "X1",
        }
    }
}
