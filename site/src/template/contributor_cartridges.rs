// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::config::cartridge::GamePlatform;
use itertools::Itertools;
use maud::{html, Markup, Render};
use std::borrow::Cow;

use crate::{
    template::{listing_entry_cell::ListingEntryCell, Optional},
    LegacyCartridgeSubmission, Submissions,
};

pub struct ContributorCartridges<'a> {
    pub contributor: &'a str,
    pub submissions: &'a Submissions,
}

impl<'a> Render for ContributorCartridges<'a> {
    fn render(&self) -> Markup {
        let mut by_platform = self
            .submissions
            .cartridges
            .iter()
            .into_group_map_by(|s| s.metadata.cfg.platform);

        for submissions in by_platform.values_mut() {
            submissions.sort_by_key(|s| (&s.metadata.cfg.name, &s.slug));
        }
        html! {
            article {
                h2 { "Cartridge submissions by " (self.contributor) }
                @for platform in GamePlatform::ALL {
                    @if let Some(submissions) = by_platform.get(&platform) {
                        h3 { (platform.name()) " cartridges" }
                        table {
                            thead {
                                tr {
                                    th { "Entry" }
                                    th { "ROM ID" }
                                    th { "Year" }
                                    th { "Release" }
                                    th { "Board type" }
                                    @if platform.has_mappers() {
                                        th { "Mapper" }
                                    }
                                }
                            }
                            tbody {
                                @for submission in submissions {
                                    (render_submission(submission, platform.has_mappers()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_submission(submission: &LegacyCartridgeSubmission, show_mapper: bool) -> Markup {
    let mapper = submission
        .metadata
        .board
        .mapper()
        .and_then(|part| part.kind.as_deref().map(Cow::Borrowed));
    html! {
        tr {
            (ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.metadata.cfg.name,
                secondary_texts: &[&submission.title],
                submission,
                show_contributor: false,
            })
            td { (submission.metadata.cfg.rom_id) }
            td { (Optional(submission.metadata.board.date_code.year.as_ref())) }
            td { (Optional(submission.metadata.code.as_ref())) }
            td { (submission.metadata.board.kind) }
            @if show_mapper {
                td { (Optional(mapper)) }
            }
        }
    }
}
