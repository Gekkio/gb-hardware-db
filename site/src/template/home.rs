// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use percy_dom::{html, IterableNodes, View, VirtualNode};
use time::{macros::format_description, Date};

use super::markdown::Markdown;

pub struct Home {
    pub content: Markdown,
    pub today: Date,
    pub console_submission_count: u32,
    pub cartridge_submission_count: u32,
}

impl View for Home {
    fn render(&self) -> VirtualNode {
        let today = self
            .today
            .format(format_description!(
                "[month repr:long] [day padding:none], [year]"
            ))
            .unwrap_or_else(|_| "?".to_string());
        let console_submission_count = self.console_submission_count;
        let cartridge_submission_count = self.cartridge_submission_count;
        html! {
            <article>
                {self.content.render()}
                {format!("Last updated: {today}")}
                <br>
                {format!("Console submission count: {console_submission_count}")}
                <br>
                {format!("Cartridge submission count: {cartridge_submission_count}")}
            </article>
        }
    }
}
