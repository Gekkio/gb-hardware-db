// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::{GameConfig, GamePlatform};
use itertools::Itertools;
use maud::{html, Markup, Render};
use std::{borrow::Cow, collections::BTreeMap};

use crate::{legacy::LegacyCartridgeSubmission, template::mapper::MapperCfg, LegacyPhotos};

pub struct PlatformCartridges<'a> {
    pub platform: GamePlatform,
    pub mapper_cfgs: &'a [MapperCfg],
    pub cfgs: &'a BTreeMap<String, GameConfig>,
    pub submissions: &'a [LegacyCartridgeSubmission],
}

impl<'a> Render for PlatformCartridges<'a> {
    fn render(&self) -> Markup {
        let mut per_game = Vec::new();
        for (code, cfg) in self.cfgs {
            if cfg.platform == self.platform {
                let group = self
                    .submissions
                    .iter()
                    .filter(|&submission| submission.code == *code)
                    .collect::<Vec<_>>();
                per_game.push((cfg, group));
            }
        }
        per_game.sort_by_key(|(cfg, _)| &cfg.name);
        let toggle_js = "\
var shouldHide = event.currentTarget.innerText.includes('Show only');
event.currentTarget.innerHTML = (shouldHide)
    ? 'Show all games'
    : 'Show only games with submissions';
document.querySelectorAll('tr.empty').forEach((m) => {
    m.hidden = shouldHide;
});";
        html! {
            article {
                h2 { (self.platform.name()) " cartridges" }
                @if self.platform.has_mappers() {
                    h3 { "GB/GBC cartridges by mapper" }
                    ul.cartridges__mapper-list {
                        @for cfg in self.mapper_cfgs {
                            li {
                                a href={ "/cartridges/" (cfg.id) ".html" } { (cfg.name) }
                            }
                        }
                    }
                }
                h3 { "Cartridges by game" }
                button.jsonly onclick=( toggle_js ) hidden {
                    "Show only games with submissions"
                }
                table {
                    thead {
                        tr {
                            th { "Title" }
                            th { "ROM ID" }
                            th { "Year(s)" }
                            th { "Release(s)" }
                            th { "Board type(s)" }
                            @if self.platform.has_mappers() {
                                th { "Mapper(s)" }
                            }
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
    let mappers = submissions.iter().filter_map(|submission| {
        let board = &submission.metadata.board;
        board
            .mapper()
            .and_then(|part| part.kind.as_deref().map(Cow::Borrowed))
    });
    let photo_submissions = submissions
        .into_iter()
        .filter(|s| s.photos.front().is_some())
        .take(1)
        .collect::<Vec<_>>();
    html! {
        tr.empty[submissions.len() == 0] {
            td.submission-list-item {
                @if submissions.len() > 0 {
                    a.submission-list-item__link href={ "/cartridges/" (cfg.rom_id) } {
                        @for submission in &photo_submissions {
                            @let code = &submission.code;
                            @let slug = &submission.slug;
                            img
                                src=(format!("/static/{code}/{slug}_thumbnail_80.jpg"))
                                srcSet=(format!("/static/{code}/{slug}_thumbnail_50.jpg 50w, /static/{code}/{slug}_thumbnail_80.jpg 80w"))
                                sizes="(min-width: 1000px) 80px, 50px"
                                role="presentation";
                        }
                        @if photo_submissions.len() > 0 {
                            br;
                        }
                        (cfg.name)
                    }
                } @else {
                    (cfg.name)
                }
            }
            td { (cfg.rom_id) }
            td { (multiline(years)) }
            td { (multiline(releases)) }
            td { (multiline(board_types)) }
            @if cfg.platform.has_mappers() {
                td { (multiline(mappers)) }
            }
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
