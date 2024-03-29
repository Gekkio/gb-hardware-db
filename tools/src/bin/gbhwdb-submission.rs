// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::Error;
use cursive::{traits::*, views::*, Cursive, CursiveExt};
use gbhwdb_backend::{
    config::cartridge::{BoardLayout, GameConfig, PartRole, PartRoleConfig},
    input::{
        cartridge::{Cartridge, CartridgeBoard, CartridgeShell},
        Part,
    },
    parser::{self, LabelParser},
    time::Month,
};
use gbhwdb_tools::cursive::*;
use slug::slugify;
use std::{
    collections::BTreeMap,
    fmt,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::atomic::{self, AtomicBool},
};

static QUIT: AtomicBool = AtomicBool::new(false);

fn should_quit() -> bool {
    QUIT.load(atomic::Ordering::SeqCst)
}

fn load_cfgs<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, GameConfig>, Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let cfgs = serde_json::from_reader(file)?;
    Ok(cfgs)
}

fn main() -> Result<(), Error> {
    let cfgs = load_cfgs("config/games.json")?;
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| {
        QUIT.store(true, atomic::Ordering::SeqCst);
        s.quit();
    });
    let contributor = ask_contributor_name(&mut siv);
    if should_quit() || contributor.is_empty() {
        Ok(())
    } else {
        while let Some((root, cartridge)) = ask_submission(&mut siv, &cfgs, &contributor) {
            let json = serde_json::to_string_pretty(&cartridge)?;
            siv.add_layer(
                Dialog::new()
                    .title(root.display().to_string())
                    .content(TextView::new(json.clone()))
                    .button("Ok", |s| s.quit()),
            );
            siv.run();
            siv.pop_layer();
            if should_quit() {
                break;
            }
            create_dir_all(&root)?;
            let file = File::create(root.join("metadata.json"))?;
            let mut file = BufWriter::new(file);
            file.write_all(&json.into_bytes())?;
        }
        Ok(())
    }
}

fn ask_contributor_name(siv: &mut Cursive) -> String {
    siv.add_layer(
        Dialog::new()
            .title("Enter contributor name")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Name:"))
                    .child(EditView::new().on_submit(|s, _| s.quit()).with_name("name")),
            )
            .button("Ok", |s| s.quit())
            .fixed_width(70),
    );
    siv.run();
    let name = siv.get_edit_view_value("name");
    siv.pop_layer();
    name
}

fn trim(text: &str) -> Option<String> {
    let text = text.trim();
    if text.len() == 0 {
        None
    } else {
        Some(text.to_owned())
    }
}

fn ask_submission(
    siv: &mut Cursive,
    cfgs: &BTreeMap<String, GameConfig>,
    contributor: &str,
) -> Option<(PathBuf, Cartridge)> {
    ask_cartridge(siv, cfgs).and_then(|(code, shell, board)| {
        let game_root = PathBuf::from("data/cartridges").join(&code);
        let contributor_slug = slugify(contributor);
        for index in 1..=20 {
            let slug = format!("{}-{}", contributor_slug, index);
            let root = game_root.join(&slug);
            if !root.join("metadata.json").exists() {
                return Some((
                    root,
                    Cartridge {
                        code,
                        slug,
                        contributor: contributor.to_owned(),
                        index,
                        shell,
                        board,
                        dump: None,
                    },
                ));
            }
        }
        None
    })
}

fn ask_cartridge(
    siv: &mut Cursive,
    cfgs: &BTreeMap<String, GameConfig>,
) -> Option<(String, CartridgeShell, CartridgeBoard)> {
    ask_shell(siv).and_then(|shell| {
        ask_game_code(siv, cfgs).and_then(|code| {
            cfgs.get(&code)
                .and_then(|_| ask_board(siv).map(|board| (code, shell, board)))
        })
    })
}

fn ask_shell(siv: &mut Cursive) -> Option<CartridgeShell> {
    siv.add_layer(
        Dialog::new()
            .title("Enter cartridge shell details")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Code:"))
                    .child(EditView::new().with_name("code"))
                    .child(TextView::new("Stamp:"))
                    .child(EditView::new().with_name("stamp")),
            )
            .button("Ok", |s| s.quit())
            .fixed_width(70),
    );
    siv.run();
    let code = siv.get_edit_view_value("code");
    let stamp = siv.get_edit_view_value("stamp");
    siv.pop_layer();
    if should_quit() {
        None
    } else {
        Some(CartridgeShell {
            code: trim(&code),
            stamp: trim(&stamp),
            outlier: false,
        })
    }
}

fn ask_game_code(siv: &mut Cursive, cfgs: &BTreeMap<String, GameConfig>) -> Option<String> {
    let cfgs = cfgs.clone();
    siv.add_layer(
        Dialog::new()
            .title("Enter game code")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Code:"))
                    .child(
                        EditView::new()
                            .on_submit(|s, _| s.quit())
                            .on_edit(move |siv, content, _| {
                                siv.call_on_name("title", |title: &mut TextView| {
                                    match cfgs.get(content) {
                                        Some(cfg) => title.set_content(cfg.name.to_owned()),
                                        None => title.set_content(""),
                                    }
                                })
                                .unwrap();
                            })
                            .with_name("code"),
                    )
                    .child(DummyView.fixed_height(1))
                    .child(TextView::new("").with_name("title").fixed_height(1)),
            )
            .button("Ok", |s| s.quit())
            .fixed_width(150),
    );
    siv.run();
    let code = siv.get_edit_view_value("code");
    siv.pop_layer();
    if should_quit() {
        None
    } else {
        trim(&code)
    }
}

fn ask_board(siv: &mut Cursive) -> Option<CartridgeBoard> {
    siv.add_layer(
        Dialog::new()
            .title("Enter board details")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Label:"))
                    .child(EditView::new().with_name("label"))
                    .child(TextView::new("Circled letters:"))
                    .child(EditView::new().with_name("circled_letters"))
                    .child(TextView::new("Panel position:"))
                    .child(EditView::new().with_name("panel_position"))
                    .child(TextView::new("Year:"))
                    .child(EditView::new().with_name("year"))
                    .child(TextView::new("Month:"))
                    .child(EditView::new().with_name("month")),
            )
            .button("Ok", |s| s.quit())
            .fixed_width(70),
    );
    siv.run();
    let label = siv.get_edit_view_value("label");
    let circled_letters = siv.get_edit_view_value("circled_letters");
    let panel_position = siv.get_edit_view_value("panel_position");
    let year = siv.get_edit_view_value("year");
    let month = siv.get_edit_view_value("month");
    siv.pop_layer();
    if should_quit() {
        return None;
    }
    let parts = PartRoleConfig::from(BoardLayout::from_label(&label).unwrap());
    siv.add_layer(
        Dialog::new()
            .title("Enter part details")
            .content(
                LinearLayout::vertical()
                    .child(part_editor("u1", parts.u1))
                    .child(part_editor("u2", parts.u2))
                    .child(part_editor("u3", parts.u3))
                    .child(part_editor("u4", parts.u4))
                    .child(part_editor("u5", parts.u5))
                    .child(part_editor("u6", parts.u6))
                    .child(part_editor("u7", parts.u7))
                    .child(part_editor("x1", parts.x1)),
            )
            .button("Ok", |s| s.quit())
            .fixed_width(150),
    );
    siv.run();
    let board = trim(&label).map(|label| CartridgeBoard {
        label,
        circled_letters: trim(&circled_letters),
        panel_position: trim(&panel_position),
        year: trim(&year).map(|year| u16::from_str(&year).unwrap()),
        month: trim(&month).map(|month| Month::try_from(u8::from_str(&month).unwrap()).unwrap()),
        u1: add_part(siv, parts.u1, "u1"),
        u2: add_part(siv, parts.u2, "u2"),
        u3: add_part(siv, parts.u3, "u3"),
        u4: add_part(siv, parts.u4, "u4"),
        u5: add_part(siv, parts.u5, "u5"),
        u6: add_part(siv, parts.u6, "u6"),
        u7: add_part(siv, parts.u7, "u7"),
        x1: add_part(siv, parts.x1, "x1"),
        outlier: false,
    });
    siv.pop_layer();
    if should_quit() {
        None
    } else {
        board
    }
}

fn part_editor(id: &str, role: Option<PartRole>) -> LinearLayout {
    let mut editor = EditView::new();
    let mut result = LinearLayout::vertical();
    let details_id = format!("{}_details", id);
    if let Some(role) = role {
        result.add_child(TextView::new(id));
        let details = TextView::new("")
            .with_name(details_id.clone())
            .fixed_height(2);
        match role {
            PartRole::Rom => {
                add_details_callback(&mut editor, &details_id, parser::mask_rom::mask_rom())
            }
            PartRole::Mapper => {
                add_details_callback(&mut editor, &details_id, parser::mapper::mapper())
            }
            PartRole::Ram => add_details_callback(&mut editor, &details_id, parser::ram::ram()),
            PartRole::SupervisorReset => add_details_callback(
                &mut editor,
                &details_id,
                parser::supervisor_reset::supervisor_reset(),
            ),
            PartRole::Crystal => add_details_callback(
                &mut editor,
                &details_id,
                parser::crystal_32kihz::crystal_32kihz(),
            ),
            PartRole::Flash => {
                add_details_callback(&mut editor, &details_id, parser::flash::flash())
            }
            PartRole::Eeprom => {
                add_details_callback(&mut editor, &details_id, parser::eeprom::eeprom())
            }
            PartRole::Accelerometer => add_details_callback(
                &mut editor,
                &details_id,
                parser::accelerometer::accelerometer(),
            ),
            PartRole::LineDecoder => add_details_callback(
                &mut editor,
                &details_id,
                parser::line_decoder::line_decoder(),
            ),
            _ => (),
        }
        result.add_child(editor.with_name(id));
        result.add_child(details);
    }
    result
}

fn add_details_callback<T: fmt::Debug, F: LabelParser<T>>(
    editor: &mut EditView,
    details_id: &str,
    f: &'static F,
) {
    let details_id = details_id.to_owned();
    editor.set_on_edit(move |siv, content, _| {
        siv.call_on_name(&details_id, |view: &mut TextView| match f.parse(&content) {
            Ok(part) => view.set_content(format!("{:?}", part)),
            Err(err) => view.set_content(format!("{}", err)),
        })
        .unwrap();
    });
}

fn add_part(siv: &mut Cursive, role: Option<PartRole>, id: &str) -> Option<Part> {
    role.map(|_| match siv.get_edit_view_value(id).as_str() {
        "-" => Part {
            label: None,
            outlier: false,
        },
        label => Part::from_label(trim(label)),
    })
}
