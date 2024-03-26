// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use crate::{
    legacy::{HasDateCode, LegacyPart},
    template::Optional,
};

pub struct ListingPart<'a> {
    pub part: Option<&'a LegacyPart>,
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
                    div { (Optional(part.rom_code.as_ref())) }
                    div { (Optional(part.date_code().calendar_short())) }
                    div { (Optional(part.manufacturer.as_ref())) }
                }
            },
        }
    }
}
