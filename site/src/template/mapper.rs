// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::{BoardConfig, PartRole};
use maud::{html, Markup, Render};

use crate::{
    legacy::LegacyCartridgeSubmission,
    process::part::ProcessedPart,
    template::Optional,
    template::{
        listing_entry_cell::ListingEntryCell, listing_part::ListingPart,
        listing_photos_cell::ListingPhotosCell,
    },
};

pub struct MapperCfg {
    pub id: &'static str,
    pub name: &'static str,
    pub parts: &'static [PartRole],
    pub match_fn: Box<dyn Fn(BoardConfig, Option<&ProcessedPart>) -> bool + Send + Sync>,
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
                table.mapper-listing {
                    colgroup {
                        col.mapper-listing__first-column;
                        col;
                        col;
                        @for _ in self.cfg.parts {
                            col;
                        }
                        col;
                    }
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
    let board = &metadata.board;
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.metadata.cfg.name,
                secondary_texts: &[&submission.code, &submission.title],
                submission,
                show_contributor: true,
            })
            td {
                (Optional(metadata.code.as_ref()))
            }
            td {
                div { (board.kind) }
                div { (Optional(board.date_code.calendar())) }
            }
            @for &role in cfg.parts {
                @let part = board.cfg.parts().find(|(_, candidate)| candidate.role == role);
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
