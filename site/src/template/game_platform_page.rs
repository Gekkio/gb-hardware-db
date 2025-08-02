// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::{GameConfig, GamePlatform};
use itertools::Itertools;
use lexical_sort::natural_lexical_cmp;
use maud::{Markup, Render, html};
use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    LegacyPhotos, legacy::LegacyCartridgeSubmission, site::board_kind_link,
    template::mapper_page::MapperCfg,
};

pub struct GamePlatformPage<'a> {
    pub platform: GamePlatform,
    pub mapper_cfgs: &'a [MapperCfg],
    pub cfgs: &'a BTreeMap<String, GameConfig>,
    pub submissions: &'a [LegacyCartridgeSubmission],
}

impl<'a> Render for GamePlatformPage<'a> {
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
        per_game.sort_unstable_by(|(a, _), (b, _)| natural_lexical_cmp(&a.name, &b.name));
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
                table.game-platform-page {
                    colgroup {
                        col;
                        col;
                        col;
                        col;
                        col;
                        @if self.platform.has_mappers() {
                            col;
                        }
                        col;
                    }
                    thead {
                        tr {
                            th scope="col" { "Title" }
                            th scope="col" { "ROM ID" }
                            th scope="col" { "Year(s)" }
                            th scope="col" { "Release(s)" }
                            th scope="col" { "Board type(s)" }
                            @if self.platform.has_mappers() {
                                th scope="col" { "Mapper(s)" }
                            }
                            th scope="col" { "Submissions" }
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
        .unique_by(|&&s| &s.metadata.board.kind)
        .sorted_by_key(|&&s| &s.metadata.board.kind)
        .map(|&s| board_kind_link(&s.metadata.board));

    let mappers = submissions.iter().filter_map(|submission| {
        let board = &submission.metadata.board;
        board
            .mapper()
            .and_then(|part| part.kind.as_deref().map(Cow::Borrowed))
    });
    let photo_submissions = submissions
        .into_iter()
        .filter(|s| s.photos.front().is_some())
        .collect::<Vec<_>>();
    html! {
        tr.empty[submissions.len() == 0] {
            td.submission-list-item {
                @if submissions.len() > 0 {
                    a.submission-list-item__link href={ "/cartridges/" (cfg.rom_id) } {
                        div.submission-list-item__photos {
                            @for submission in &photo_submissions {
                                @let code = &submission.code;
                                @let slug = &submission.slug;
                                img
                                    src=(format!("/static/{code}/{slug}_thumbnail_80.jpg"))
                                    srcSet=(format!("/static/{code}/{slug}_thumbnail_50.jpg 50w, /static/{code}/{slug}_thumbnail_80.jpg 80w"))
                                    sizes="(min-width: 1000px) 80px, 50px"
                                    role="presentation";
                            }
                        }
                        (cfg.name)
                    }
                } @else {
                    (cfg.name)
                }
            }
            td {
                @if submissions.len() > 0 {
                    a href={ "/cartridges/" (cfg.rom_id) } { (cfg.rom_id) }
                } @else {
                    (cfg.rom_id)
                }
            }
            td { (multiline(years)) }
            td { (multiline(releases)) }
            td {
                @for board_type in board_types {
                    (board_type)
                    br;
                }
            }
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
