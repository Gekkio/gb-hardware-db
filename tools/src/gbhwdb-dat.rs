use cursive::traits::*;
use cursive::views::*;
use cursive::Cursive;
use failure::{format_err, Error};
use gbhwdb_tools::config::{BoardLayout, GameConfig, GamePlatform};
use gbhwdb_tools::dat::DatFile;
use glob::glob;
use itertools::Itertools;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{self, AtomicBool};
use strsim::jaro;

#[derive(Clone, Debug)]
struct Dats {
    gb: DatFile,
    gbc: DatFile,
}

impl Dats {
    pub fn get_platform(&self, name: &str) -> Option<GamePlatform> {
        match (self.gb.names.contains(name), self.gbc.names.contains(name)) {
            (true, false) => Some(GamePlatform::Gb),
            (false, true) => Some(GamePlatform::Gbc),
            _ => None,
        }
    }
}

fn load_dats() -> Result<Dats, Error> {
    let mut gb_dat = None;
    let mut gbc_dat = None;
    for entry in glob("*.dat")
        .expect("Invalid glob pattern")
        .filter_map(Result::ok)
    {
        match gbhwdb_tools::dat::from_path(&entry) {
            Ok(dat) => match dat.header.as_str() {
                "Nintendo - Game Boy" => gb_dat = Some(dat),
                "Nintendo - Game Boy Color" => gbc_dat = Some(dat),
                _ => (),
            },
            Err(e) => eprintln!("Failed to read {}: {}", entry.to_string_lossy(), e),
        }
    }
    match (gb_dat, gbc_dat) {
        (Some(gb), Some(gbc)) => Ok(Dats { gb, gbc }),
        (None, Some(_)) => Err(format_err!("No GB dat found")),
        (None, None) => Err(format_err!("No GBC dat found")),
        _ => Err(format_err!("No dats found")),
    }
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
    SaveQuit,
    Quit,
}

fn main_menu(siv: &mut Cursive, cfgs: &BTreeMap<String, GameConfig>, dats: &Dats) -> Command {
    let mut select = SelectView::new();
    select.add_item("Synchronize", Command::Sync);
    select.add_item("Add a game", Command::Add);
    select.add_item("Save and quit", Command::SaveQuit);
    let choice = Rc::new(Cell::new(Command::Quit));
    {
        let choice = choice.clone();
        select.set_on_submit(move |s, &selected| {
            choice.set(selected);
            s.quit();
        });
    }
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
                .child(TextView::new(format!("Number of games: {}", cfgs.len())))
                .child(DummyView.fixed_height(1))
                .child(select),
        ),
    );
    siv.run();
    siv.pop_layer();
    choice.get()
}

static QUIT: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Error> {
    let mut cfgs = load_cfgs("../config/games.json")?;
    let dats = load_dats()?;
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| {
        QUIT.store(true, atomic::Ordering::SeqCst);
        s.quit();
    });
    while !QUIT.load(atomic::Ordering::SeqCst) {
        let cmd = main_menu(&mut siv, &cfgs, &dats);
        match cmd {
            Command::Sync => sync(&mut siv, &mut cfgs, &dats),
            Command::Add => add(&mut siv, &mut cfgs, &dats),
            Command::SaveQuit => {
                write_cfgs("../config/games.json", &cfgs)?;
                break;
            }
            Command::Quit => {
                break;
            }
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
    let gb_names = dats.gb.names.iter();
    let gbc_names = dats.gbc.names.iter();
    let names = gb_names.chain(gbc_names).cloned().collect::<HashSet<_>>();
    let gb_games = dats
        .gb
        .names
        .iter()
        .cloned()
        .map(|name| (GamePlatform::Gb, name));
    let gbc_games = dats
        .gbc
        .names
        .iter()
        .cloned()
        .map(|name| (GamePlatform::Gbc, name));
    let games = gb_games.chain(gbc_games).collect::<Vec<_>>();
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
                .map(|c| (format!("{}", c), Some(c.name)));
            let mut select = SelectView::new();
            select.add_item("(skip)", None);
            select.add_all(candidates);
            let choice = Rc::new(Cell::new(None));
            {
                let choice = choice.clone();
                select.set_on_submit(move |s, selected| {
                    choice.set(selected.as_ref().cloned());
                    s.quit();
                });
            }
            {
                let current_name = game_cfg.name.clone();
                select.set_on_select(move |s, selected| {
                    let content = match selected {
                        Some(candidate) => format!("After:  {}", candidate),
                        None => format!("After:  {}", &current_name),
                    };
                    s.call_on_id("selection", |view: &mut TextView| {
                        view.set_content(content);
                    });
                });
            }
            siv.add_layer(
                Dialog::new()
                    .title(format!("Fix name problem {}/{}", idx + 1, total))
                    .padding((2, 2, 1, 1))
                    .content(
                        LinearLayout::vertical()
                            .child(TextView::new(format!("Game code: {}", code)))
                            .child(TextView::new(format!("Before: {}", game_cfg.name)))
                            .child(
                                TextView::new(format!("After:  {}", game_cfg.name))
                                    .with_id("selection"),
                            )
                            .child(DummyView.fixed_height(1))
                            .child(select),
                    ),
            );
            siv.run();
            siv.pop_layer();
            if QUIT.load(atomic::Ordering::SeqCst) {
                return;
            }
            if let Some(name) = choice.replace(None) {
                game_cfg.name = name;
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
            if QUIT.load(atomic::Ordering::SeqCst) {
                return;
            }
            if choice.get() {
                cfg.platform = platform;
            }
        }
    }

    siv.add_layer(
        Dialog::around(TextView::new("Synchronization complete")).button("Ok", |s| s.quit()),
    );
    siv.run();
    siv.pop_layer();
}

fn add(siv: &mut Cursive, cfgs: &mut BTreeMap<String, GameConfig>, dats: &Dats) {
    let existing_names = cfgs
        .values()
        .map(|cfg| cfg.name.clone())
        .collect::<HashSet<_>>();
    let gb_games = dats
        .gb
        .names
        .iter()
        .cloned()
        .map(|name| (GamePlatform::Gb, name));
    let gbc_games = dats
        .gbc
        .names
        .iter()
        .cloned()
        .map(|name| (GamePlatform::Gbc, name));
    let games = gb_games.chain(gbc_games).collect::<Vec<_>>();
    let mut search = EditView::new();
    search.set_on_edit(move |s, text, _| {
        s.call_on_id("search_results", |results: &mut SelectView<Candidate>| {
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
        });
    });
    let choice = Rc::new(Cell::<Option<Candidate>>::new(None));
    {
        let choice = choice.clone();
        search.set_on_submit(move |s, _| {
            if let Some(Some(selection)) = s
                .call_on_id("search_results", |results: &mut SelectView<Candidate>| {
                    results.selection()
                })
            {
                choice.set(Some(Clone::clone(&selection)));
                s.quit();
            }
        });
    }
    let mut select = SelectView::<Candidate>::new();
    {
        let choice = choice.clone();
        select.set_on_submit(move |s, selection: &Candidate| {
            choice.set(Some(Clone::clone(&selection)));
            s.quit();
        });
    }
    siv.add_layer(
        Dialog::new()
            .title("Select game to add")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Search:"))
                    .child(search)
                    .child(DummyView.fixed_height(1))
                    .child(TextView::new("Results:"))
                    .child(select.with_id("search_results").fixed_height(10)),
            )
            .fixed_width(150),
    );
    siv.run();
    siv.pop_layer();
    let (platform, name) = choice
        .replace(None)
        .map(|c| (c.platform, c.name))
        .unwrap_or((GamePlatform::Gb, String::new()));
    if name.len() == 0 || QUIT.load(atomic::Ordering::SeqCst) {
        return;
    }
    let code = EditView::new();
    let mut layout = RadioGroup::new();
    let layout_choice = Rc::new(Cell::new(BoardLayout::RomMapperRam));
    let layout_container = LinearLayout::vertical()
        .child(layout.button(BoardLayout::Rom, "Rom"))
        .child(layout.button(BoardLayout::RomMapper, "Rom + mapper"))
        .child(
            layout
                .button(BoardLayout::RomMapperRam, "Rom + mapper + ram")
                .selected(),
        )
        .child(layout.button(
            BoardLayout::RomMapperRamXtal,
            "Rom + mapper + ram + crystal",
        ))
        .child(layout.button(BoardLayout::Mbc2, "MBC2"))
        .child(layout.button(BoardLayout::Mbc6, "MBC6"))
        .child(layout.button(BoardLayout::Mbc7, "MBC7"))
        .child(layout.button(BoardLayout::Type15, "Type 15 (MBC5 + dual ROM)"))
        .child(layout.button(BoardLayout::Huc3, "HuC-3"))
        .child(layout.button(BoardLayout::Tama, "Tamagotchi 3"));
    {
        let layout_choice = layout_choice.clone();
        layout.set_on_change(move |_, &layout| layout_choice.set(layout));
    }
    let mut dialog = Dialog::new().title("Add a game").content(
        LinearLayout::vertical()
            .child(TextView::new("Name:"))
            .child(TextView::new(name.as_str()))
            .child(TextView::new("Platform:"))
            .child(TextView::new(format!("{}", platform)))
            .child(TextView::new("Code:"))
            .child(code.with_id("code"))
            .child(TextView::new("Board layout:"))
            .child(layout_container),
    );
    let code_value = Rc::new(Cell::new(None));
    {
        let code_value = code_value.clone();
        dialog.add_button("Ok", move |s| {
            s.call_on_id("code", |code: &mut EditView| {
                code_value.set(Some(code.get_content().to_string()));
            });
            s.quit();
        });
    }
    siv.add_layer(dialog.fixed_width(150));
    siv.run();
    siv.pop_layer();
    if QUIT.load(atomic::Ordering::SeqCst) {
        return;
    }
    if let Some(code) = code_value.replace(None) {
        if code.len() > 0 {
            cfgs.insert(
                code,
                GameConfig {
                    name,
                    platform,
                    layout: layout_choice.get(),
                },
            );
        }
    }
}
