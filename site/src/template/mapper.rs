// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::{BoardLayout, PartRole, PartRoleConfig};
use maud::{html, Markup, Render};

use super::{
    listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
    listing_photos_cell::ListingPhotosCell,
};
use crate::{
    legacy::{HasDateCode, LegacyCartridgeSubmission, LegacyPart},
    template::Optional,
};

pub struct MapperCfg {
    pub id: &'static str,
    pub name: &'static str,
    pub parts: &'static [PartRole],
    pub match_fn: Box<dyn Fn(BoardLayout, Option<&LegacyPart>) -> bool + Send + Sync>,
}

pub struct Mapper<'a> {
    pub cfg: &'a MapperCfg,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
}

impl<'a> Render for Mapper<'a> {
    fn render(&self) -> Markup {
        html! {
            article {
                h2 { "Cartridges by mapper: " (self.cfg.name) }
                table {
                    thead {
                        tr {
                            th { "Entry" }
                            th { "Release" }
                            th { "Board" }
                            @for role in self.cfg.parts {
                                th { (role.display()) }
                            }
                            th { "Photos" }
                        }
                    }
                    tbody {
                        @for submission in &self.submissions {
                            (render_submission(self.cfg, submission))
                        }
                    }
                }
            }
        }
    }
}

fn render_submission(cfg: &MapperCfg, submission: &LegacyCartridgeSubmission) -> Markup {
    let metadata = &submission.metadata;
    let parts = PartRoleConfig::from(submission.metadata.board.layout);
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.metadata.cfg.name,
                secondary_texts: &[&submission.code, &submission.title],
                submission,
            })
            td {
                (Optional(metadata.code.as_ref()))
            }
            td {
                div { (metadata.board.kind) }
                div { (Optional(metadata.board.date_code().calendar())) }
            }
            @for &role in cfg.parts {
                @let part = parts.into_iter().find(|&(_, candidate)| candidate == role)
                    .and_then(|(designator, _)| submission.metadata.board[designator].as_ref());
                (ListingPart { part, hide_type: false })
            }
            (ListingPhotosCell { submission })
        }
    }
}
