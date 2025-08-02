// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::{BoardConfig, GameConfig, PartRole};
use maud::{Markup, Render, html};

use crate::{
    legacy::LegacyCartridgeSubmission,
    process::part::ProcessedPart,
    template::{
        Optional, listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
        listing_photos_cell::ListingPhotosCell,
    },
};

pub struct MapperCfg {
    pub id: &'static str,
    pub name: &'static str,
    pub parts: &'static [PartRole],
    pub match_fn: Box<dyn Fn(BoardConfig, Option<&ProcessedPart>) -> bool + Send + Sync>,
}

pub struct MapperPage<'a> {
    pub cfg: &'a MapperCfg,
    pub submissions: Vec<(&'a GameConfig, Vec<&'a LegacyCartridgeSubmission>)>,
}

impl<'a> Render for MapperPage<'a> {
    fn render(&self) -> Markup {
        html! {
            article.mapper-page {
                h2 { "Cartridges by mapper: " (self.cfg.name) }
                table {
                    colgroup {
                        col;
                        col;
                        col;
                        @for _ in self.cfg.parts {
                            col;
                        }
                        col;
                    }
                    thead {
                        tr {
                            th scope="col" { "Entry" }
                            th scope="col" { "Release" }
                            th scope="col" { "Board" }
                            @for role in self.cfg.parts {
                                th scope="col" { (role.display()) }
                            }
                            th scope="col" { "Photos" }
                        }
                    }
                    @for (game, chunk) in &self.submissions {
                        tbody.mapper-page__game {
                            tr.mapper-page__game-header {
                                th colspan=(self.cfg.parts.len() + 4) scope="rowgroup" {
                                    a href={ ("/cartridges/") (game.rom_id) } {
                                        div.mapper-page__game-name { (game.name) }
                                        aside { (game.rom_id) }
                                    }
                                }
                            }
                            @for submission in chunk {
                                (render_submission(self.cfg, submission))
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_submission(cfg: &MapperCfg, submission: &LegacyCartridgeSubmission) -> Markup {
    let metadata = &submission.metadata;
    let board = &metadata.board;
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.title,
                secondary_texts: &[],
                submission,
                show_contributor: true,
            })
            td {
                div { (Optional(metadata.code.as_ref())) }
                @if let Some(stamp) = &metadata.stamp {
                    div { "Stamp: " (stamp) }
                }
            }
            td {
                div { (board.kind) }
                div { (Optional(board.date_code.calendar())) }
            }
            @for &role in cfg.parts {
                @let part = board.cfg.parts().find(|(_, candidate)| candidate.role() == role);
                @if let Some((designator, _)) = part {
                    (ListingPart {
                        part: board.parts.get(&designator),
                        hide_type: false,
                    })
                }
                @else {
                    td.listing-part.listing-part--not-applicable {
                        "N/A"
                    }
                }
            }
            (ListingPhotosCell { submission })
        }
    }
}
