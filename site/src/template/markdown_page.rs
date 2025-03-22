// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{Markup, Render, html};

use super::markdown::Markdown;

pub struct MarkdownPage {
    pub markdown: Markdown,
}

impl Render for MarkdownPage {
    fn render(&self) -> Markup {
        html! {
            article {
                (self.markdown)
            }
        }
    }
}
