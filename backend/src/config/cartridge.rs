// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    fs::File,
    io::{BufReader, BufWriter},
    ops::{Index, IndexMut},
    path::Path,
    sync::OnceLock,
};

use crate::sha256::Sha256;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameConfig {
    #[serde(skip, default)]
    pub rom_id: String,
    pub name: String,
    pub rom_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<Sha256>,
    pub platform: GamePlatform,
    pub layouts: Vec<BoardLayout>,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum BoardLayout {
    #[serde(rename = "rom")]
    Rom,
    #[serde(rename = "rom_mapper")]
    RomMapper,
    #[serde(rename = "rom_mapper_ram")]
    RomMapperRam,
    #[serde(rename = "rom_mapper_ram_xtal")]
    RomMapperRamXtal,
    #[serde(rename = "mbc2")]
    Mbc2,
    #[serde(rename = "mbc6")]
    Mbc6,
    #[serde(rename = "mbc7")]
    Mbc7,
    #[serde(rename = "type_15")]
    Type15,
    #[serde(rename = "huc3")]
    Huc3,
    #[serde(rename = "tama")]
    Tama,
    #[serde(rename = "agb_e06")]
    AgbE06,
}

fn create_map() -> HashMap<&'static str, BoardLayout> {
    let mut m = HashMap::new();
    m.insert("AGB-E06", BoardLayout::AgbE06);
    m.insert("0200309E4-01", BoardLayout::Tama);
    m.insert("AAAC S", BoardLayout::Rom);
    m.insert("CGB-A32", BoardLayout::Mbc6);
    m.insert("DMG-A02", BoardLayout::RomMapperRam);
    m.insert("DMG-A03", BoardLayout::RomMapperRam);
    m.insert("DMG-A04", BoardLayout::RomMapperRam);
    m.insert("DMG-A06", BoardLayout::RomMapperRam);
    m.insert("DMG-A07", BoardLayout::RomMapper);
    m.insert("DMG-A08", BoardLayout::RomMapperRam);
    m.insert("DMG-A09", BoardLayout::RomMapper);
    m.insert("DMG-A10", BoardLayout::RomMapper);
    m.insert("DMG-A11", BoardLayout::RomMapperRam);
    m.insert("DMG-A12", BoardLayout::RomMapperRam);
    m.insert("DMG-A13", BoardLayout::RomMapper);
    m.insert("DMG-A14", BoardLayout::RomMapperRam);
    m.insert("DMG-A15", BoardLayout::Type15);
    m.insert("DMG-A16", BoardLayout::RomMapperRam);
    m.insert("DMG-A18", BoardLayout::RomMapper);
    m.insert("DMG-A40", BoardLayout::Mbc7);
    m.insert("DMG-A47", BoardLayout::Mbc7);
    m.insert("DMG-AAA", BoardLayout::Rom);
    m.insert("DMG-BBA", BoardLayout::RomMapper);
    m.insert("DMG-BCA", BoardLayout::RomMapper);
    m.insert("DMG-BEAN", BoardLayout::RomMapper);
    m.insert("DMG-BEAN(K)", BoardLayout::RomMapper);
    m.insert("DMG-BFAN", BoardLayout::RomMapper);
    m.insert("DMG-DECN", BoardLayout::RomMapperRam);
    m.insert("DMG-DECN(K)", BoardLayout::RomMapperRam);
    m.insert("DMG-DEDN", BoardLayout::RomMapperRam);
    m.insert("DMG-DFCN", BoardLayout::RomMapperRam);
    m.insert("DMG-DGCU", BoardLayout::RomMapperRam);
    m.insert("DMG-GDAN", BoardLayout::Mbc2);
    m.insert("DMG-KECN", BoardLayout::RomMapperRamXtal);
    m.insert("DMG-KFCN", BoardLayout::RomMapperRamXtal);
    m.insert("DMG-KFDN", BoardLayout::RomMapperRamXtal);
    m.insert("DMG-KGDU", BoardLayout::RomMapperRamXtal);
    m.insert("DMG-LFDN", BoardLayout::RomMapperRam);
    m.insert("DMG-M-BFAN", BoardLayout::RomMapper);
    m.insert("DMG-MC-DFCN", BoardLayout::RomMapperRam);
    m.insert("DMG-MC-SFCN", BoardLayout::RomMapperRam);
    m.insert("DMG-MHEU", BoardLayout::RomMapperRamXtal);
    m.insert("DMG-TEDN", BoardLayout::RomMapperRam);
    m.insert("DMG-TFDN", BoardLayout::RomMapperRam);
    m.insert("DMG-UEDT", BoardLayout::Huc3);
    m.insert("DMG-UFDT", BoardLayout::Huc3);
    m.insert("DMG-UGDU", BoardLayout::Huc3);
    m.insert("DMG-Z01", BoardLayout::RomMapperRam);
    m.insert("DMG-Z02", BoardLayout::RomMapperRam);
    m.insert("DMG-Z03", BoardLayout::RomMapperRam);
    m.insert("DMG-Z04", BoardLayout::RomMapperRam);
    m
}

impl BoardLayout {
    pub fn from_label(label: &str) -> Option<BoardLayout> {
        static MAP: OnceLock<HashMap<&'static str, BoardLayout>> = OnceLock::new();
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct PartRoleConfig {
    pub u1: Option<PartRole>,
    pub u2: Option<PartRole>,
    pub u3: Option<PartRole>,
    pub u4: Option<PartRole>,
    pub u5: Option<PartRole>,
    pub u6: Option<PartRole>,
    pub u7: Option<PartRole>,
    pub x1: Option<PartRole>,
}

impl Index<PartDesignator> for PartRoleConfig {
    type Output = Option<PartRole>;

    fn index(&self, index: PartDesignator) -> &Self::Output {
        match index {
            PartDesignator::U1 => &self.u1,
            PartDesignator::U2 => &self.u2,
            PartDesignator::U3 => &self.u3,
            PartDesignator::U4 => &self.u4,
            PartDesignator::U5 => &self.u5,
            PartDesignator::U6 => &self.u6,
            PartDesignator::U7 => &self.u7,
            PartDesignator::X1 => &self.x1,
        }
    }
}

impl IndexMut<PartDesignator> for PartRoleConfig {
    fn index_mut(&mut self, index: PartDesignator) -> &mut Self::Output {
        match index {
            PartDesignator::U1 => &mut self.u1,
            PartDesignator::U2 => &mut self.u2,
            PartDesignator::U3 => &mut self.u3,
            PartDesignator::U4 => &mut self.u4,
            PartDesignator::U5 => &mut self.u5,
            PartDesignator::U6 => &mut self.u6,
            PartDesignator::U7 => &mut self.u7,
            PartDesignator::X1 => &mut self.x1,
        }
    }
}

impl IntoIterator for PartRoleConfig {
    type Item = (PartDesignator, PartRole);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            PartDesignator::ALL
                .into_iter()
                .filter_map(move |d| self[d].map(|role| (d, role))),
        )
    }
}

impl<'a> IntoIterator for &'a PartRoleConfig {
    type Item = (PartDesignator, PartRole);
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            PartDesignator::ALL
                .into_iter()
                .filter_map(move |d| self[d].map(|role| (d, role))),
        )
    }
}

impl From<BoardLayout> for PartRoleConfig {
    fn from(layout: BoardLayout) -> Self {
        match layout {
            BoardLayout::Rom => PartRoleConfig {
                u1: Some(PartRole::Rom),
                ..PartRoleConfig::default()
            },
            BoardLayout::RomMapper => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                ..PartRoleConfig::default()
            },
            BoardLayout::RomMapperRam => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                u3: Some(PartRole::Ram),
                u4: Some(PartRole::SupervisorReset),
                ..PartRoleConfig::default()
            },
            BoardLayout::RomMapperRamXtal => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                u3: Some(PartRole::Ram),
                u4: Some(PartRole::SupervisorReset),
                x1: Some(PartRole::Crystal),
                ..PartRoleConfig::default()
            },
            BoardLayout::Mbc2 => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                u3: Some(PartRole::SupervisorReset),
                ..PartRoleConfig::default()
            },
            BoardLayout::Mbc6 => PartRoleConfig {
                u1: Some(PartRole::Mapper),
                u2: Some(PartRole::Rom),
                u3: Some(PartRole::Flash),
                u4: Some(PartRole::Ram),
                u5: Some(PartRole::SupervisorReset),
                ..PartRoleConfig::default()
            },
            BoardLayout::Mbc7 => PartRoleConfig {
                u1: Some(PartRole::Mapper),
                u2: Some(PartRole::Rom),
                u3: Some(PartRole::Eeprom),
                u4: Some(PartRole::Accelerometer),
                ..PartRoleConfig::default()
            },
            BoardLayout::Type15 => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                u3: Some(PartRole::Ram),
                u4: Some(PartRole::SupervisorReset),
                u5: Some(PartRole::Rom),
                u6: Some(PartRole::LineDecoder),
                ..PartRoleConfig::default()
            },
            BoardLayout::Huc3 => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                u3: Some(PartRole::Ram),
                u4: Some(PartRole::SupervisorReset),
                u5: Some(PartRole::HexInverter),
                x1: Some(PartRole::Crystal),
                ..PartRoleConfig::default()
            },
            BoardLayout::Tama => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Mapper),
                u3: Some(PartRole::Mcu),
                u4: Some(PartRole::Rtc),
                u5: Some(PartRole::SupervisorReset),
                x1: Some(PartRole::Crystal),
                ..PartRoleConfig::default()
            },
            BoardLayout::AgbE06 => PartRoleConfig {
                u1: Some(PartRole::Rom),
                u2: Some(PartRole::Ram),
                u3: Some(PartRole::SupervisorReset),
                ..PartRoleConfig::default()
            },
        }
    }
}
