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
    parser::{
        accelerometer::accelerometer, crystal_32kihz::crystal_32kihz, eeprom::eeprom, flash::flash,
        hex_inverter::hex_inverter, line_decoder::line_decoder, mapper::mapper, mask_rom::mask_rom,
        ram::ram, rtc::rtc, supervisor_reset::supervisor_reset, tama::tama, LabelParser,
    },
    sha256::Sha256,
};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameConfig {
    #[serde(skip, default)]
    pub rom_id: String,
    pub name: String,
    pub rom_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<Sha256>,
    pub platform: GamePlatform,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum GamePlatform {
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "gbc")]
    Gbc,
    #[serde(rename = "gba")]
    Gba,
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
    AgbE06,
    AgbE11,
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

        fn part<T: 'static>(
            role: PartRole,
            parser: &'static impl LabelParser<T>,
        ) -> Option<BoardPart> {
            Some(BoardPart {
                role,
                parser: Box::new(move |input| {
                    let value = parser.parse(input)?;
                    Ok(Box::new(value))
                }),
            })
        }

        match self {
            BoardConfig::AgbE06 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // SOP-28 RAM
                D::U2 => part(PartRole::Ram, ram()),
                // SOP-8 BU9803F
                D::U3 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::AgbE11 | BoardConfig::AgbY11 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // SOP-28 FRAM
                D::U2 => part(PartRole::Ram, ram()),
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
                D::U4 => part(PartRole::Rtc, rtc()),
                // SOP-8 M62021P
                D::U5 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::Aaac => match designator {
                // glop top ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                _ => None,
            },
            BoardConfig::CgbA32 => match designator {
                // TQFP-64 MBC6
                D::U1 => part(PartRole::Mapper, mapper()),
                // SOP-32 ROM
                D::U2 => part(PartRole::Rom, mask_rom()),
                // TSOP-I-40 Flash
                D::U3 => part(PartRole::Flash, flash()),
                // SOP-28 RAM
                D::U4 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U5 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA02 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA03 | BoardConfig::DmgA08 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA04 => match designator {
                // TSOP-I-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // MOT1 => motor
                _ => None,
            },
            BoardConfig::DmgA06 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA07 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                _ => None,
            },
            BoardConfig::DmgA09 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                _ => None,
            },
            BoardConfig::DmgA11 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // MOT1 => motor
                _ => None,
            },
            BoardConfig::DmgA14 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-32 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA15 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSOP-II-44 ROM
                D::U5 => part(PartRole::Rom, mask_rom()),
                // SSOP-8
                D::U6 => part(PartRole::LineDecoder, line_decoder()),
                _ => None,
            },
            BoardConfig::DmgA16 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-32 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgA40 => match designator {
                // QFP-56 MBC7
                D::U1 => part(PartRole::Mapper, mapper()),
                // SOP-32 ROM
                D::U2 => part(PartRole::Rom, mask_rom()),
                // TSSOP-8 EEPROM
                D::U3 => part(PartRole::Eeprom, eeprom()),
                // QC-14 accelerometer
                D::U4 => part(PartRole::Accelerometer, accelerometer()),
                _ => None,
            },
            BoardConfig::DmgA47 => match designator {
                // QFP-56 MBC7
                D::U1 => part(PartRole::Mapper, mapper()),
                // TSOP-II-44 ROM
                D::U2 => part(PartRole::Rom, mask_rom()),
                // TSSOP-8 EEPROM
                D::U3 => part(PartRole::Eeprom, eeprom()),
                // QC-14 accelerometer
                D::U4 => part(PartRole::Accelerometer, accelerometer()),
                _ => None,
            },
            BoardConfig::DmgAaa => match designator {
                // QFP-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                _ => None,
            },
            BoardConfig::DmgBba | BoardConfig::DmgBca => match designator {
                // QFP-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // SOP-24 MBC1
                D::U2 => part(PartRole::Mapper, mapper()),
                _ => None,
            },
            BoardConfig::DmgBean | BoardConfig::DmgBfan | BoardConfig::DmgMBfan => match designator
            {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // SOP-24 MBC1
                D::U2 => part(PartRole::Mapper, mapper()),
                _ => None,
            },
            BoardConfig::DmgDecn | BoardConfig::DmgDedn | BoardConfig::DmgMcDfcn => {
                match designator {
                    // SOP-32 ROM
                    D::U1 => part(PartRole::Rom, mask_rom()),
                    // SOP-24 MBC1
                    D::U2 => part(PartRole::Mapper, mapper()),
                    // SOP-28 RAM
                    D::U3 => part(PartRole::Ram, ram()),
                    // SOP-8 26A
                    D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                    _ => None,
                }
            }
            BoardConfig::DmgDgcu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // SOP-24 MBC1
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgGdan => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // SOP-28 MBC2
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-8 26A
                D::U3 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgKecn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 26A / MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgKfcn | BoardConfig::DmgKfdn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgKgdu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgLfdn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC3
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgMcSfcn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MMM01
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 26A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgMheu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC30
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-32 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgTedn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 HuC-1
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 26A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgTfdn => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 HuC-1
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgUedt => match designator {
                // TSOP-I-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-48 HuC-3
                D::U2 => part(PartRole::Mapper, mapper()),
                // TSOP-I-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 26A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSSOP-14
                D::U5 => part(PartRole::HexInverter, hex_inverter()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgUfdt => match designator {
                // TSOP-I-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-48 HuC-3
                D::U2 => part(PartRole::Mapper, mapper()),
                // TSOP-I-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSSOP-14
                D::U5 => part(PartRole::HexInverter, hex_inverter()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgUgdu => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-48 HuC-3
                D::U2 => part(PartRole::Mapper, mapper()),
                // TSOP-I-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                // TSSOP-14
                D::U5 => part(PartRole::HexInverter, hex_inverter()),
                D::X1 => part(PartRole::Crystal, crystal_32kihz()),
                _ => None,
            },
            BoardConfig::DmgZ02 => match designator {
                // SOP-32 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgZ03 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
                // SOP-8 MM1134A
                D::U4 => part(PartRole::SupervisorReset, supervisor_reset()),
                _ => None,
            },
            BoardConfig::DmgZ04 => match designator {
                // TSOP-II-44 ROM
                D::U1 => part(PartRole::Rom, mask_rom()),
                // QFP-32 MBC5
                D::U2 => part(PartRole::Mapper, mapper()),
                // SOP-28 RAM
                D::U3 => part(PartRole::Ram, ram()),
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
    pub parser: Box<dyn Fn(&str) -> Result<Box<dyn Any>, String>>,
}

fn create_map() -> HashMap<&'static str, BoardConfig> {
    let mut m = HashMap::new();
    m.insert("AGB-E06", BoardConfig::AgbE06);
    m.insert("AGB-E11", BoardConfig::AgbE11);
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
        let map = MAP.get_or_init(|| create_map());
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
        cfg.rom_id = rom_id.clone();
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
