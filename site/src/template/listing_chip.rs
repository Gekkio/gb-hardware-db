// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use maud::{html, Markup, Render};

use crate::legacy::{HasDateCode, LegacyChip};

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
                        div { (chip.kind.as_deref().unwrap_or_default()) }
                    }
                    div { (chip.rom_code.as_deref().unwrap_or_default()) }
                    div { (chip.date_code().calendar_short().unwrap_or_default()) }
                    div { (chip.manufacturer.as_deref().unwrap_or_default()) }
                }
            },
        }
    }
}
