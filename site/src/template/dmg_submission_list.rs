// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use super::console_submission_list::ConsoleSubmissionList;
use crate::{
    legacy::{console::LegacyDmgMetadata, HasDateCode, LegacyDmgSubmission},
    template::Optional,
};

pub struct DmgSubmissionList<'a> {
    pub submissions: &'a [LegacyDmgSubmission],
}

impl<'a> DmgSubmissionList<'a> {
    pub fn new(submissions: &'a [LegacyDmgSubmission]) -> Self {
        DmgSubmissionList { submissions }
    }
}

impl<'a> Render for DmgSubmissionList<'a> {
    fn render(&self) -> Markup {
        ConsoleSubmissionList {
            submissions: self.submissions,
            board_column_name: "Mainboard",
            render_console_column: true,
            extra_columns: &["LCD board", "Power board", "Jack board"],
            extra_cells: vec![
                Box::new(|m: &LegacyDmgMetadata| {
                    html! {
                        @if let Some(board) = &m.lcd_board {
                            div {
                                div { (board.kind) }
                                div { (Optional(board.date_code().calendar_short())) }
                            }
                        }
                    }
                }),
                Box::new(|m: &LegacyDmgMetadata| {
                    html! {
                        @if let Some(board) = &m.power_board {
                            div {
                                div { "Type " (board.kind) }
                                div { (Optional(board.date_code().calendar_short())) }
                            }
                        }
                    }
                }),
                Box::new(|m: &LegacyDmgMetadata| {
                    html! {
                        @if let Some(board) = &m.jack_board {
                            (board.kind.as_str())
                        }
                    }
                }),
            ],
        }
        .render()
    }
}
