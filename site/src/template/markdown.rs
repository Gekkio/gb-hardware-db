// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use percy_dom::IterableNodes;
use pulldown_cmark::{html::push_html, Parser};

use super::raw_html::parse_html_fragment;

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
    pub fn render(&self) -> IterableNodes {
        parse_html_fragment(&self.html)
    }
}
