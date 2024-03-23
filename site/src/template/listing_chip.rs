// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use crate::{
    legacy::{HasDateCode, LegacyChip},
    template::Optional,
};

pub struct ListingChip<'a> {
    pub chip: Option<&'a LegacyChip>,
    pub hide_type: bool,
}

impl<'a> Render for ListingChip<'a> {
    fn render(&self) -> Markup {
        match self.chip {
            None => html! { td; },
            Some(chip) => html! {
                td.listing-chip {
                    @if !self.hide_type {
                        div { (Optional(chip.kind.as_ref())) }
                    }
                    div { (Optional(chip.rom_code.as_ref())) }
                    div { (Optional(chip.date_code().calendar_short())) }
                    div { (Optional(chip.manufacturer.as_ref())) }
                }
            },
        }
    }
}
