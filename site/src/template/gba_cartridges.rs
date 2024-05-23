// SPDX-FileCopyrightText: 2017-2024 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{config::cartridge::GameConfig, CartridgeClass};
use itertools::Itertools;
use maud::{html, Markup, Render};
use std::{borrow::Cow, collections::BTreeMap};

use crate::legacy::LegacyCartridgeSubmission;

pub struct GbaCartridges<'a> {
    pub cfgs: &'a BTreeMap<String, GameConfig>,
    pub submissions: &'a [LegacyCartridgeSubmission],
}

impl<'a> Render for GbaCartridges<'a> {
    fn render(&self) -> Markup {
        let mut per_game = Vec::new();
        for (code, group) in &self
            .submissions
            .iter()
            .sorted_by_key(|submission| &submission.code)
            .chunk_by(|submission| &submission.code)
        {
            let cfg = &self.cfgs[code];
            if CartridgeClass::from(cfg.platform) == CartridgeClass::Gba {
                per_game.push((cfg, group.collect::<Vec<_>>()));
            }
        }
        per_game.sort_by_key(|(cfg, _)| &cfg.name);
        html! {
            article {
                h2 { "Game Boy Advance cartridges" }
                h3 { "Cartridges by game" }
                table {
                    thead {
                        tr {
                            th { "Title" }
                            th { "ROM ID" }
                            th { "Year(s)" }
                            th { "Release(s)" }
                            th { "Board type(s)" }
                            th { "Submissions" }
                        }
                    }
                    tbody {
                        @for (cfg, submissions) in &per_game {
                            (render_game(cfg, submissions))
                        }
                    }
                }
            }
        }
    }
}

fn render_game(cfg: &GameConfig, submissions: &[&LegacyCartridgeSubmission]) -> Markup {
    let years = submissions.iter().filter_map(|submission| {
        submission
            .metadata
            .board
            .date_code
            .year
            .map(|year| Cow::Owned(year.to_string()))
    });
    let releases = submissions
        .iter()
        .filter_map(|submission| submission.metadata.code.as_deref().map(Cow::Borrowed));
    let board_types = submissions
        .iter()
        .map(|submission| Cow::Borrowed(submission.metadata.board.kind.as_ref()));
    html! {
        tr {
            td.submission-list-item {
                a.submission-list-item__link href={ "/cartridges/" (cfg.rom_id) } { (cfg.name) }
            }
            td { (cfg.rom_id) }
            td { (multiline(years)) }
            td { (multiline(releases)) }
            td { (multiline(board_types)) }
            td { (submissions.len()) }
        }
    }
}

fn multiline<'a>(lines: impl Iterator<Item = Cow<'a, str>>) -> Markup {
    let lines = lines.unique().sorted();
    html! {
        @for line in lines {
            span {
                (line)
                br;
            }
        }
    }
}
