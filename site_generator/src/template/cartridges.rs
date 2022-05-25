use gbhwdb_backend::config::cartridge::{ChipRole, ChipRoleConfig, GameConfig, GamePlatform};
use itertools::Itertools;
use percy_dom::{html, IterableNodes, View, VirtualNode};
use std::{borrow::Cow, collections::BTreeMap};

use super::mapper::MapperCfg;
use crate::legacy::LegacyCartridgeSubmission;

pub struct Cartridges<'a> {
    pub mapper_cfgs: &'a [MapperCfg],
    pub cfgs: &'a BTreeMap<String, GameConfig>,
    pub submissions: &'a [LegacyCartridgeSubmission],
}

impl<'a> View for Cartridges<'a> {
    fn render(&self) -> VirtualNode {
        let mut per_game = Vec::new();
        for (code, group) in &self
            .submissions
            .iter()
            .sorted_by_key(|submission| &submission.code)
            .group_by(|submission| &submission.code)
        {
            let cfg = &self.cfgs[code];
            per_game.push((cfg, group.collect::<Vec<_>>()));
        }
        per_game.sort_by_key(|(cfg, _)| &cfg.name);
        html! {
            <article>
                <h2>{"Game Boy cartridges"}</h2>
                <h3>{"Cartridges by mapper"}</h3>
                <ul class="cartridges__mapper-list">
                { self.mapper_cfgs.iter().map(|cfg| html! {
                    <li>
                        <a href={format!("/cartridges/{}.html", cfg.id)}>{cfg.name}</a>
                    </li>
                }).collect::<Vec<_>>() }
                </ul>
                <h3>{"Cartridges by game"}</h3>
                <table>
                    <thead>
                        <tr>
                            <th>{"Title"}</th>
                            <th>{"ROM ID"}</th>
                            <th>{"Year(s)"}</th>
                            <th>{"Release(s)"}</th>
                            <th>{"Board type(s)"}</th>
                            <th>{"Mapper(s)"}</th>
                            <th>{"Submissions"}</th>
                        </tr>
                    </thead>
                    <tbody class="divider">
                        <tr>
                            <th colspan="7">{"Game Boy"}</th>
                        </tr>
                    </tbody>
                    <tbody>
                        { per_game.iter()
                            .filter(|(cfg, _)| cfg.platform == GamePlatform::Gb)
                            .map(|(cfg, submissions)| render_game(cfg, submissions))
                            .collect::<Vec<_>>()
                        }
                    </tbody>
                    <tbody class="divider">
                        <tr>
                            <th colspan="7">{"Game Boy Color"}</th>
                        </tr>
                    </tbody>
                    <tbody>
                        { per_game.iter()
                            .filter(|(cfg, _)| cfg.platform == GamePlatform::Gbc)
                            .map(|(cfg, submissions)| render_game(cfg, submissions))
                            .collect::<Vec<_>>()
                        }
                    </tbody>
                </table>
                <h3>{"Data dumps"}</h3>
                <a href="/static/export/cartridges.csv">{"UTF-8 encoded CSV"}</a>
            </article>
        }
    }
}

fn render_game(cfg: &GameConfig, submissions: &[&LegacyCartridgeSubmission]) -> VirtualNode {
    let years = submissions.iter().filter_map(|submission| {
        submission
            .metadata
            .board
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
        let roles = ChipRoleConfig::from(submission.metadata.board.layout);
        let chip = roles
            .iter()
            .find(|&(_, role)| role == ChipRole::Mapper)
            .and_then(|(designator, _)| submission.metadata.board[designator].as_ref());
        chip.and_then(|chip| chip.kind.as_deref().map(Cow::Borrowed))
    });
    html! {
        <tr>
            <td class="submission-list-item">
                <a class="submission-list-item__link" href={format!("/cartridges/{}", cfg.rom_id)}>{&cfg.name}</a>
            </td>
            <td>{&cfg.rom_id}</td>
            <td>{multiline(years)}</td>
            <td>{multiline(releases)}</td>
            <td>{multiline(board_types)}</td>
            <td>{multiline(mappers)}</td>
            <td>{submissions.len()}</td>
        </tr>
    }
}

fn multiline<'a>(lines: impl Iterator<Item = Cow<'a, str>>) -> IterableNodes {
    let lines = lines.unique().sorted();
    lines
        .map(|line| {
            html! {
                <span>{&*line}<br></span>
            }
        })
        .collect::<Vec<_>>()
        .into()
}
