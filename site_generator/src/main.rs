use failure::Error;
use gbhwdb_backend::config::cartridge::*;
use gbhwdb_backend::input::cartridge::*;
use gbhwdb_backend::parser::*;
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::u32;
use walkdir::{DirEntry, WalkDir};

use legacy::*;

mod legacy;

fn is_metadata_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.file_name() == "metadata.json"
}

fn to_legacy_manufacturer(manufacturer: Option<Manufacturer>) -> Option<String> {
    manufacturer.map(|manufacturer| {
        (match manufacturer {
            Manufacturer::Analog => "analog",
            Manufacturer::AtT => "at_t",
            Manufacturer::Bsi => "bsi",
            Manufacturer::Fujitsu => "fujitsu",
            Manufacturer::Hudson => "hudson",
            Manufacturer::Hyundai => "hyundai",
            Manufacturer::Kds => "kds",
            Manufacturer::Lgs => "lgs",
            Manufacturer::LsiLogic => "lsi-logic",
            Manufacturer::Macronix => "macronix",
            Manufacturer::Mitsubishi => "mitsubishi",
            Manufacturer::Mitsumi => "mitsumi",
            Manufacturer::MoselVitelic => "mosel-vitelic",
            Manufacturer::Motorola => "motorola",
            Manufacturer::Nec => "nec",
            Manufacturer::Oki => "oki",
            Manufacturer::Rohm => "rohm",
            Manufacturer::Samsung => "samsung",
            Manufacturer::Sanyo => "sanyo",
            Manufacturer::Sharp => "sharp",
            Manufacturer::Smsc => "smsc",
            Manufacturer::TexasInstruments => "texas-instruments",
            Manufacturer::Toshiba => "toshiba",
            Manufacturer::Winbond => "winbond",
        })
        .to_owned()
    })
}

fn to_legacy_mapper_type(mapper: MapperType) -> Option<String> {
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

fn to_legacy_year(board_year: Option<u32>, chip_year: Option<Year>) -> Option<u16> {
    match (board_year, chip_year) {
        (_, Some(Year::Full(year))) => Some(year),
        (Some(board_year), Some(Year::Partial(year))) => {
            let diff_90 = (board_year as i32 - 1990 - year as i32).abs();
            let diff_00 = (board_year as i32 - 2000 - year as i32).abs();
            let year = if diff_90 < diff_00 {
                1990 + year as u16
            } else {
                2000 + year as u16
            };
            assert!(year >= 1989 && year < 2010);
            Some(year)
        }
        _ => None,
    }
}

fn to_legacy_rom(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_mask_rom(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = chip.chip_type;
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_mapper(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_mapper(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = to_legacy_mapper_type(chip.mbc_type);
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_ram(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_ram(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = chip.chip_type;
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_ram_backup(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip = gbhwdb_backend::parser::parse_ram_backup(&label)
            .unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = Some(chip.chip_type);
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_tama(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_tama(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = Some(
            (match chip.tama_type {
                TamaType::Tama5 => "TAMA5",
                TamaType::Tama6 => "TAMA6",
                TamaType::Tama7 => "TAMA7",
            })
            .to_owned(),
        );
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_accelerometer(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip = gbhwdb_backend::parser::parse_accelerometer(&label)
            .unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = chip.chip_type;
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_flash(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_flash(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = chip.chip_type;
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_eeprom(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_eeprom(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = chip.chip_type;
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.week = chip.week.map(|week| week as u16);
    })
}

fn to_legacy_crystal(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip =
            gbhwdb_backend::parser::parse_crystal(&label).unwrap_or_else(|_| panic!("{}", label));
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
        legacy.month = chip.month.map(|month| month as u16);
    })
}

fn to_legacy_line_decoder(board_year: Option<u32>, chip: Option<Chip>) -> Option<LegacyChip> {
    to_legacy_chip(chip, |label, legacy| {
        let chip = gbhwdb_backend::parser::parse_line_decoder(&label)
            .unwrap_or_else(|_| panic!("{}", label));
        legacy.kind = chip.chip_type;
        legacy.manufacturer = to_legacy_manufacturer(chip.manufacturer);
        legacy.year = to_legacy_year(board_year, chip.year);
    })
}

fn to_legacy_chip<F: FnOnce(String, &mut LegacyChip)>(
    chip: Option<Chip>,
    f: F,
) -> Option<LegacyChip> {
    chip.map(|chip| {
        let mut legacy = LegacyChip {
            kind: None,
            label: chip.label.clone(),
            manufacturer: None,
            year: None,
            month: None,
            week: None,
        };
        if let Some(label) = chip.label {
            f(label, &mut legacy);
        }
        legacy
    })
}

fn add_legacy_chips(layout: BoardLayout, board: CartridgeBoard, legacy: &mut LegacyBoard) {
    match layout {
        BoardLayout::Rom => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
        }
        BoardLayout::RomMapper => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
        }
        BoardLayout::RomMapperRam => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
            legacy.ram = to_legacy_ram(board.year, board.u3);
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u4);
        }
        BoardLayout::RomMapperRamXtal => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
            legacy.ram = to_legacy_ram(board.year, board.u3);
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u4);
            legacy.crystal = to_legacy_crystal(board.year, board.x1);
        }
        BoardLayout::Mbc2 => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u3);
        }
        BoardLayout::Mbc6 => {
            legacy.mapper = to_legacy_mapper(board.year, board.u1);
            legacy.rom = to_legacy_rom(board.year, board.u2);
            legacy.flash = to_legacy_flash(board.year, board.u3);
            legacy.ram = to_legacy_ram(board.year, board.u4);
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u5);
        }
        BoardLayout::Mbc7 => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
            legacy.eeprom = to_legacy_eeprom(board.year, board.u3);
            legacy.accelerometer = to_legacy_accelerometer(board.year, board.u4);
        }
        BoardLayout::Type15 => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
            legacy.ram = to_legacy_ram(board.year, board.u3);
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u4);
            legacy.rom2 = to_legacy_rom(board.year, board.u5);
            legacy.line_decoder = to_legacy_line_decoder(board.year, board.u6);
        }
        BoardLayout::Huc3 => {
            legacy.rom = to_legacy_rom(board.year, board.u1);
            legacy.mapper = to_legacy_mapper(board.year, board.u2);
            legacy.ram = to_legacy_ram(board.year, board.u3);
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u4);
            legacy.u5 = to_legacy_chip(board.u5, |_, _| ());
            legacy.crystal = to_legacy_crystal(board.year, board.x1);
        }
        BoardLayout::Tama => {
            legacy.rom = to_legacy_tama(board.year, board.u1);
            legacy.mapper = to_legacy_tama(board.year, board.u2);
            legacy.ram = to_legacy_tama(board.year, board.u3);
            legacy.u4 = to_legacy_chip(board.u4, |_, _| ());
            legacy.ram_protector = to_legacy_ram_backup(board.year, board.u5);
            legacy.crystal = to_legacy_crystal(board.year, board.x1);
        }
    }
}

fn main() -> Result<(), Error> {
    let cfgs = gbhwdb_backend::config::cartridge::load_cfgs("config/games.json")?;
    let walker = WalkDir::new("data/cartridges").min_depth(3).max_depth(3);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let cartridge: Cartridge = serde_json::from_reader(file)?;
            let cfg = cfgs.get(&cartridge.code).unwrap();

            let layout = BoardLayout::from_label(&cartridge.board.label).unwrap_or_else(|| {
                panic!(
                    "Failed to find board layout for board {}",
                    cartridge.board.label
                )
            });
            assert_eq!(layout, cfg.layout);

            if let Some(year) = cartridge.board.year {
                assert!(year >= 1989 && year < 2010);
            }
            if let Some(month) = cartridge.board.month {
                assert!(month >= 1 && month < 13);
            }

            let mut board = LegacyBoard {
                kind: cartridge.board.label.clone(),
                circled_letters: cartridge.board.circled_letters.clone(),
                extra_label: cartridge.board.extra_label.clone(),
                year: cartridge.board.year.map(|year| year as u16),
                month: cartridge.board.month.map(|month| month as u16),
                rom: None,
                rom2: None,
                mapper: None,
                ram: None,
                ram_protector: None,
                flash: None,
                u4: None,
                u5: None,
                line_decoder: None,
                eeprom: None,
                accelerometer: None,
                crystal: None,
                battery: None,
            };
            add_legacy_chips(cfg.layout, cartridge.board, &mut board);
            let metadata = LegacyMetadata {
                code: cartridge.shell.code,
                stamp: cartridge.shell.stamp,
                board,
            };
            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.pcb_front = get_photo(root, "02_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "03_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: cartridge.code,
                title: format!("Entry #{}", cartridge.index),
                slug: cartridge.slug,
                contributor: cartridge.contributor,
                metadata,
                photos,
            });
        }
    }
    create_dir_all("build/data")?;
    submissions.sort_by_key(|submission| (submission.code.clone(), submission.slug.clone()));
    let file = File::create("build/data/cartridges.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn get_photo(root: &Path, name: &str) -> Option<LegacyPhoto> {
    if root.join(name).exists() {
        Some(LegacyPhoto {
            path: root
                .canonicalize()
                .unwrap()
                .join(name)
                .display()
                .to_string(),
            name: name.to_owned(),
        })
    } else {
        None
    }
}
