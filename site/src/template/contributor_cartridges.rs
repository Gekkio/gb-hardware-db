use gbhwdb_backend::config::cartridge::GamePlatform;
use itertools::Itertools;
use maud::{html, Markup, Render};
use std::{borrow::Cow, collections::HashMap};

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
                @if by_platform.contains_key(&GamePlatform::Gb) || by_platform.contains_key(&GamePlatform::Gbc) {
                    table {
                        thead {
                            tr {
                                th { "Entry" }
                                th { "ROM ID" }
                                th { "Year" }
                                th { "Release" }
                                th { "Board type" }
                                th { "Mapper" }
                            }
                        }
                        @if let Some(submissions) = by_platform.get(&GamePlatform::Gb) {
                            tbody.divider {
                                tr {
                                    th colspan="6" { "Game Boy" }
                                }
                            }
                            tbody {
                                @for submission in submissions {
                                    (render_gb_submission(submission))
                                }
                            }
                        }
                        @if let Some(submissions) = by_platform.get(&GamePlatform::Gbc) {
                            tbody.divider {
                                tr {
                                    th colspan="6" { "Game Boy Color" }
                                }
                            }
                            tbody {
                                @for submission in submissions {
                                    (render_gb_submission(submission))
                                }
                            }
                        }
                    }
                }
                @if let Some(submissions) = by_platform.get(&GamePlatform::Gba) {
                    h2 { "Game Boy Advance cartridges" }
                    table {
                        thead {
                            tr {
                                th { "Entry" }
                                th { "ROM ID" }
                                th { "Year" }
                                th { "Release" }
                                th { "Board type" }
                            }
                        }
                        tbody {
                            @for submission in submissions {
                                (render_gba_submission(submission))
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_gb_submission(submission: &LegacyCartridgeSubmission) -> Markup {
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
            td { (Optional(mapper)) }
        }
    }
}

fn render_gba_submission(submission: &LegacyCartridgeSubmission) -> Markup {
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
        }
    }
}
