use failure::Error;
use gbhwdb_backend::config::cartridge::*;
use gbhwdb_backend::input::cartridge::*;
use gbhwdb_backend::parser;
use std::fs::{create_dir_all, File};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use legacy::chip::*;
use legacy::*;

mod legacy;

fn is_metadata_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && entry.file_name() == "metadata.json"
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

fn main() -> Result<(), Error> {
    create_dir_all("build/data")?;
    process_cartridge_submissions()?;
    process_dmg_submissions()?;
    process_sgb_submissions()?;
    process_mgb_submissions()?;
    process_mgl_submissions()?;
    process_sgb2_submissions()?;
    process_cgb_submissions()?;
    process_agb_submissions()?;
    process_ags_submissions()?;
    process_gbs_submissions()?;
    Ok(())
}

fn process_cartridge_submissions() -> Result<(), Error> {
    use legacy::cartridge::*;
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
            assert!(cfg.layouts.contains(&layout));

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
            add_legacy_chips(layout, cartridge.board, &mut board);
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
                sort_group: None,
                contributor: cartridge.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.code.clone(), submission.slug.clone()));
    let file = File::create("build/data/cartridges.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_dmg_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::dmg::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/DMG").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: DmgConsole = serde_json::from_reader(file)?;

            let cpu = console.mainboard.u1.as_ref().map(|chip| {
                to_legacy_chip(None, chip, parser::parse_gen1_cpu).unwrap_or_else(|| LegacyChip {
                    kind: Some("blob".to_string()),
                    ..LegacyChip::default()
                })
            });
            let year_hint = cpu.as_ref().map(|cpu| cpu.year.unwrap_or(1996));

            let work_ram = console.mainboard.u2.as_ref().map(|chip| {
                to_legacy_chip(year_hint, chip, parser::parse_ram).unwrap_or_else(|| LegacyChip {
                    kind: Some("blob".to_string()),
                    ..LegacyChip::default()
                })
            });
            let video_ram = console.mainboard.u3.as_ref().map(|chip| {
                to_legacy_chip(year_hint, chip, parser::parse_ram).unwrap_or_else(|| LegacyChip {
                    kind: Some("blob".to_string()),
                    ..LegacyChip::default()
                })
            });
            let amplifier = console.mainboard.u4.as_ref().map(|chip| {
                to_legacy_chip(year_hint, chip, parser::parse_dmg_amp).unwrap_or_else(|| {
                    LegacyChip {
                        kind: Some("blob".to_string()),
                        ..LegacyChip::default()
                    }
                })
            });
            let crystal = map_legacy_chip(year_hint, &console.mainboard.x1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("4.194304 MHz".to_owned()),
                    ..chip
                });

            let mainboard = LegacyDmgMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                extra_label: console.mainboard.extra_label.clone(),
                stamp: console.mainboard.stamp.clone(),
                cpu,
                work_ram,
                video_ram,
                amplifier,
                crystal,
            };

            let lcd_board = console.lcd_board.as_ref().map(|board| {
                let regulator = map_legacy_chip(year_hint, &board.chip, parser::parse_dmg_reg);
                let column_driver = board
                    .screen
                    .as_ref()
                    .and_then(|screen| screen.column_driver.as_ref())
                    .map(|chip| to_legacy_lcd_chip(year_hint, chip));

                let row_driver = board
                    .screen
                    .as_ref()
                    .and_then(|screen| screen.row_driver.as_ref())
                    .map(|chip| to_legacy_lcd_chip(year_hint, chip));

                LegacyDmgLcdBoard {
                    kind: board.label.clone(),
                    circled_letters: board.circled_letters.clone(),
                    stamp: board.stamp.clone(),
                    year: board.year,
                    month: board.month,
                    lcd_panel: board.screen.as_ref().and_then(to_legacy_dmg_lcd_panel),
                    column_driver,
                    row_driver,
                    regulator,
                }
            });

            let power_board = console
                .power_board
                .as_ref()
                .map(|board| LegacyDmgPowerBoard {
                    kind: board.kind.clone(),
                    label: (if board.kind == "D" {
                        "DC CONV2 DMG"
                    } else {
                        "DC CONV DMG"
                    })
                    .to_owned(),
                    year: board.year,
                    month: board.month,
                });

            let jack_board = console.jack_board.as_ref().map(|board| LegacyDmgJackBoard {
                kind: board.kind.clone(),
                extra_label: board.extra_label.clone(),
            });

            let mainboard_stamp = console
                .mainboard
                .stamp
                .as_ref()
                .filter(|_| !console.mainboard.outlier)
                .map(|stamp| {
                    gbhwdb_backend::parser::parse_dmg_stamp(&stamp)
                        .unwrap_or_else(|_| panic!("{}", stamp))
                });
            let lcd_board_stamp = console
                .lcd_board
                .as_ref()
                .and_then(|board| board.stamp.as_ref().filter(|_| !board.outlier))
                .map(|stamp| {
                    gbhwdb_backend::parser::parse_dmg_stamp(&stamp)
                        .unwrap_or_else(|_| panic!("{}", stamp))
                });
            let stamp = mainboard_stamp.or(lcd_board_stamp);

            let metadata = LegacyDmgMetadata {
                kind: "DMG".to_string(),
                color: console.shell.color.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                month: stamp.as_ref().and_then(|stamp| stamp.month),
                mainboard,
                lcd_board,
                power_board,
                jack_board,
            };

            let mut photos = LegacyDmgPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.mainboard_front = get_photo(root, "03_mainboard_front.jpg");
            photos.mainboard_back = get_photo(root, "04_mainboard_back.jpg");
            photos.lcd_board_front = get_photo(root, "05_lcd_board_front.jpg");
            photos.lcd_board_back = get_photo(root, "06_lcd_board_back.jpg");
            photos.power_board_front = get_photo(root, "07_power_board_front.jpg");
            photos.power_board_back = get_photo(root, "08_power_board_back.jpg");
            photos.jack_board_front = get_photo(root, "09_jack_board_front.jpg");
            photos.jack_board_back = get_photo(root, "10_jack_board_back.jpg");
            submissions.push(LegacySubmission {
                code: "dmg".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index)),
                slug: console.slug,
                sort_group: console.shell.serial,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/dmg.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_sgb_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::sgb::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/SGB").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: SgbConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_gen1_cpu);
            let icd2 = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_icd2);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_ram);
            let video_ram = map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_ram);
            let rom = map_legacy_chip(year_hint, &console.mainboard.u5, parser::parse_sgb_rom);
            let cic = map_legacy_chip(year_hint, &console.mainboard.u6, parser::parse_cic);
            let mainboard = LegacySgbMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                letter_at_top_right: console.mainboard.letter_at_top_right.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                icd2,
                work_ram,
                video_ram,
                rom,
                cic,
            };

            let metadata = LegacySgbMetadata {
                kind: "SGB".to_string(),
                stamp: console.shell.stamp,
                mainboard,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "sgb".to_string(),
                title: format!("Unit #{}", console.index),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/sgb.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_mgb_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::mgb::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/MGB").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: MgbConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_gen2_cpu);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_ram);
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_mgb_amp);
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_dmg_reg);
            let crystal = map_legacy_chip(year_hint, &console.mainboard.x1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("4.194304 MHz".to_owned()),
                    ..chip
                });
            let mainboard = LegacyMgbMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                number_pair: console.mainboard.number_pair.clone(),
                stamp: console.mainboard.stamp.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                work_ram,
                amplifier,
                regulator,
                crystal,
            };
            let lcd = to_legacy_lcd_panel(year_hint, &console.screen);

            let stamp = console.mainboard.stamp.as_ref().map(|stamp| {
                gbhwdb_backend::parser::parse_dmg_stamp(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyMgbMetadata {
                kind: "MGB".to_string(),
                color: console.shell.color.clone(),
                release_code: console.shell.release_code.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                month: stamp.as_ref().and_then(|stamp| stamp.month),
                mainboard,
                lcd,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "mgb".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index)),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/mgb.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_mgl_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::mgl::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/MGL").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: MglConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_gen2_cpu);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_ram);
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_mgb_amp);
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_dmg_reg);
            let crystal = map_legacy_chip(year_hint, &console.mainboard.x1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("4.194304 MHz".to_owned()),
                    ..chip
                });
            let t1 = map_legacy_chip(year_hint, &console.mainboard.t1, parser::parse_transformer);
            let mainboard = LegacyMglMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                number_pair: console.mainboard.number_pair.clone(),
                stamp: console.mainboard.stamp.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                work_ram,
                amplifier,
                regulator,
                crystal,
                t1,
            };
            let lcd = to_legacy_lcd_panel(year_hint, &console.screen);

            let stamp = console.mainboard.stamp.as_ref().map(|stamp| {
                gbhwdb_backend::parser::parse_cgb_stamp(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyMglMetadata {
                kind: "MGL".to_string(),
                color: console.shell.color.clone(),
                release_code: console.shell.release_code.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                week: stamp.as_ref().and_then(|stamp| stamp.week),
                mainboard,
                lcd,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "mgl".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index)),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/mgl.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_sgb2_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::sgb2::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/SGB2").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: Sgb2Console = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_gen2_cpu);
            let icd2 = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_icd2);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_ram);
            let rom = map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_sgb_rom);
            let cic = map_legacy_chip(year_hint, &console.mainboard.u5, parser::parse_cic);
            let coil = map_legacy_chip(year_hint, &console.mainboard.coil1, parser::parse_coil);
            let crystal =
                map_legacy_chip(year_hint, &console.mainboard.xtal1, parser::parse_crystal).map(
                    |chip| LegacyChip {
                        kind: Some("20.971520 MHz".to_owned()),
                        ..chip
                    },
                );
            let mainboard = LegacySgb2Mainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                letter_at_top_right: console.mainboard.letter_at_top_right.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                icd2,
                work_ram,
                rom,
                cic,
                coil,
                crystal,
            };

            let metadata = LegacySgb2Metadata {
                kind: "SGB2".to_string(),
                stamp: console.shell.stamp,
                mainboard,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "sgb2".to_string(),
                title: format!("Unit #{}", console.index),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/sgb2.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_cgb_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::cgb::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/CGB").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: CgbConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year.or(Some(1998));
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_cgb_cpu);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_ram);
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_mgb_amp);
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_cgb_reg);
            let crystal = map_legacy_chip(year_hint, &console.mainboard.x1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("8.388608 MHz".to_owned()),
                    ..chip
                });
            let mainboard = LegacyCgbMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                number_pair: console.mainboard.number_pair.clone(),
                stamp: console.mainboard.stamp.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                work_ram,
                amplifier,
                regulator,
                crystal,
            };

            let (old_stamp, new_stamp) = match &console.mainboard.stamp {
                Some(stamp) => {
                    if stamp.starts_with(&['6', '7', '8', '9'][..]) {
                        (
                            Some(
                                gbhwdb_backend::parser::parse_dmg_stamp(&stamp)
                                    .unwrap_or_else(|_| panic!("{}", stamp)),
                            ),
                            None,
                        )
                    } else {
                        (
                            None,
                            Some(
                                gbhwdb_backend::parser::parse_cgb_stamp(&stamp)
                                    .unwrap_or_else(|_| panic!("{}", stamp)),
                            ),
                        )
                    }
                }
                None => (None, None),
            };
            let stamp_year = new_stamp
                .as_ref()
                .and_then(|stamp| stamp.year)
                .or(old_stamp.as_ref().and_then(|stamp| stamp.year));

            let metadata = LegacyCgbMetadata {
                kind: "CGB".to_string(),
                color: console.shell.color.clone(),
                release_code: console.shell.release_code.clone(),
                year: to_legacy_year(year_hint, stamp_year),
                month: old_stamp.as_ref().and_then(|stamp| stamp.month),
                week: new_stamp.as_ref().and_then(|stamp| stamp.week),
                mainboard,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "cgb".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index)),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/cgb.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_agb_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::agb::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/AGB").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: AgbConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year.or(Some(2001));
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_agb_cpu);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_agb_ram);
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_agb_reg);
            let u4 = map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_agb_u4);
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u6, parser::parse_agb_amp);
            let crystal = map_legacy_chip(year_hint, &console.mainboard.x1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("4.194304 MHz".to_owned()),
                    ..chip
                });
            let mainboard = LegacyAgbMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                number_pair: console.mainboard.number_pair.clone(),
                stamp: console.mainboard.stamp.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                work_ram,
                amplifier,
                regulator,
                crystal,
                u4,
            };

            let stamp = console.mainboard.stamp.as_ref().map(|stamp| {
                gbhwdb_backend::parser::parse_cgb_stamp(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyAgbMetadata {
                kind: "AGB".to_string(),
                color: console.shell.color.clone(),
                release_code: console.shell.release_code.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                week: stamp.as_ref().and_then(|stamp| stamp.week),
                mainboard,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "agb".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index)),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/agb.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_ags_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::ags::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/AGS").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: AgsConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year.or(Some(2003));
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::parse_agb_cpu);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_agb_ram);
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_agb_amp);
            let u4 = map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_ags_u4);
            let u5 = map_legacy_chip(year_hint, &console.mainboard.u5, parser::parse_ags_u5);
            let crystal = map_legacy_chip(year_hint, &console.mainboard.x1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("4.194304 MHz".to_owned()),
                    ..chip
                });
            let mainboard = LegacyAgsMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                number_pair: console.mainboard.number_pair.clone(),
                stamp: console.mainboard.stamp.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                work_ram,
                amplifier,
                u4,
                u5,
                crystal,
            };

            let metadata = LegacyAgsMetadata {
                kind: "AGS".to_string(),
                color: console.shell.color.clone(),
                release_code: console.shell.release_code.clone(),
                mainboard,
            };

            let mut photos = LegacyAgsPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.top = get_photo(root, "02_top.jpg");
            photos.back = get_photo(root, "03_back.jpg");
            photos.pcb_front = get_photo(root, "04_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "05_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "ags".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index)),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/ags.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}

fn process_gbs_submissions() -> Result<(), Error> {
    use gbhwdb_backend::input::gbs::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/GBS").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: GbsConsole = serde_json::from_reader(file)?;

            let year_hint = console.mainboard.year.or(Some(2003));
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u2, parser::parse_agb_cpu);
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u3, parser::parse_agb_ram);
            let u4 = map_legacy_chip(year_hint, &console.mainboard.u4, parser::parse_gbs_dol);
            let u5 = map_legacy_chip(year_hint, &console.mainboard.u5, parser::parse_gbs_reg);
            let u6 = map_legacy_chip(year_hint, &console.mainboard.u6, parser::parse_gbs_reg);
            let crystal = map_legacy_chip(year_hint, &console.mainboard.y1, parser::parse_crystal)
                .map(|chip| LegacyChip {
                    kind: Some("33.554432 MHz".to_owned()),
                    ..chip
                });
            let mainboard = LegacyGbsMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                number_pair: console.mainboard.number_pair.clone(),
                stamp: console.mainboard.stamp.clone(),
                stamp_front: console.mainboard.stamp_front.clone(),
                stamp_back: console.mainboard.stamp_back.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                work_ram,
                crystal,
                u4,
                u5,
                u6,
            };

            let stamp = console.mainboard.stamp.as_ref().map(|stamp| {
                gbhwdb_backend::parser::parse_cgb_stamp(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyGbsMetadata {
                kind: "GBS".to_string(),
                color: console.shell.color.clone(),
                release_code: console.shell.release_code.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                week: stamp.as_ref().and_then(|stamp| stamp.week),
                mainboard,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "gbs".to_string(),
                title: format!("Unit #{}", console.index),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/gbs.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(())
}
