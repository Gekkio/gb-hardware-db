// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, PreEscaped, Render};
use pulldown_cmark::{html::push_html, Parser};

pub struct Markdown {
    html: String,
}

impl Markdown {
    pub fn parse(markdown: &str) -> Self {
        let parser = Parser::new(markdown);
        let mut html = String::new();
        push_html(&mut html, parser);
        Markdown { html }
    }
}

impl Render for Markdown {
    fn render(&self) -> Markup {
        html! {
            (PreEscaped(&self.html))
        }
    }
}
