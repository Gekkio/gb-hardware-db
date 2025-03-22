// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{Markup, Render, html};
use time::{Date, macros::format_description};

use super::markdown::Markdown;

pub struct Home {
    pub content: Markdown,
    pub today: Date,
    pub console_submission_count: u32,
    pub cartridge_submission_count: u32,
}

impl Render for Home {
    fn render(&self) -> Markup {
        let today = self
            .today
            .format(format_description!(
                "[month repr:long] [day padding:none], [year]"
            ))
            .unwrap_or_else(|_| "?".to_string());
        html! {
            article {
                (self.content)
                "Last updated: " (today)
                br;
                "Console submission count: " (self.console_submission_count)
                br;
                "Cartridge submission count: " (self.cartridge_submission_count)
            }
        }
    }
}
