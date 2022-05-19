use gbhwdb_backend::{
    config::cartridge::*,
    input::{cartridge::*, Chip},
    parser::*,
    time::Month,
};
use serde::{Deserialize, Serialize};

use super::{to_legacy_manufacturer, to_legacy_year, DateCode, HasDateCode, LegacyChip};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    pub board: LegacyBoard,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyBoard {
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
    pub rom: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom2: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapper: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_protector: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_decoder: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eeprom: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerometer: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
    #[serde(skip_serializing)]
    pub battery: Option<String>,
}

impl HasDateCode for LegacyBoard {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            week: None,
        }
    }
}

pub fn to_legacy_mapper_type(mapper: MapperType) -> Option<String> {
    Some(
        (match mapper {
            MapperType::Mbc1(Mbc1Version::Original) => "MBC1",
            MapperType::Mbc1(Mbc1Version::A) => "MBC1A",
            MapperType::Mbc1(Mbc1Version::B) => "MBC1B",
            MapperType::Mbc1(Mbc1Version::B1) => "MBC1B1",
            MapperType::Mbc2(Mbc2Version::Original) => "MBC2",
            MapperType::Mbc2(Mbc2Version::A) => "MBC2A",
            MapperType::Mbc3(Mbc3Version::Original) => "MBC3",
            MapperType::Mbc3(Mbc3Version::A) => "MBC3A",
            MapperType::Mbc3(Mbc3Version::B) => "MBC3B",
            MapperType::Mbc30 => "MBC30",
            MapperType::Mbc5 => "MBC5",
            MapperType::Mbc6 => "MBC6",
            MapperType::Mbc7 => "MBC7",
            MapperType::Mmm01 => "MMM01",
            MapperType::Huc3 => "HuC-3",
            MapperType::Huc1(Huc1Version::Original) => "HuC-1",
            MapperType::Huc1(Huc1Version::A) => "HuC-1A",
        })
        .to_owned(),
    )
}

pub fn add_legacy_chips(layout: BoardLayout, board: CartridgeBoard, legacy: &mut LegacyBoard) {
    let roles = ChipRoleConfig::from_layout(layout);
    match layout {
        BoardLayout::Rom => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
        }
        BoardLayout::RomMapper => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
        }
        BoardLayout::RomMapperRam => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.ram = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u4, board.u4);
        }
        BoardLayout::RomMapperRamXtal => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.ram = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u4, board.u4);
            legacy.crystal = to_legacy_chip(board.year, roles.x1, board.x1);
        }
        BoardLayout::Mbc2 => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u3, board.u3);
        }
        BoardLayout::Mbc6 => {
            legacy.mapper = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.rom = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.flash = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.ram = to_legacy_chip(board.year, roles.u4, board.u4);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u5, board.u5);
        }
        BoardLayout::Mbc7 => {
            legacy.mapper = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.rom = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.eeprom = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.accelerometer = to_legacy_chip(board.year, roles.u4, board.u4);
        }
        BoardLayout::Type15 => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.ram = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u4, board.u4);
            legacy.rom2 = to_legacy_chip(board.year, roles.u5, board.u5);
            legacy.line_decoder = to_legacy_chip(board.year, roles.u6, board.u6);
        }
        BoardLayout::Huc3 => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.ram = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u4, board.u4);
            legacy.u5 = to_legacy_chip(board.year, roles.u5, board.u5);
            legacy.crystal = to_legacy_chip(board.year, roles.x1, board.x1);
        }
        BoardLayout::Tama => {
            legacy.rom = to_legacy_chip(board.year, roles.u1, board.u1);
            legacy.mapper = to_legacy_chip(board.year, roles.u2, board.u2);
            legacy.ram = to_legacy_chip(board.year, roles.u3, board.u3);
            legacy.u4 = to_legacy_chip(board.year, roles.u4, board.u4);
            legacy.ram_protector = to_legacy_chip(board.year, roles.u5, board.u5);
            legacy.crystal = to_legacy_chip(board.year, roles.x1, board.x1);
        }
    }
}

pub fn to_legacy_chip(
    board_year: Option<u16>,
    role: Option<ChipRole>,
    chip: Option<Chip>,
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
        if let Some(label) = chip.label {
            if let Some(role) = role {
                match role {
                    ChipRole::Rom => {
                        let chip = gbhwdb_backend::parser::mask_rom::mask_rom()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = chip.chip_type;
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
                    }
                    ChipRole::Mapper => {
                        let chip = gbhwdb_backend::parser::mapper::mapper()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = to_legacy_mapper_type(chip.mbc_type);
                        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
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
                    ChipRole::RamBackup => {
                        let chip = gbhwdb_backend::parser::ram_backup::ram_backup()
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
                    ChipRole::Tama => {
                        let chip = gbhwdb_backend::parser::tama::tama()
                            .parse(&label)
                            .unwrap_or_else(|_| panic!("{}", label));
                        legacy.kind = Some(
                            (match chip.tama_type {
                                TamaType::Tama5 => "TAMA5",
                                TamaType::Tama6 => "TAMA6",
                                TamaType::Tama7 => "TAMA7",
                            })
                            .to_owned(),
                        );
                        legacy.year = to_legacy_year(board_year, chip.year);
                        legacy.week = chip.week;
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
                    _ => (),
                }
            }
        }
        legacy
    })
}
