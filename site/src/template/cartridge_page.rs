// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};
use time::{format_description::FormatItem, macros::format_description};

use crate::{
    legacy::{LegacyCartridgeSubmission, LegacyPhoto},
    template::submission_part_table::{submission_part_table, SubmissionPart},
};

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
        let parts = board.cfg.parts().map(|(designator, part)| SubmissionPart {
            designator: designator.as_str(),
            label: part.role.display(),
            part: board.parts.get(&designator),
        });
        html! {
            article.page-cartridge {
                h2 { (metadata.cfg.name) ": " (self.submission.title) " [" (self.submission.contributor) "]" }
                div.page-cartridge__photo {
                    @if let Some(photo) = &photos.front {
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
                    @if let Some(photo) = &photos.without_battery {
                        (self.render_photo(photo))
                    }
                }
                dl {
                    dt { "Board type" }
                    dd { (board.kind) }
                    @if let Some(date) = board.date_code.calendar() {
                        dt { "Manufacture date" }
                        dd { (date) }
                    }
                    @if let Some(value) = &board.circled_letters {
                        dt { "Circled letter(s) on board" }
                        dd { (value) }
                    }
                    @if let Some(value) = &board.panel_position {
                        dt { "Position in PCB panel" }
                        dd { (value) }
                    }
                }
                h3 { "Parts" }
                (submission_part_table(parts))
                @if let Some(dump) = &metadata.dump {
                    div {
                        h3 { "ROM dump" }
                        dl {
                            dt { "Used tool" }
                            dd { (dump.tool) }
                            dt { "Dump date" }
                            dd { (dump.date.format(DATE_FORMAT).unwrap_or_default()) }
                            @if let Some(hash) = dump.crc32 {
                                dt { "CRC-32" }
                                dd {
                                    pre.page-cartridge__dump-hash { (hash) }
                                    @if let Some(cfg_hash) = metadata.cfg.crc32 {
                                        @if hash == cfg_hash {
                                            " ✅"
                                        }
                                        @else {
                                            " ❌"
                                        }
                                    }
                                }
                            }
                            @if let Some(hash) = dump.md5 {
                                dt { "MD5" }
                                dd {
                                    pre.page-cartridge__dump-hash { (hash) }
                                    @if let Some(cfg_hash) = metadata.cfg.md5 {
                                        @if hash == cfg_hash {
                                            " ✅"
                                        }
                                        @else {
                                            " ❌"
                                        }
                                    }
                                }
                            }
                            @if let Some(hash) = dump.sha1 {
                                dt { "SHA-1" }
                                dd {
                                    pre.page-cartridge__dump-hash { (hash) }
                                    @if let Some(cfg_hash) = metadata.cfg.sha1 {
                                        @if hash == cfg_hash {
                                            " ✅"
                                        }
                                        @else {
                                            " ❌"
                                        }
                                    }
                                }
                            }
                            @if let Some(hash) = dump.sha256 {
                                dt { "SHA-256" }
                                dd {
                                    pre.page-cartridge__dump-hash { (hash) }
                                    @if let Some(cfg_hash) = metadata.cfg.sha256 {
                                        @if hash == cfg_hash {
                                            " ✅"
                                        }
                                        @else {
                                            " ❌"
                                        }
                                    }
                                }
                            }
                            @if let Some(log) = &dump.log {
                                dt { "Dump log" }
                                dd {
                                    pre.page-cartridge__dump-log { (log) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
