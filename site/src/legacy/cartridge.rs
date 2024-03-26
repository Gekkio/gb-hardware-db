// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{
    config::cartridge::*,
    input::{cartridge::*, Part},
    parser::*,
    time::Month,
};
use std::ops::{Index, IndexMut};

use super::{to_legacy_manufacturer, to_legacy_year, DateCode, HasDateCode, LegacyPart};

#[derive(Clone, Debug)]
pub struct LegacyMetadata {
    pub cfg: GameConfig,
    pub code: Option<String>,
    pub stamp: Option<String>,
    pub board: LegacyBoard,
    pub dump: Option<CartridgeDump>,
}

impl super::LegacyMetadata for LegacyMetadata {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyBoard {
    pub layout: BoardLayout,
    pub kind: String,
    pub circled_letters: Option<String>,
    pub extra_label: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub u1: Option<LegacyPart>,
    pub u2: Option<LegacyPart>,
    pub u3: Option<LegacyPart>,
    pub u4: Option<LegacyPart>,
    pub u5: Option<LegacyPart>,
    pub u6: Option<LegacyPart>,
    pub u7: Option<LegacyPart>,
    pub x1: Option<LegacyPart>,
}

impl Index<PartDesignator> for LegacyBoard {
    type Output = Option<LegacyPart>;

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

pub fn add_legacy_parts(layout: BoardLayout, board: CartridgeBoard, legacy: &mut LegacyBoard) {
    let roles = PartRoleConfig::from(layout);
    for (designator, role) in roles.into_iter() {
        legacy[designator] =
            to_legacy_part(layout, board.year, Some(role), board[designator].as_ref());
    }
}

pub fn to_legacy_part(
    layout: BoardLayout,
    board_year: Option<u16>,
    role: Option<PartRole>,
    part: Option<&Part>,
) -> Option<LegacyPart> {
    if role == None {
        assert_eq!(part, None);
    }
    part.map(|part| {
        let mut legacy = LegacyPart {
            kind: None,
            label: part.label.clone(),
            manufacturer: None,
            year: None,
            month: None,
            week: None,
            rom_code: None,
        };
        if let Some(label) = part.label.as_ref() {
            if let Some(role) = role {
                match role {
                    PartRole::Rom => {
                        if layout == BoardLayout::Tama {
                            let part = gbhwdb_backend::parser::tama::tama()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = Some("TAMA7".to_owned());
                            legacy.year = to_legacy_year(board_year, part.year);
                            legacy.week = part.week;
                        } else {
                            let part = gbhwdb_backend::parser::mask_rom::mask_rom()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = part.chip_type;
                            legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                            legacy.year = to_legacy_year(board_year, part.year);
                            legacy.week = part.week;
                        }
                    }
                    PartRole::Mapper => {
                        if layout == BoardLayout::Tama {
                            let part = gbhwdb_backend::parser::tama::tama()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = Some("TAMA5".to_owned());
                            legacy.year = to_legacy_year(board_year, part.year);
                            legacy.week = part.week;
                        } else {
                            let part = gbhwdb_backend::parser::mapper::mapper()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("{}", label));
                            legacy.kind = Some(part.mbc_type.display_name().to_owned());
                            legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                            legacy.year = to_legacy_year(board_year, part.year);
                            legacy.week = part.week;
                        }
                    }
                    PartRole::Ram => {
                        let part = gbhwdb_backend::parser::ram::ram()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::SupervisorReset => {
                        let part = gbhwdb_backend::parser::supervisor_reset::supervisor_reset()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.chip_type);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::Crystal => {
                        let part = gbhwdb_backend::parser::crystal_32kihz::crystal_32kihz()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.format_frequency());
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.month = part.month;
                    }
                    PartRole::Flash => {
                        let part = gbhwdb_backend::parser::flash::flash()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::Eeprom => {
                        let part = gbhwdb_backend::parser::eeprom::eeprom()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::Accelerometer => {
                        let part = gbhwdb_backend::parser::accelerometer::accelerometer()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::LineDecoder => {
                        let part = gbhwdb_backend::parser::line_decoder::line_decoder()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                    }
                    PartRole::HexInverter => {
                        let part = gbhwdb_backend::parser::hex_inverter::hex_inverter()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::Mcu => {
                        let part = gbhwdb_backend::parser::tama::tama()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some("TAMA6".to_owned());
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::Rtc => {
                        let part = gbhwdb_backend::parser::rtc::rtc()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(part.kind);
                        legacy.manufacturer = to_legacy_manufacturer(part.manufacturer);
                        legacy.year = to_legacy_year(board_year, part.year);
                        legacy.week = part.week;
                    }
                    PartRole::Unknown => (),
                }
            }
        }
        legacy
    })
}
