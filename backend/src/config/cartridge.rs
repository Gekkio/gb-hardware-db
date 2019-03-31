use failure::Error;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Index;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameConfig {
    pub name: String,
    pub platform: GamePlatform,
    pub layout: BoardLayout,
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
}

lazy_static! {
    static ref MAP: HashMap<&'static str, BoardLayout> = create_map();
}

fn create_map() -> HashMap<&'static str, BoardLayout> {
    let mut m = HashMap::new();
    m.insert("0200309E4-01", BoardLayout::Tama);
    m.insert("AAAC", BoardLayout::Rom);
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
        label
            .rfind(|c: char| c == '-')
            .map(|pos| label.split_at(pos).0)
            .and_then(|key| MAP.get(key).cloned())
            .or_else(|| MAP.get(label).cloned())
    }
}

pub fn load_cfgs<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, GameConfig>, Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let cfgs = serde_json::from_reader(file)?;
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
pub enum ChipPosition {
    U1,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    X1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChipRole {
    Unknown,
    Rom,
    Mapper,
    Ram,
    RamBackup,
    Crystal,
    Flash,
    Eeprom,
    Accelerometer,
    LineDecoder,
    Tama,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct ChipRoleConfig {
    pub u1: Option<ChipRole>,
    pub u2: Option<ChipRole>,
    pub u3: Option<ChipRole>,
    pub u4: Option<ChipRole>,
    pub u5: Option<ChipRole>,
    pub u6: Option<ChipRole>,
    pub u7: Option<ChipRole>,
    pub x1: Option<ChipRole>,
}

impl Index<ChipPosition> for ChipRoleConfig {
    type Output = Option<ChipRole>;
    fn index(&self, pos: ChipPosition) -> &Option<ChipRole> {
        match pos {
            ChipPosition::U1 => &self.u1,
            ChipPosition::U2 => &self.u2,
            ChipPosition::U3 => &self.u3,
            ChipPosition::U4 => &self.u4,
            ChipPosition::U5 => &self.u5,
            ChipPosition::U6 => &self.u6,
            ChipPosition::U7 => &self.u7,
            ChipPosition::X1 => &self.x1,
        }
    }
}

impl ChipRoleConfig {
    pub fn from_layout(layout: BoardLayout) -> ChipRoleConfig {
        match layout {
            BoardLayout::Rom => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                ..ChipRoleConfig::default()
            },
            BoardLayout::RomMapper => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                ..ChipRoleConfig::default()
            },
            BoardLayout::RomMapperRam => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                u3: Some(ChipRole::Ram),
                u4: Some(ChipRole::RamBackup),
                ..ChipRoleConfig::default()
            },
            BoardLayout::RomMapperRamXtal => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                u3: Some(ChipRole::Ram),
                u4: Some(ChipRole::RamBackup),
                x1: Some(ChipRole::Crystal),
                ..ChipRoleConfig::default()
            },
            BoardLayout::Mbc2 => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                u3: Some(ChipRole::RamBackup),
                ..ChipRoleConfig::default()
            },
            BoardLayout::Mbc6 => ChipRoleConfig {
                u1: Some(ChipRole::Mapper),
                u2: Some(ChipRole::Rom),
                u3: Some(ChipRole::Flash),
                u4: Some(ChipRole::Ram),
                u5: Some(ChipRole::RamBackup),
                ..ChipRoleConfig::default()
            },
            BoardLayout::Mbc7 => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                u3: Some(ChipRole::Eeprom),
                u4: Some(ChipRole::Accelerometer),
                ..ChipRoleConfig::default()
            },
            BoardLayout::Type15 => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                u3: Some(ChipRole::Ram),
                u4: Some(ChipRole::RamBackup),
                u5: Some(ChipRole::Rom),
                u6: Some(ChipRole::LineDecoder),
                ..ChipRoleConfig::default()
            },
            BoardLayout::Huc3 => ChipRoleConfig {
                u1: Some(ChipRole::Rom),
                u2: Some(ChipRole::Mapper),
                u3: Some(ChipRole::Ram),
                u4: Some(ChipRole::RamBackup),
                u5: Some(ChipRole::Unknown),
                x1: Some(ChipRole::Crystal),
                ..ChipRoleConfig::default()
            },
            BoardLayout::Tama => ChipRoleConfig {
                u1: Some(ChipRole::Tama),
                u2: Some(ChipRole::Tama),
                u3: Some(ChipRole::Tama),
                u4: Some(ChipRole::Unknown),
                u5: Some(ChipRole::RamBackup),
                x1: Some(ChipRole::Crystal),
                ..ChipRoleConfig::default()
            },
        }
    }
}
