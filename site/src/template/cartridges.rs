// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use super::markdown::Markdown;

pub struct Cartridges {
    pub content: Markdown,
}

impl Render for Cartridges {
    fn render(&self) -> Markup {
        html! {
            article {
                (self.content)
                h3 { "Data dumps" }
                a href="/static/export/cartridges.csv" { "UTF-8 encoded CSV" }
            }
        }
    }
}
