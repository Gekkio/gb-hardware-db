// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Error};
use cursive::{traits::*, view::Margins, views::*, Cursive, CursiveExt};
use gbhwdb_backend::config::cartridge::{GameConfig, GamePlatform};
use gbhwdb_tools::{cursive::*, dat::DatFile};
use glob::glob;
use itertools::Itertools;
use std::{
    cell::Cell,
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    fmt,
    fs::File,
    io::{BufReader, BufWriter},
    ops::Index,
    path::Path,
    rc::Rc,
    sync::atomic::{self, AtomicBool},
};
use strsim::jaro;

#[derive(Clone, Debug)]
struct Dats {
    gb: DatFile,
    gbc: DatFile,
    gba: DatFile,
}

impl Index<GamePlatform> for Dats {
    type Output = DatFile;

    fn index(&self, index: GamePlatform) -> &Self::Output {
        match index {
            GamePlatform::Gb => &self.gb,
            GamePlatform::Gbc => &self.gbc,
            GamePlatform::Gba => &self.gba,
        }
    }
}

impl Dats {
    pub fn get_platform(&self, name: &str) -> Option<GamePlatform> {
        match (
            self.gb.games.contains_key(name),
            self.gbc.games.contains_key(name),
            self.gba.games.contains_key(name),
        ) {
            (true, false, false) => Some(GamePlatform::Gb),
            (false, true, false) => Some(GamePlatform::Gbc),
            (false, false, true) => Some(GamePlatform::Gba),
            _ => None,
        }
    }
    pub fn all_names(&self) -> HashSet<String> {
        self.gb
            .games
            .keys()
            .chain(self.gbc.games.keys())
            .chain(self.gba.games.keys())
            .cloned()
            .collect()
    }
    pub fn all_games(&self) -> Vec<(GamePlatform, String)> {
        let gb = self
            .gb
            .games
            .iter()
            .map(|(name, _)| (GamePlatform::Gb, name.clone()));
        let gbc = self
            .gbc
            .games
            .iter()
            .map(|(name, _)| (GamePlatform::Gbc, name.clone()));
        let gba = self
            .gba
            .games
            .iter()
            .map(|(name, _)| (GamePlatform::Gba, name.clone()));
        gb.chain(gbc).chain(gba).collect()
    }
}

fn load_dats() -> Result<Dats, Error> {
    let mut gb_dat = None;
    let mut gbc_dat = None;
    let mut gba_dat = None;
    for entry in glob("*.dat")
        .expect("Invalid glob pattern")
        .filter_map(Result::ok)
    {
        match gbhwdb_tools::dat::from_path(&entry) {
            Ok(dat) => match dat.header.as_str() {
                "Nintendo - Game Boy" => gb_dat = Some(dat),
                "Nintendo - Game Boy Color" => gbc_dat = Some(dat),
                "Nintendo - Game Boy Advance" => gba_dat = Some(dat),
                _ => (),
            },
            Err(e) => eprintln!("Failed to read {}: {}", entry.to_string_lossy(), e),
        }
    }
    Ok(Dats {
        gb: gb_dat.ok_or(anyhow!("No GB dat found"))?,
        gbc: gbc_dat.ok_or(anyhow!("No GBC dat found"))?,
        gba: gba_dat.ok_or(anyhow!("No GBA dat found"))?,
    })
}

fn load_cfgs<P: AsRef<Path>>(path: P) -> Result<BTreeMap<String, GameConfig>, Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let cfgs = serde_json::from_reader(file)?;
    Ok(cfgs)
}

fn write_cfgs<P: AsRef<Path>>(path: P, cfgs: &BTreeMap<String, GameConfig>) -> Result<(), Error> {
    let file = File::create(path)?;
    let file = BufWriter::new(file);
    serde_json::to_writer_pretty(file, cfgs)?;
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Command {
    Sync,
    Add,
    Quit,
}

fn main_menu(siv: &mut Cursive, cfgs: &BTreeMap<String, GameConfig>, dats: &Dats) -> Command {
    siv.add_layer(
        Dialog::new().title("gbhwdb-dat").content(
            LinearLayout::vertical()
                .child(TextView::new(format!(
                    " GB dat version: {}",
                    dats.gb.version
                )))
                .child(TextView::new(format!(
                    "GBC dat version: {}",
                    dats.gbc.version
                )))
                .child(TextView::new(format!(
                    "GBA dat version: {}",
                    dats.gba.version
                )))
                .child(TextView::new(format!("Game count: {}", cfgs.len())))
                .child(DummyView.fixed_height(1))
                .child(
                    SelectView::new()
                        .item("Synchronize", Command::Sync)
                        .item("Add a game", Command::Add)
                        .item("Quit", Command::Quit)
                        .on_submit(|s, _| s.quit())
                        .with_name("cmd"),
                ),
        ),
    );
    siv.run();
    let cmd = siv.get_select_view_selection::<Command>("cmd");
    siv.pop_layer();
    if should_quit() {
        Command::Quit
    } else {
        cmd.unwrap_or(Command::Quit)
    }
}

static QUIT: AtomicBool = AtomicBool::new(false);

fn should_quit() -> bool {
    QUIT.load(atomic::Ordering::SeqCst)
}

fn main() -> Result<(), Error> {
    let mut cfgs = load_cfgs("config/games.json")?;
    let dats = load_dats()?;
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| {
        QUIT.store(true, atomic::Ordering::SeqCst);
        s.quit();
    });
    while !should_quit() {
        let cmd = main_menu(&mut siv, &cfgs, &dats);
        match cmd {
            Command::Sync => {
                sync(&mut siv, &mut cfgs, &dats);
                write_cfgs("config/games.json", &cfgs)?;
            }
            Command::Add => {
                add(&mut siv, &mut cfgs, &dats);
                write_cfgs("config/games.json", &cfgs)?;
            }
            Command::Quit => break,
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Candidate {
    platform: GamePlatform,
    name: String,
    rating: f64,
}

impl Candidate {
    pub fn new(platform: GamePlatform, current_name: &str, name: &str) -> Candidate {
        Candidate {
            platform,
            name: name.to_owned(),
            rating: jaro(&current_name.to_lowercase(), &name.to_lowercase()),
        }
    }
}

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Score {:.02}: {} [{}]",
            self.rating, self.name, self.platform
        )
    }
}

fn sync(siv: &mut Cursive, cfgs: &mut BTreeMap<String, GameConfig>, dats: &Dats) {
    let names = dats.all_names();
    let games = dats.all_games();
    let name_problems = cfgs
        .iter_mut()
        .filter(|(_, game_cfg)| !names.contains(&game_cfg.name))
        .collect::<Vec<_>>();
    if name_problems.len() > 0 {
        let total = name_problems.len();
        for (idx, (code, game_cfg)) in name_problems.into_iter().enumerate() {
            let candidates = games
                .iter()
                .map(|(platform, name)| Candidate::new(*platform, &game_cfg.name, &name))
                .sorted_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(Ordering::Equal))
                .take(5)
                .map(|c| (format!("{}", c), Some(c)));
            let current_name = game_cfg.name.clone();
            siv.add_layer(
                Dialog::new()
                    .title(format!("Fix name problem {}/{}", idx + 1, total))
                    .padding(Margins::lrtb(2, 2, 1, 1))
                    .content(
                        LinearLayout::vertical()
                            .child(TextView::new(format!("Game code: {}", code)))
                            .child(TextView::new(format!("Before: {}", game_cfg.name)))
                            .child(
                                TextView::new(format!("After:  {}", game_cfg.name))
                                    .with_name("selection"),
                            )
                            .child(DummyView.fixed_height(1))
                            .child(
                                SelectView::new()
                                    .item("(skip)", None)
                                    .with_all(candidates)
                                    .on_submit(|s, _| s.quit())
                                    .on_select(move |s, selected| {
                                        let name = selected
                                            .as_ref()
                                            .map(|c| &c.name)
                                            .unwrap_or(&current_name);
                                        s.set_text_view_content(
                                            "selection",
                                            format!("After:  {}", name),
                                        );
                                    })
                                    .with_name("choice"),
                            ),
                    ),
            );
            siv.run();
            let choice = siv
                .get_select_view_selection::<Option<Candidate>>("choice")
                .and_then(|c| c);
            siv.pop_layer();
            if should_quit() {
                return;
            }
            if let Some(c) = choice {
                game_cfg.name = c.name;
            }
        }
    }

    let platform_problems = cfgs
        .iter_mut()
        .filter_map(|(code, cfg)| match dats.get_platform(&cfg.name) {
            Some(platform) if platform != cfg.platform => Some((code, cfg, platform)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if platform_problems.len() > 0 {
        let total = platform_problems.len();
        for (idx, (code, cfg, platform)) in platform_problems.into_iter().enumerate() {
            let choice = Rc::new(Cell::new(false));
            let mut dialog = Dialog::new()
                .title(format!("Fix platform problem {}/{}", idx + 1, total))
                .content(
                    LinearLayout::vertical()
                        .child(TextView::new(format!("Game code: {}", code)))
                        .child(TextView::new(format!("Before: {}", cfg.platform)))
                        .child(TextView::new(format!("After:  {}", platform))),
                );
            {
                let choice = choice.clone();
                dialog.add_button("Ok", move |s| {
                    choice.set(true);
                    s.quit();
                });
            }
            {
                let choice = choice.clone();
                dialog.add_button("Cancel", move |s| {
                    choice.set(false);
                    s.quit();
                });
            }
            siv.add_layer(dialog);
            siv.run();
            siv.pop_layer();
            if should_quit() {
                return;
            }
            if choice.get() {
                cfg.platform = platform;
            }
        }
    }

    for cfg in cfgs.values_mut() {
        let dat_game = &dats[cfg.platform].games[&cfg.name];
        cfg.rom_verified = dat_game.rom_verified;
        cfg.no_intro_id = dat_game.id.clone();
        if dat_game.sha256.is_some() {
            cfg.sha256 = dat_game.sha256;
        }
    }
    siv.add_layer(
        Dialog::around(TextView::new("Synchronization complete")).button("Ok", |s| s.quit()),
    );
    siv.run();
    siv.pop_layer();
}

fn add(siv: &mut Cursive, cfgs: &mut BTreeMap<String, GameConfig>, dats: &Dats) {
    siv.add_layer(
        Dialog::new()
            .title("Enter game code")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Code:"))
                    .child(EditView::new().on_submit(|s, _| s.quit()).with_name("code")),
            )
            .button("Ok", |s| s.quit())
            .fixed_width(70),
    );
    siv.run();
    let code = siv.get_edit_view_value("code");
    siv.pop_layer();
    if code.len() == 0 || cfgs.contains_key(&code) {
        return;
    }

    let games = dats.all_games();
    let mut search = EditView::new();
    search.set_on_edit(move |s, text, _| {
        s.call_on_name("search_results", |results: &mut SelectView<Candidate>| {
            results.clear();
            if text.len() > 0 {
                let candidates = games
                    .iter()
                    .map(|(platform, name)| Candidate::new(*platform, text, &name))
                    .sorted_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(Ordering::Equal))
                    .take(10)
                    .map(|c| (format!("{}", c), c));
                results.add_all(candidates);
            }
        })
        .unwrap();
    });
    siv.add_layer(
        Dialog::new()
            .title("Select game to add")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Search:"))
                    .child(search)
                    .child(DummyView.fixed_height(1))
                    .child(TextView::new("Results:"))
                    .child(
                        SelectView::<Candidate>::new()
                            .on_submit(|s, _| s.quit())
                            .with_name("search_results")
                            .fixed_height(10),
                    ),
            )
            .fixed_width(150),
    );
    siv.run();
    let found_game = siv
        .get_select_view_selection::<Candidate>("search_results")
        .and_then(|c| {
            let dat = &dats[c.platform];
            let dat_game = dat.games.get(&c.name)?;
            Some((c, dat_game))
        });
    siv.pop_layer();
    if should_quit() {
        return;
    };
    let Some((candidate, dat_game)) = found_game else {
        return;
    };
    let mut dialog = Dialog::new().title("Add a game").content(
        LinearLayout::vertical()
            .child(TextView::new("Name:"))
            .child(TextView::new(dat_game.name.as_str()))
            .child(TextView::new("Platform:"))
            .child(TextView::new(format!("{}", candidate.platform)))
            .child(TextView::new("Code:"))
            .child(TextView::new(code.as_str())),
    );
    dialog.add_button("Ok", move |s| s.quit());
    siv.add_layer(dialog.fixed_width(150));
    siv.run();
    siv.pop_layer();
    if should_quit() {
        return;
    }
    let rom_id = code.clone();
    cfgs.insert(
        code,
        GameConfig {
            name: dat_game.name.clone(),
            rom_id,
            rom_verified: dat_game.rom_verified,
            sha256: dat_game.sha256,
            platform: candidate.platform,
            no_intro_id: dat_game.id.clone(),
        },
    );
}
