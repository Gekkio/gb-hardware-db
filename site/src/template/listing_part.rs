// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{Markup, Render, html};

use crate::{process::part::ProcessedPart, template::Optional};

pub struct ListingPart<'a> {
    pub part: Option<&'a ProcessedPart>,
    pub hide_type: bool,
}

impl<'a> Render for ListingPart<'a> {
    fn render(&self) -> Markup {
        match self.part {
            None => html! { td; },
            Some(part) => html! {
                td.listing-part {
                    @if !self.hide_type {
                        div { (Optional(part.kind.as_ref())) }
                    }
                    div { (Optional(part.rom_id.as_ref())) }
                    div { (Optional(part.date_code.calendar())) }
                    div { (Optional(part.manufacturer.as_ref().map(|m| m.name()))) }
                }
            },
        }
    }
}
