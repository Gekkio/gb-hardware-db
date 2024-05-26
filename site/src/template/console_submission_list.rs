// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use crate::{
    legacy::{
        console::{ChipInfo, LegacyConsoleMetadata},
        HasDateCode, LegacyPhotos, LegacySubmission,
    },
    template::listing_part::ListingPart,
    template::{listing_entry_cell::ListingEntryCell, listing_photos_cell::ListingPhotosCell},
};

pub struct ConsoleSubmissionList<'a, M, P> {
    pub submissions: &'a [LegacySubmission<M, P>],
    pub board_column_name: &'static str,
    pub render_console_column: bool,
    pub extra_columns: &'static [&'static str],
    pub extra_cells: Vec<Box<dyn Fn(&M) -> Markup>>,
}

impl<'a, M, P> ConsoleSubmissionList<'a, M, P> {
    pub fn new(submissions: &'a [LegacySubmission<M, P>]) -> Self {
        ConsoleSubmissionList {
            submissions,
            board_column_name: "Board",
            render_console_column: true,
            extra_columns: &[],
            extra_cells: vec![],
        }
    }
    pub fn render_console_column(mut self, value: bool) -> Self {
        self.render_console_column = value;
        self
    }
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> Render for ConsoleSubmissionList<'a, M, P> {
    fn render(&self) -> Markup {
        let console = M::CONSOLE;
        let parts = M::parts();
        html! {
            article {
                h2 { (console.name()) " (" (console.code()) ")" }
                table {
                    thead {
                        tr {
                            th { "Submission" }
                            @if self.render_console_column {
                                th { "Console" }
                            }
                            th { (self.board_column_name) }
                            @for part in &parts {
                                th { (part.label) " (" (part.designator) ")" }
                            }
                            @for column in self.extra_columns {
                                th { (column) }
                            }
                            th { "Photos" }
                        }
                    }
                    tbody {
                        @for submission in self.submissions {
                            (Submission {
                                submission,
                                parts: &parts,
                                extra_cells: &self.extra_cells,
                                render_console_column: self.render_console_column
                            })
                        }
                    }
                }
                h3 { "Data dumps " }
                a href={ "/static/export/consoles/" (console.id()) ".csv" } { "UTF-8 encoded CSV" }
            }
        }
    }
}

struct Submission<'a, M: LegacyConsoleMetadata, P> {
    pub submission: &'a LegacySubmission<M, P>,
    pub parts: &'a [ChipInfo<M>],
    pub render_console_column: bool,
    pub extra_cells: &'a [Box<dyn Fn(&M) -> Markup>],
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> Render for Submission<'a, M, P> {
    fn render(&self) -> Markup {
        let metadata = &self.submission.metadata;
        let lcd_date_code = metadata
            .lcd_panel()
            .and_then(|panel| panel.date_code().calendar());
        html! {
            tr {
                (ListingEntryCell {
                    url_prefix: "/consoles",
                    primary_text: &self.submission.title,
                    secondary_texts: &[],
                    submission: self.submission,
                })
                @if self.render_console_column {
                    td {
                        @if let Some(color) = metadata.shell().color {
                            div { "Color: " (color) }
                        }
                        @if let Some(release_code) = metadata.shell().release_code {
                            div { "Release: " (release_code) }
                        }
                        @if let Some(date_code) = metadata.shell().date_code.calendar() {
                            div { "Assembled: " (date_code) }
                        }
                        @if let Some(date_code) = lcd_date_code {
                            div { "LCD panel: " (date_code) }
                        }
                    }
                }
                td {
                    div { (metadata.mainboard().kind) }
                    @if let Some(date_code) = metadata.mainboard().date_code.calendar() {
                        div { (date_code) }
                    }
                }
                @for part in self.parts {
                    (ListingPart {
                        part: (part.getter)(metadata),
                        hide_type: part.hide_type,
                    })
                }
                @for cell in self.extra_cells {
                    td { (cell(&metadata)) }
                }
                (ListingPhotosCell { submission: self.submission })
            }
        }
    }
}
