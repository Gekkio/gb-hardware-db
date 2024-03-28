// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::{GameConfig, PartRoleConfig};
use maud::{html, Markup, Render};

use crate::{
    legacy::LegacyCartridgeSubmission,
    template::Optional,
    template::{
        listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
        listing_photos_cell::ListingPhotosCell,
    },
};

#[derive(Clone, Debug)]
pub struct Game<'a> {
    pub cfg: &'a GameConfig,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
}

impl<'a> Render for Game<'a> {
    fn render(&self) -> Markup {
        let layout = self.cfg.layouts[0];
        let parts = PartRoleConfig::from(layout);
        html! {
            article {
                h2 { (self.cfg.name) }
                table {
                    thead {
                        tr {
                            th { "Entry" }
                            th { "Release" }
                            th { "Board" }
                            @for (designator, role) in &parts {
                                th { (role.display()) " (" (designator.as_str()) ")" }
                            }
                            th {"Photos" }
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

fn render_submission(submission: &LegacyCartridgeSubmission, parts: &PartRoleConfig) -> Markup {
    let metadata = &submission.metadata;
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.title,
                secondary_texts: &[],
                submission,
            })
            td { (Optional(metadata.code.as_ref())) }
            td {
                div { (metadata.board.kind) }
                div { (Optional(metadata.board.date_code.calendar())) }
            }
            @for (designator, _) in parts {
                (ListingPart {
                    part: metadata.board.parts.get(&designator),
                    hide_type: false,
                })
            }
            (ListingPhotosCell { submission })
        }
    }
}
