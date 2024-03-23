// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::{ChipRoleConfig, GameConfig};
use maud::{html, Markup, Render};

use super::{
    listing_chip::ListingChip, listing_entry_cell::ListingEntryCell,
    listing_photos_cell::ListingPhotosCell,
};
use crate::legacy::{HasDateCode, LegacyCartridgeSubmission};

#[derive(Clone, Debug)]
pub struct Game<'a> {
    pub cfg: &'a GameConfig,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
}

impl<'a> Render for Game<'a> {
    fn render(&self) -> Markup {
        let layout = self.cfg.layouts[0];
        let chips = ChipRoleConfig::from(layout);
        html! {
            article {
                h2 { (self.cfg.name) }
                table {
                    thead {
                        tr {
                            th { "Entry" }
                            th { "Release" }
                            th { "Board" }
                            @for (designator, role) in &chips {
                                th { (role.display()) " (" (designator.as_str()) ")" }
                            }
                            th {"Photos" }
                        }
                    }
                    tbody {
                        @for submission in &self.submissions {
                            (render_submission(submission, &chips))
                        }
                    }
                }
            }
        }
    }
}

fn render_submission(submission: &LegacyCartridgeSubmission, chips: &ChipRoleConfig) -> Markup {
    let metadata = &submission.metadata;
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.title,
                secondary_texts: &[],
                submission,
            })
            td { (metadata.code.as_deref().unwrap_or_default()) }
            td {
                div { (metadata.board.kind) }
                div { (metadata.board.date_code().calendar().unwrap_or_default()) }
            }
            @for (designator, _) in chips {
                (ListingChip {
                    chip: metadata.board[designator].as_ref(),
                    hide_type: false,
                })
            }
            (ListingPhotosCell { submission })
        }
    }
}
