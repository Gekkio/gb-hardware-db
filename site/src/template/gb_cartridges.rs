// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{
    config::cartridge::{GameConfig, GamePlatform, PartRole, PartRoleConfig},
    CartridgeClass,
};
use itertools::Itertools;
use maud::{html, Markup, Render};
use std::{borrow::Cow, collections::BTreeMap};

use crate::{legacy::LegacyCartridgeSubmission, template::mapper::MapperCfg};

pub struct GbCartridges<'a> {
    pub mapper_cfgs: &'a [MapperCfg],
    pub cfgs: &'a BTreeMap<String, GameConfig>,
    pub submissions: &'a [LegacyCartridgeSubmission],
}

impl<'a> Render for GbCartridges<'a> {
    fn render(&self) -> Markup {
        let mut per_game = Vec::new();
        for (code, group) in &self
            .submissions
            .iter()
            .sorted_by_key(|submission| &submission.code)
            .group_by(|submission| &submission.code)
        {
            let cfg = &self.cfgs[code];
            if CartridgeClass::from(cfg.platform) == CartridgeClass::Gb {
                per_game.push((cfg, group.collect::<Vec<_>>()));
            }
        }
        per_game.sort_by_key(|(cfg, _)| &cfg.name);
        html! {
            article {
                h2 { "Game Boy cartridges" }
                h3 { "Cartridges by mapper" }
                ul.cartridges__mapper-list {
                    @for cfg in self.mapper_cfgs {
                        li {
                            a href={ "/cartridges/" (cfg.id) ".html" } { (cfg.name) }
                        }
                    }
                }
                h3 { "Cartridges by game" }
                table {
                    thead {
                        tr {
                            th { "Title" }
                            th { "ROM ID" }
                            th { "Year(s)" }
                            th { "Release(s)" }
                            th { "Board type(s)" }
                            th { "Mapper(s)" }
                            th { "Submissions" }
                        }
                    }
                    tbody.divider {
                        tr {
                            th colspan="7" { "Game Boy" }
                        }
                    }
                    tbody {
                        @for (cfg, submissions) in &per_game {
                            @if cfg.platform == GamePlatform::Gb {
                                (render_game(cfg, submissions))
                            }
                        }
                    }
                    tbody.divider {
                        tr {
                            th colspan="7" { "Game Boy Color" }
                        }
                    }
                    tbody {
                        @for (cfg, submissions) in &per_game {
                            @if cfg.platform == GamePlatform::Gbc {
                                (render_game(cfg, submissions))
                            }
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
        let roles = PartRoleConfig::from(submission.metadata.board.layout);
        let parts = roles
            .into_iter()
            .find(|&(_, role)| role == PartRole::Mapper)
            .and_then(|(designator, _)| submission.metadata.board.parts.get(&designator));
        parts.and_then(|part| part.kind.as_deref().map(Cow::Borrowed))
    });
    html! {
        tr {
            td.submission-list-item {
                a.submission-list-item__link href={ "/cartridges/" (cfg.rom_id) } { (cfg.name) }
            }
            td { (cfg.rom_id) }
            td { (multiline(years)) }
            td { (multiline(releases)) }
            td { (multiline(board_types)) }
            td { (multiline(mappers)) }
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
