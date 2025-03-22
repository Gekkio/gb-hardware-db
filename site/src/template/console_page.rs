// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{Markup, Render, html};

use crate::{
    legacy::{
        HasDateCode, LegacyPhoto, LegacyPhotos, LegacySubmission, PhotoInfo, PhotoKind,
        console::LegacyConsoleMetadata,
    },
    template::{
        Optional,
        submission_part_table::{SubmissionPart, submission_part_table},
    },
};

pub struct ConsolePage<'a, M, P> {
    pub submission: &'a LegacySubmission<M, P>,
    pub extra_sections: Vec<Box<dyn Fn(&Self, &M) -> Markup>>,
    pub extra_parts: Vec<Box<dyn Fn(&M) -> SubmissionPart>>,
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> ConsolePage<'a, M, P> {
    pub fn new(submission: &'a LegacySubmission<M, P>) -> Self {
        ConsolePage {
            submission,
            extra_sections: Vec::new(),
            extra_parts: Vec::new(),
        }
    }
    fn render_photo_info(&self, photo: &PhotoInfo<P>) -> Option<Markup> {
        (photo.getter)(&self.submission.photos).map(|photo| self.render_photo(photo))
    }
    pub fn render_photo(&self, photo: &LegacyPhoto) -> Markup {
        let url = format!(
            "/static/{console}/{slug}_{name}",
            console = M::CONSOLE.id(),
            slug = self.submission.slug,
            name = photo.name
        );
        html! {
            a href=(url) {
                img src=(url);
            }
        }
    }
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> Render for ConsolePage<'a, M, P> {
    fn render(&self) -> Markup {
        let metadata = &self.submission.metadata;
        let mainboard = metadata.mainboard();
        let parts = M::parts()
            .into_iter()
            .map(|info| {
                let part = (info.getter)(&self.submission.metadata);
                SubmissionPart {
                    designator: info.designator,
                    label: info.label,
                    part,
                }
            })
            .chain(metadata.lcd_panel().into_iter().flat_map(|panel| {
                [
                    SubmissionPart {
                        designator: "-",
                        label: "LCD column driver",
                        part: panel.column_driver.as_ref(),
                    },
                    SubmissionPart {
                        designator: "-",
                        label: "LCD row driver",
                        part: panel.row_driver.as_ref(),
                    },
                ]
                .into_iter()
            }))
            .chain(self.extra_parts.iter().map(|f| f(metadata)));
        html! {
            article class=(format!("page-console page-console--{console}", console = M::CONSOLE.id())) {
                h2 { (M::CONSOLE.code()) ": " (self.submission.title) " [" (self.submission.contributor) "]" }
                div.page-console__photo {
                    @for info in P::infos() {
                        @if info.kind == PhotoKind::MainUnit {
                            (Optional(self.render_photo_info(&info)))
                        }
                    }
                }
                dl {
                    @if let Some(color) = metadata.shell().color {
                        dt { "Color" }
                        dd { (color) }
                    }
                    @if let Some(release_code) = metadata.shell().release_code {
                        dt { "Release code" }
                        dd { (release_code) }
                    }
                    @if let Some(assembled) = metadata.shell().date_code.calendar() {
                        dt { "Assembly date" }
                        dd { (assembled) }
                    }
                    @if let Some(stamp) = metadata.shell().stamp {
                        dt { "Stamp on case" }
                        dd { (stamp) }
                    }
                    @if let Some(panel) = metadata.lcd_panel() {
                        @if let Some(label) = &panel.label {
                            dt { "LCD panel label" }
                            dd { (label) }
                        }
                        @if let Some(date) = panel.date_code().calendar() {
                            dt { "LCD panel date" }
                            dd { (date) }
                        }
                    }
                }
                h3 { "Mainboard" }
                div.page-console__photo {
                    @for info in P::infos() {
                        @if info.kind == PhotoKind::MainBoard {
                            (Optional(self.render_photo_info(&info)))
                        }
                    }
                }
                dl {
                    dt { "Board type" }
                    dd { (mainboard.kind) }
                    @if let Some(date) = mainboard.date_code.calendar() {
                        dt { "Manufacture date" }
                        dd { (date) }
                    }
                    @if let Some(value) = mainboard.number_pair {
                        dt { "Number pair on board" }
                        dd { (value) }
                    }
                    @if let Some(value) = mainboard.stamp {
                        dt { "Stamp on board" }
                        dd { (value) }
                    }
                    @if let Some(value) = mainboard.stamp_front {
                        dt { "Secondary stamp on board (front)" }
                        dd { (value) }
                    }
                    @if let Some(value) = mainboard.stamp_back {
                        dt { "Secondary stamp on board (back)" }
                        dd { (value) }
                    }
                    @if let Some(value) = mainboard.circled_letters {
                        dt { "Circled letter(s) on board" }
                        dd { (value) }
                    }
                    @if let Some(value) = mainboard.letter_at_top_right {
                        dt { "Letter at top right" }
                        dd { (value) }
                    }
                    @if let Some(value) = mainboard.extra_label {
                        dt { "Extra label" }
                        dd { (value) }
                    }
                }
                @for section in &self.extra_sections {
                    ((section)(self, metadata))
                }
                h3 { "Parts" }
                (submission_part_table(parts))
            }
        }
    }
}
