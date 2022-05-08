use anyhow::Error;
use gbhwdb_backend::{
    config::cartridge::*,
    input::cartridge::*,
    parser::{self, LabelParser},
};
use glob::glob;
use site::{build_site, Console};
use std::{
    convert::TryFrom,
    fs::{self, create_dir_all, File},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

use crate::legacy::chip::*;
use crate::legacy::*;

mod legacy;
mod site;
mod template;

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
    let mut site = build_site();
    create_dir_all("build/data")?;
    site.counts.cartridges = process_cartridge_submissions()?;
    site.counts.update(Console::Dmg, process_dmg_submissions()?);
    site.counts.update(Console::Sgb, process_sgb_submissions()?);
    site.counts.update(Console::Mgb, process_mgb_submissions()?);
    site.counts.update(Console::Mgl, process_mgl_submissions()?);
    site.counts
        .update(Console::Sgb2, process_sgb2_submissions()?);
    site.counts.update(Console::Cgb, process_cgb_submissions()?);
    site.counts.update(Console::Agb, process_agb_submissions()?);
    site.counts.update(Console::Ags, process_ags_submissions()?);
    site.counts.update(Console::Gbs, process_gbs_submissions()?);
    site.counts.update(Console::Oxy, process_oxy_submissions()?);
    site.generate_all("build/site")?;
    copy_static_files()?;
    Ok(())
}

fn process_cartridge_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(cartridge.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_dmg_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let cpu = console.mainboard.u1.as_ref().map(|chip| {
                to_legacy_chip(None, chip, parser::gen1_soc::gen1_soc()).unwrap_or_else(|| {
                    LegacyChip {
                        kind: Some("blob".to_string()),
                        ..LegacyChip::default()
                    }
                })
            });
            let year_hint = cpu.as_ref().map(|cpu| cpu.year.unwrap_or(1996));

            let work_ram = console.mainboard.u2.as_ref().map(|chip| {
                to_legacy_chip(year_hint, chip, parser::ram::ram()).unwrap_or_else(|| LegacyChip {
                    kind: Some("blob".to_string()),
                    ..LegacyChip::default()
                })
            });
            let video_ram = console.mainboard.u3.as_ref().map(|chip| {
                to_legacy_chip(year_hint, chip, parser::ram::ram()).unwrap_or_else(|| LegacyChip {
                    kind: Some("blob".to_string()),
                    ..LegacyChip::default()
                })
            });
            let amplifier = console.mainboard.u4.as_ref().map(|chip| {
                to_legacy_chip(year_hint, chip, parser::dmg_amp::dmg_amp()).unwrap_or_else(|| {
                    LegacyChip {
                        kind: Some("blob".to_string()),
                        ..LegacyChip::default()
                    }
                })
            });
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.x1,
                parser::crystal_4mihz::crystal_4mihz(),
            );

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
                let regulator = map_legacy_chip(year_hint, &board.chip, parser::dmg_reg::dmg_reg());
                let lcd_panel = board
                    .screen
                    .as_ref()
                    .and_then(|screen| to_legacy_lcd_panel(year_hint, screen));

                LegacyDmgLcdBoard {
                    kind: board.label.clone(),
                    circled_letters: board.circled_letters.clone(),
                    stamp: board.stamp.clone(),
                    year: board.year,
                    month: board.month,
                    lcd_panel,
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
                    gbhwdb_backend::parser::dmg_stamp::dmg_stamp()
                        .parse(&stamp)
                        .unwrap_or_else(|_| panic!("{}", stamp))
                });
            let lcd_board_stamp = console
                .lcd_board
                .as_ref()
                .and_then(|board| board.stamp.as_ref().filter(|_| !board.outlier))
                .map(|stamp| {
                    gbhwdb_backend::parser::dmg_stamp::dmg_stamp()
                        .parse(&stamp)
                        .unwrap_or_else(|_| panic!("{}", stamp))
                });
            let stamp = mainboard_stamp.or(lcd_board_stamp);

            let metadata = LegacyDmgMetadata {
                color: console.shell.color.map(|c| format!("{:?}", c)),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                month: stamp.as_ref().and_then(|stamp| stamp.month),
                mainboard,
                lcd_board,
                power_board,
                jack_board,
            };

            let has_outliers = console.shell.outlier
                || console.mainboard.outlier
                || console
                    .lcd_board
                    .as_ref()
                    .map(|board| {
                        board.outlier
                            || board
                                .screen
                                .as_ref()
                                .map(|screen| screen.outlier)
                                .unwrap_or(false)
                    })
                    .unwrap_or(false)
                || console
                    .power_board
                    .as_ref()
                    .map(|board| board.outlier)
                    .unwrap_or(false)
                || console
                    .jack_board
                    .as_ref()
                    .map(|board| board.outlier)
                    .unwrap_or(false);
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
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
                slug: console.slug,
                sort_group: Some(
                    (match (console.shell.serial.is_some(), has_outliers) {
                        (true, false) => "A",
                        (false, false) => "B",
                        (true, true) => "C",
                        (false, true) => "D",
                    })
                    .to_owned(),
                ),
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/dmg.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(u32::try_from(submissions.len())?)
}

fn process_sgb_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::gen1_soc::gen1_soc(),
            );
            let icd2 = map_legacy_chip(year_hint, &console.mainboard.u2, parser::icd2::icd2());
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u3, parser::ram::ram());
            let video_ram = map_legacy_chip(year_hint, &console.mainboard.u4, parser::ram::ram());
            let rom = map_legacy_chip(year_hint, &console.mainboard.u5, parser::sgb_rom::sgb_rom());
            let cic = map_legacy_chip(year_hint, &console.mainboard.u6, parser::cic::cic());
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_mgb_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::gen2_soc::gen2_soc(),
            );
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::ram::ram());
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::mgb_amp::mgb_amp());
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u4, parser::dmg_reg::dmg_reg());
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.x1,
                parser::crystal_4mihz::crystal_4mihz(),
            );
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
            let lcd_panel = to_legacy_lcd_panel(year_hint, &console.screen);

            let stamp = console.mainboard.stamp.as_ref().map(|stamp| {
                gbhwdb_backend::parser::dmg_stamp::dmg_stamp()
                    .parse(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyMgbMetadata {
                color: console.shell.color.map(|c| format!("{:?}", c)),
                release_code: console.shell.release_code.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                month: stamp.as_ref().and_then(|stamp| stamp.month),
                mainboard,
                lcd_panel,
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
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_mgl_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::gen2_soc::gen2_soc(),
            );
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::ram::ram());
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::mgb_amp::mgb_amp());
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u4, parser::dmg_reg::dmg_reg());
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.x1,
                parser::crystal_4mihz::crystal_4mihz(),
            );
            let t1 = map_legacy_chip(
                year_hint,
                &console.mainboard.t1,
                parser::mgl_transformer::mgl_transformer(),
            );
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
            let lcd_panel = to_legacy_lcd_panel(year_hint, &console.screen);

            let stamp = console.mainboard.stamp.as_ref().map(|stamp| {
                gbhwdb_backend::parser::cgb_stamp::cgb_stamp()
                    .parse(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyMglMetadata {
                color: console.shell.color.map(|c| format!("{:?}", c)),
                release_code: console.shell.release_code.clone(),
                year: stamp
                    .as_ref()
                    .and_then(|stamp| to_legacy_year(year_hint, stamp.year)),
                week: stamp.as_ref().and_then(|stamp| stamp.week),
                mainboard,
                lcd_panel,
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
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_sgb2_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );

            let year_hint = console.mainboard.year;
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::gen2_soc::gen2_soc(),
            );
            let icd2 = map_legacy_chip(year_hint, &console.mainboard.u2, parser::icd2::icd2());
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u3, parser::ram::ram());
            let rom = map_legacy_chip(year_hint, &console.mainboard.u4, parser::sgb_rom::sgb_rom());
            let cic = map_legacy_chip(year_hint, &console.mainboard.u5, parser::cic::cic());
            let coil = map_legacy_chip(year_hint, &console.mainboard.coil1, parser::coil::coil());
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.xtal1,
                parser::crystal_20mihz::crystal_20mihz(),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_cgb_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let year_hint = console.mainboard.year.or(Some(1998));
            let cpu = map_legacy_chip(year_hint, &console.mainboard.u1, parser::cgb_soc::cgb_soc());
            let work_ram = map_legacy_chip(year_hint, &console.mainboard.u2, parser::ram::ram());
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::mgb_amp::mgb_amp());
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u4, parser::cgb_reg::cgb_reg());
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.x1,
                parser::crystal_8mihz::crystal_8mihz(),
            );
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
                                gbhwdb_backend::parser::dmg_stamp::dmg_stamp()
                                    .parse(&stamp)
                                    .unwrap_or_else(|_| panic!("{}", stamp)),
                            ),
                            None,
                        )
                    } else {
                        (
                            None,
                            Some(
                                gbhwdb_backend::parser::cgb_stamp::cgb_stamp()
                                    .parse(&stamp)
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
                color: console.shell.color.map(|c| format!("{:?}", c)),
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
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_agb_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let year_hint = console.mainboard.year.or(Some(2001));
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::agb_soc_qfp_128::agb_soc_qfp_128(),
            );
            let work_ram = map_legacy_chip(
                year_hint,
                &console.mainboard.u2,
                parser::sram_tsop1_48::sram_tsop1_48(),
            );
            let regulator =
                map_legacy_chip(year_hint, &console.mainboard.u3, parser::agb_reg::agb_reg());
            let u4 = map_legacy_chip(
                year_hint,
                &console.mainboard.u4,
                parser::agb_pmic::agb_pmic(),
            );
            let amplifier =
                map_legacy_chip(year_hint, &console.mainboard.u6, parser::agb_amp::agb_amp());
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.x1,
                parser::crystal_4mihz::crystal_4mihz(),
            );
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
                gbhwdb_backend::parser::cgb_stamp::cgb_stamp()
                    .parse(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyAgbMetadata {
                color: console.shell.color.map(|c| format!("{:?}", c)),
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
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_ags_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let year_hint = console.mainboard.year.or(Some(2003));
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::agb_soc_qfp_156::agb_soc_qfp_156(),
            );
            let work_ram = map_legacy_chip(
                year_hint,
                &console.mainboard.u2,
                parser::sram_tsop1_48::sram_tsop1_48(),
            );
            let amplifier = match console.mainboard.label.as_str() {
                // FIXME: Not really an amplifier
                "C/AGS-CPU-30" | "C/AGT-CPU-01" => map_legacy_chip(
                    year_hint,
                    &console.mainboard.u3,
                    parser::ags_pmic_new::ags_pmic_new(),
                ),
                _ => map_legacy_chip(year_hint, &console.mainboard.u3, parser::agb_amp::agb_amp()),
            };
            let u4 = map_legacy_chip(
                year_hint,
                &console.mainboard.u4,
                parser::ags_pmic_old::ags_pmic_old(),
            );
            let u5 = map_legacy_chip(
                year_hint,
                &console.mainboard.u5,
                parser::ags_charge_ctrl::ags_charge_ctrl(),
            );
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.x1,
                parser::crystal_4mihz::crystal_4mihz(),
            );
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
                color: console.shell.color.map(|c| format!("{:?}", c)),
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
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_gbs_submissions() -> Result<u32, Error> {
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
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );

            let year_hint = console.mainboard.year.or(Some(2003));
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u2,
                parser::agb_soc_qfp_128::agb_soc_qfp_128(),
            );
            let work_ram = map_legacy_chip(
                year_hint,
                &console.mainboard.u3,
                parser::sram_tsop1_48::sram_tsop1_48(),
            );
            let u4 = map_legacy_chip(year_hint, &console.mainboard.u4, parser::gbs_dol::gbs_dol());
            let u5 = map_legacy_chip(year_hint, &console.mainboard.u5, parser::gbs_reg::gbs_reg());
            let u6 = map_legacy_chip(year_hint, &console.mainboard.u6, parser::gbs_reg::gbs_reg());
            let crystal = map_legacy_chip(
                year_hint,
                &console.mainboard.y1,
                parser::crystal_32mihz::crystal_32mihz(),
            );
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
                gbhwdb_backend::parser::cgb_stamp::cgb_stamp()
                    .parse(&stamp)
                    .unwrap_or_else(|_| panic!("{}", stamp))
            });

            let metadata = LegacyGbsMetadata {
                color: console.shell.color.map(|c| format!("{:?}", c)),
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
    Ok(u32::try_from(submissions.len())?)
}

fn process_oxy_submissions() -> Result<u32, Error> {
    use gbhwdb_backend::input::oxy::*;
    use legacy::console::*;
    let walker = WalkDir::new("data/consoles/OXY").min_depth(2).max_depth(2);
    let mut submissions = Vec::new();
    for entry in walker.into_iter().filter_entry(is_metadata_file) {
        let entry = entry?;
        if let Some(root) = entry.path().parent() {
            println!("{}", entry.path().display());
            let file = File::open(&entry.path())?;
            let console: OxyConsole = serde_json::from_reader(file)?;
            assert_eq!(
                Some(console.slug.as_str()),
                root.file_name().and_then(|name| name.to_str())
            );
            if let Some(serial) = &console.shell.serial {
                assert_eq!(&console.slug, serial);
            }

            let year_hint = console.mainboard.year.or(Some(2005));
            let cpu = map_legacy_chip(
                year_hint,
                &console.mainboard.u1,
                parser::agb_soc_bga::agb_soc_bga(),
            );
            let u2 = map_legacy_chip(
                year_hint,
                &console.mainboard.u2,
                parser::oxy_pmic::oxy_pmic(),
            );
            let u4 = map_legacy_chip(year_hint, &console.mainboard.u4, parser::oxy_u4::oxy_u4());
            let u5 = map_legacy_chip(year_hint, &console.mainboard.u5, parser::oxy_u5::oxy_u5());
            let mainboard = LegacyOxyMainboard {
                kind: console.mainboard.label.clone(),
                circled_letters: console.mainboard.circled_letters.clone(),
                year: console.mainboard.year,
                month: console.mainboard.month,
                cpu,
                u2,
                u4,
                u5,
            };

            let metadata = LegacyOxyMetadata {
                color: console.shell.color.map(|c| format!("{:?}", c)),
                release_code: console.shell.release_code.clone(),
                mainboard,
            };

            let mut photos = LegacyPhotos::default();
            photos.front = get_photo(root, "01_front.jpg");
            photos.back = get_photo(root, "02_back.jpg");
            photos.pcb_front = get_photo(root, "03_pcb_front.jpg");
            photos.pcb_back = get_photo(root, "04_pcb_back.jpg");
            submissions.push(LegacySubmission {
                code: "oxy".to_string(),
                title: console
                    .shell
                    .serial
                    .clone()
                    .unwrap_or_else(|| format!("Unit #{}", console.index.unwrap())),
                slug: console.slug,
                sort_group: None,
                contributor: console.contributor,
                metadata,
                photos,
            });
        }
    }
    submissions.sort_by_key(|submission| (submission.slug.clone()));
    let file = File::create("build/data/oxy.json")?;
    serde_json::to_writer_pretty(file, &submissions)?;
    Ok(u32::try_from(submissions.len())?)
}

fn copy_static_files() -> Result<(), Error> {
    static PATTERNS: [&str; 8] = [
        "static/**/*.html",
        "static/**/*.txt",
        "static/**/*.ico",
        "static/**/*.jpg",
        "static/**/*.png",
        "static/**/*.svg",
        "static/**/*.webmanifest",
        "static/**/*.xml",
    ];
    let target = Path::new("build/site");
    for pattern in &PATTERNS {
        for entry in glob(pattern)? {
            let path = entry?;
            let target = target.join(path.strip_prefix("static")?);
            if let Some(parent) = target.parent() {
                create_dir_all(parent)?;
            }
            fs::copy(&path, target)?;
        }
    }
    Ok(())
}
