// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use crate::legacy::{
    console::{ChipInfo, LegacyConsoleMetadata},
    HasDateCode, LegacyChip, LegacyPhoto, LegacyPhotos, LegacySubmission, PhotoInfo, PhotoKind,
};

pub struct ConsolePage<'a, M, P> {
    pub submission: &'a LegacySubmission<M, P>,
    pub extra_sections: Vec<Box<dyn Fn(&Self, &M) -> Markup>>,
    pub extra_chips: Vec<Box<dyn Fn(&M) -> (&str, &str, Option<&LegacyChip>)>>,
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> ConsolePage<'a, M, P> {
    pub fn new(submission: &'a LegacySubmission<M, P>) -> Self {
        ConsolePage {
            submission,
            extra_sections: Vec::new(),
            extra_chips: Vec::new(),
        }
    }
    fn render_chip_info(&self, info: &ChipInfo<M>) -> Markup {
        let chip = (info.getter)(&self.submission.metadata);
        render_chip(info.designator, info.label, chip)
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
        html! {
            article class=(format!("page-console page-console--{console}", console = M::CONSOLE.id())) {
                h2 { (M::CONSOLE.code()) ": " (self.submission.title) " [" (self.submission.contributor) "]" }
                div.page-console__photo {
                    @for info in P::infos() {
                        @if info.kind == PhotoKind::MainUnit {
                            (self.render_photo_info(&info).unwrap_or_default())
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
                            (self.render_photo_info(&info).unwrap_or_default())
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
                h3 { "Chips" }
                table {
                    thead {
                        tr {
                            th;
                            th { "Chip" }
                            th { "Type" }
                            th { "Manufacturer" }
                            th { "Date" }
                            th { "Label" }
                        }
                    }
                    tbody {
                        @for info in M::chips() {
                            (self.render_chip_info(&info))
                        }
                        @if let Some(panel) = metadata.lcd_panel() {
                            (render_chip("-", "LCD column driver", panel.column_driver.as_ref()))
                                (render_chip("-", "LCD row driver", panel.row_driver.as_ref()))
                        }
                        @for f in &self.extra_chips {
                            @let (designator, label, chip) = f(metadata);
                            (render_chip(designator, label, chip))
                        }
                    }
                }
            }
        }
    }
}

fn render_chip(designator: &str, label: &str, chip: Option<&LegacyChip>) -> Markup {
    html! {
        tr.console-page-chip {
            td { (designator) }
            td { (label) }
            @if let Some(chip) = chip {
                td { (chip.kind.as_deref().unwrap_or_default()) }
                td { (chip.manufacturer.as_deref().unwrap_or_default()) }
                td { (chip.date_code().calendar().unwrap_or_default()) }
                td { (chip.label.as_deref().unwrap_or_default()) }
            } @else {
                td;
                td;
                td;
                td;
            }
        }
    }
}
