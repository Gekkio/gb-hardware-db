// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::{GameConfig, GamePlatform, PartDesignator, PartRole};
use maud::{Markup, Render, html};
use std::collections::BTreeSet;

use crate::{
    legacy::LegacyCartridgeSubmission,
    template::Optional,
    template::{
        listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
        listing_photos_cell::ListingPhotosCell,
    },
};

#[derive(Clone, Debug)]
pub struct CartridgesByGame<'a> {
    pub cfg: &'a GameConfig,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
}

impl<'a> Render for CartridgesByGame<'a> {
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
            article {
                h2 { (self.cfg.name) }
                @if let Some(href) = no_intro_href {
                    a.external href=(href) {
                        "Game entry in No-Intro Dat-o-Matic"
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
