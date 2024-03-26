// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use super::{console_page::ConsolePage, submission_part_table::SubmissionPart};
use crate::legacy::{console::LegacyDmgMetadata, HasDateCode, LegacyDmgSubmission};

pub struct DmgConsolePage<'a> {
    pub submission: &'a LegacyDmgSubmission,
}

impl<'a> DmgConsolePage<'a> {
    pub fn new(submission: &'a LegacyDmgSubmission) -> Self {
        DmgConsolePage { submission }
    }
}

impl<'a> Render for DmgConsolePage<'a> {
    fn render(&self) -> Markup {
        ConsolePage {
            submission: self.submission,
            extra_sections: vec![Box::new(|page, m: &LegacyDmgMetadata| {
                html! {
                    div {
                        h3 { "LCD board" }
                        div.page-console__photo {
                            @if let Some(photo) = &page.submission.photos.lcd_board_front {
                                (page.render_photo(photo))
                            }
                            @if let Some(photo) = &page.submission.photos.lcd_board_back {
                                (page.render_photo(photo))
                            }
                        }
                        @if let Some(board) = &m.lcd_board {
                            dl {
                                dt { "Board type" }
                                dd { (board.kind) }
                                @if let Some(date) = board.date_code().calendar() {
                                    dt { "Manufacture date" }
                                    dd { (date) }
                                }
                                @if let Some(value) = &board.stamp {
                                    dt { "Stamp" }
                                    dd { (value) }
                                }
                                @if let Some(value) = &board.circled_letters {
                                    dt { "Circled letter(s) on board" }
                                    dd { (value) }
                                }
                            }
                        }
                        h3 { "Power board" }
                        div.page-console__photo {
                            @if let Some(photo) = &page.submission.photos.power_board_front {
                                (page.render_photo(photo))
                            }
                            @if let Some(photo) = &page.submission.photos.power_board_back {
                                (page.render_photo(photo))
                            }
                        }
                        @if let Some(board) = &m.power_board {
                            dl {
                                dt { "Board type" }
                                dd { (board.kind) }
                                @if let Some(date) = board.date_code().calendar() {
                                    dt { "Manufacture date" }
                                    dd { (date) }
                                }
                                dt { "Label" }
                                dd { (board.label) }
                            }
                        }
                        h3 { "Jack board" }
                        div.page-console__photo {
                            @if let Some(photo) = &page.submission.photos.jack_board_front {
                                (page.render_photo(photo))
                            }
                            @if let Some(photo) = &page.submission.photos.jack_board_back {
                                (page.render_photo(photo))
                            }
                        }
                        @if let Some(board) = &m.jack_board {
                            dl {
                                dt { "Board type" }
                                dd { (board.kind) }
                                @if let Some(value) = &board.extra_label {
                                    dt { "Extra label" }
                                    dd { (value) }
                                }
                            }
                        }
                    }
                }
            })],
            extra_parts: vec![Box::new(|m: &LegacyDmgMetadata| SubmissionPart {
                designator: "-",
                label: "LCD bias generator",
                part: m
                    .lcd_board
                    .as_ref()
                    .and_then(|board| board.regulator.as_ref()),
            })],
        }
        .render()
    }
}
