// SPDX-FileCopyrightText: 2017-2025 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::{BoardConfig, GameConfig};
use maud::{Markup, Render, html};

use crate::{
    legacy::LegacyCartridgeSubmission,
    template::{
        Optional, listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
        listing_photos_cell::ListingPhotosCell,
    },
};

pub struct CartridgeBoardPage<'a> {
    pub cfg: &'a BoardConfig,
    pub submissions: Vec<(&'a GameConfig, Vec<&'a LegacyCartridgeSubmission>)>,
}

impl<'a> Render for CartridgeBoardPage<'a> {
    fn render(&self) -> Markup {
        html! {
            article.cartridge-board-page {
                h2 { "Cartridges by board type: " (self.cfg.label()) }
                table {
                    colgroup {
                        col;
                        col;
                        col;
                        @for _ in self.cfg.parts() {
                            col;
                        }
                        col;
                    }
                    thead {
                        tr {
                            th scope="col" { "Entry" }
                            th scope="col" { "Release" }
                            th scope="col" { "Board" }
                            @for (designator, part) in self.cfg.parts() {
                                th scope="col" { (part.role().display()) " (" (designator.as_str()) ")" }
                            }
                            th scope="col" { "Photos" }
                        }
                    }
                    @for (game, chunk) in &self.submissions {
                        tbody.mapper-page__game {
                            tr.mapper-page__game-header {
                                th colspan=(self.cfg.parts().count() + 4) scope="rowgroup" {
                                    a href={ ("/cartridges/") (game.rom_id) } {
                                        div.mapper-page__game-name { (game.name) }
                                        aside { (game.rom_id) }
                                    }
                                }
                            }
                            @for submission in chunk {
                                (render_submission(&self.cfg, submission))
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_submission(cfg: &BoardConfig, submission: &LegacyCartridgeSubmission) -> Markup {
    let metadata = &submission.metadata;
    let board = &metadata.board;
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.title,
                secondary_texts: &[&submission.contributor],
                submission,
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
            @for (designator, _) in cfg.parts() {
                (ListingPart {
                    part: board.parts.get(&designator),
                    hide_type: false,
                })
            }
            (ListingPhotosCell { submission })
        }
    }
}
