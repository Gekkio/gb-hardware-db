use serde::{Deserialize, Serialize};
use std::fmt;

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
}

impl fmt::Display for GamePlatform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GamePlatform::Gb => write!(f, "GB"),
            GamePlatform::Gbc => write!(f, "GBC"),
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

impl BoardLayout {
    pub fn from_board_type(board_type: &str) -> Option<BoardLayout> {
        match board_type {
            "AAAC" | "DMG-AAA" => Some(BoardLayout::Rom),
            "DMG-A07" | "DMG-A09" | "DMG-BBA" | "DMG-BEAN" | "DMG-BFAN" | "DMG-M-BFAN" => {
                Some(BoardLayout::RomMapper)
            }
            "DMG-GDAN" => Some(BoardLayout::Mbc2),
            "DMG-A02" | "DMG-A03" | "DMG-A04" | "DMG-A06" | "DMG-A08" | "DMG-A11" | "DMG-A14"
            | "DMG-DECN" | "DMG-DEDN" | "DMG-DGCU" | "DMG-MC-DFCN" | "DMG-MC-SFCN" | "DMG-LFDN"
            | "DMG-TEDN" | "DMG-TFDN" | "DMG-Z02" | "DMG-Z04" => Some(BoardLayout::RomMapperRam),
            "DMG-MHEU" | "DMG-KECN" | "DMG-KFCN" | "DMG-KFDN" | "DMG-KGDU" => {
                Some(BoardLayout::RomMapperRamXtal)
            }
            "DMG-UEDT" | "DMG-UFDT" | "DMG-UGDU" => Some(BoardLayout::Huc3),
            "DMG-A40" | "DMG-A47" => Some(BoardLayout::Mbc7),
            "DMG-A15" => Some(BoardLayout::Type15),
            "CGB-A32" => Some(BoardLayout::Mbc6),
            "0200309E4" => Some(BoardLayout::Tama),
            _ => None,
        }
    }
}
