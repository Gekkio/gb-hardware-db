// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::ChipRoleConfig;
use maud::{html, Markup, Render};
use time::{format_description::FormatItem, macros::format_description};

use crate::legacy::{HasDateCode, LegacyCartridgeSubmission, LegacyChip, LegacyPhoto};

pub struct CartridgePage<'a> {
    pub submission: &'a LegacyCartridgeSubmission,
}

impl<'a> CartridgePage<'a> {
    pub fn new(submission: &'a LegacyCartridgeSubmission) -> Self {
        CartridgePage { submission }
    }
    pub fn render_photo(&self, photo: &LegacyPhoto) -> Markup {
        let url = format!(
            "/static/{code}/{slug}_{name}",
            code = self.submission.code,
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

static DATE_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]");

impl<'a> Render for CartridgePage<'a> {
    fn render(&self) -> Markup {
        let metadata = &self.submission.metadata;
        let photos = &self.submission.photos;
        let board = &metadata.board;
        html! {
            article.page-cartridge {
                h2 { (metadata.cfg.name) ": " (self.submission.title) " [" (self.submission.contributor) "]" }
                div.page-cartridge__photo {
                    @if let Some(photo) = &photos.front {
                        (self.render_photo(photo))
                    }
                    @if let Some(photo) = &photos.back {
                        (self.render_photo(photo))
                    }
                }
                dl {
                    @if let Some(value) = &metadata.code {
                        dt { "Release" }
                        dd { (value) }
                    }
                    @if let Some(value) = &metadata.stamp {
                        dt { "Stamp on case" }
                        dd { (value) }
                    }
                }
                h3 { "Board" }
                div.page-cartridge__photo {
                    @if let Some(photo) = &photos.pcb_front {
                        (self.render_photo(photo))
                    }
                    @if let Some(photo) = &photos.pcb_back {
                        (self.render_photo(photo))
                    }
                }
                dl {
                    dt { "Board type" }
                    dd { (board.kind) }
                    @if let Some(date) = board.date_code().calendar() {
                        dt { "Manufacture date" }
                        dd { (date) }
                    }
                    @if let Some(value) = &board.circled_letters {
                        dt { "Circled letter(s) on board" }
                        dd { (value) }
                    }
                    @if let Some(value) = &board.extra_label {
                        dt { "Extra label" }
                        dd { (value) }
                    }
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
                    @for (designator, role) in &ChipRoleConfig::from(board.layout) {
                        (render_chip(designator.as_str(), role.display(), metadata.board[designator].as_ref()))
                    }
                }
                @if let Some(dump) = &metadata.dump {
                    div {
                        h3 { "ROM dump" }
                        dl {
                            dt { "Used tool" }
                            dd { (dump.tool) }
                            dt { "Dump date" }
                            dd { (dump.date.format(DATE_FORMAT).unwrap_or_default()) }
                            dt { "SHA256" }
                            dd { (dump.sha256) }
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
