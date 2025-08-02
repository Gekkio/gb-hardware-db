// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::{GameConfig, GamePlatform, PartDesignator, PartRole};
use maud::{Markup, Render, html};
use std::collections::BTreeSet;

use crate::{
    legacy::LegacyCartridgeSubmission,
    site::board_kind_link,
    template::{
        Optional, listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
        listing_photos_cell::ListingPhotosCell,
    },
};

#[derive(Clone, Debug)]
pub struct GamePage<'a> {
    pub cfg: &'a GameConfig,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
    pub variants: &'a [(&'a GameConfig, bool)],
}

impl<'a> Render for GamePage<'a> {
    fn render(&self) -> Markup {
        let parts = self
            .submissions
            .iter()
            .flat_map(|s| {
                s.metadata
                    .board
                    .cfg
                    .parts()
                    .map(|(designator, part)| (designator, part.role()))
            })
            .collect::<BTreeSet<_>>();
        let no_intro_href = if self.cfg.no_intro_id.is_empty() {
            None
        } else {
            Some(format!(
                "https://datomatic.no-intro.org/index.php?page=show_record&s={system}&n={entry}",
                system = match self.cfg.platform {
                    GamePlatform::Gb => "46",
                    GamePlatform::Gbc => "47",
                    GamePlatform::Gba => "23",
                },
                entry = urlencoding::encode(&self.cfg.no_intro_id),
            ))
        };
        html! {
            article.game-page {
                h2 { (self.cfg.name) }
                @if let Some(href) = no_intro_href {
                    a.external href=(href) {
                        "Game entry in No-Intro Dat-o-Matic"
                    }
                }
                @if self.variants.len() > 1 {
                    h3 { "Variants" }
                    ul.game-page__variants {
                        @for &(variant, link) in self.variants {
                            li.game-page__variant .game-page__variant--selected[variant.rom_id == self.cfg.rom_id] {
                                @if link {
                                    a href={ "/cartridges/" (variant.rom_id) } {
                                        div { (variant.name) }
                                        div { (variant.rom_id) }
                                    }
                                } @else {
                                    div { (variant.name) }
                                    div { (variant.rom_id) }
                                }
                            }
                        }
                    }
                }
                table {
                    thead {
                        tr {
                            th scope="col" { "Entry" }
                            th scope="col" { "Release" }
                            th scope="col" { "Board" }
                            @for (designator, role) in &parts {
                                th scope="col" { (role.display()) " (" (designator.as_str()) ")" }
                            }
                            th scope="col" {"Photos" }
                        }
                    }
                    tbody {
                        @for submission in &self.submissions {
                            (render_submission(submission, &parts))
                        }
                    }
                }
            }
        }
    }
}

fn render_submission(
    submission: &LegacyCartridgeSubmission,
    parts: &BTreeSet<(PartDesignator, PartRole)>,
) -> Markup {
    let metadata = &submission.metadata;
    let board = &metadata.board;
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.contributor,
                secondary_texts: &[&submission.title],
                submission,
            })
            td {
                div { (Optional(metadata.code.as_ref())) }
                @if let Some(stamp) = &metadata.stamp {
                    div { "Stamp: " (stamp) }
                }
            }
            td {
                div { (board_kind_link(board)) }
                div { (Optional(board.date_code.calendar())) }
            }
            @for &(designator, role) in parts {
                @if board.cfg.part(designator).map(|p| p.role()) == Some(role) {
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
