// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{
    config::cartridge::*,
    input::{cartridge::*, Chip},
    parser::*,
    time::Month,
};
use serde::Serialize;
use std::ops::{Index, IndexMut};

use super::{to_legacy_manufacturer, to_legacy_year, DateCode, HasDateCode, LegacyChip};

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMetadata {
    #[serde(skip)]
    pub cfg: GameConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    pub board: LegacyBoard,
    pub dump: Option<CartridgeDump>,
}

impl super::LegacyMetadata for LegacyMetadata {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyBoard {
    #[serde(skip)]
    pub layout: BoardLayout,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u1: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u2: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u3: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u6: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u7: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1: Option<LegacyChip>,
}

impl Index<PartDesignator> for LegacyBoard {
    type Output = Option<LegacyChip>;

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

impl IndexMut<PartDesignator> for LegacyBoard {
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

impl HasDateCode for LegacyBoard {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
    }
}

pub fn add_legacy_chips(layout: BoardLayout, board: CartridgeBoard, legacy: &mut LegacyBoard) {
    let roles = ChipRoleConfig::from(layout);
    for (designator, role) in roles.iter() {
        legacy[designator] =
            to_legacy_chip(layout, board.year, Some(role), board[designator].as_ref());
    }
}

pub fn to_legacy_chip(
    layout: BoardLayout,
    board_year: Option<u16>,
    role: Option<ChipRole>,
    chip: Option<&Chip>,
) -> Option<LegacyChip> {
    if role == None {
        assert_eq!(chip, None);
    }
    chip.map(|chip| {
        let mut legacy = LegacyChip {
            kind: None,
            label: chip.label.clone(),
            manufacturer: None,
            year: None,
            month: None,
            week: None,
            rom_code: None,
        };
        if let Some(label) = chip.label.as_ref() {
            if let Some(role) = role {
                match role {
                    ChipRole::Rom => {
                        if layout == BoardLayout::Tama {
                            let chip = gbhwdb_backend::parser::tama::tama()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = Some("TAMA7".to_owned());
                            legacy.year = to_legacy_year(board_year, chip.year);
                            legacy.week = chip.week;
                        } else {
                            let chip = gbhwdb_backend::parser::mask_rom::mask_rom()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = chip.chip_type;
                            legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                            legacy.year = to_legacy_year(board_year, chip.year);
                            legacy.week = chip.week;
                        }
                    }
                    ChipRole::Mapper => {
                        if layout == BoardLayout::Tama {
                            let chip = gbhwdb_backend::parser::tama::tama()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = Some("TAMA5".to_owned());
                            legacy.year = to_legacy_year(board_year, chip.year);
                            legacy.week = chip.week;
                        } else {
                            let chip = gbhwdb_backend::parser::mapper::mapper()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = Some(chip.mbc_type.display_name().to_owned());
                            legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                            legacy.year = to_legacy_year(board_year, chip.year);
                            legacy.week = chip.week;
                        }
                    }
                    ChipRole::Ram => {
                        let chip = gbhwdb_backend::parser::ram::ram()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::SupervisorReset => {
                        let chip = gbhwdb_backend::parser::supervisor_reset::supervisor_reset()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.chip_type);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Crystal => {
                        let chip = gbhwdb_backend::parser::crystal_32kihz::crystal_32kihz()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.format_frequency());
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.month = chip.month;
                    }
                    ChipRole::Flash => {
                        let chip = gbhwdb_backend::parser::flash::flash()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Eeprom => {
                        let chip = gbhwdb_backend::parser::eeprom::eeprom()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Accelerometer => {
                        let chip = gbhwdb_backend::parser::accelerometer::accelerometer()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::LineDecoder => {
                        let chip = gbhwdb_backend::parser::line_decoder::line_decoder()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                    }
                    ChipRole::HexInverter => {
                        let chip = gbhwdb_backend::parser::hex_inverter::hex_inverter()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Mcu => {
                        let chip = gbhwdb_backend::parser::tama::tama()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some("TAMA6".to_owned());
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Rtc => {
                        let chip = gbhwdb_backend::parser::rtc::rtc()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(chip.kind);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Unknown => (),
                }
            }
        }
        legacy
    })
}
