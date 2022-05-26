// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::legacy::{HasDateCode, LegacyChip};

pub struct ListingChip<'a> {
    pub chip: Option<&'a LegacyChip>,
    pub hide_type: bool,
}

impl<'a> View for ListingChip<'a> {
    fn render(&self) -> VirtualNode {
        match self.chip {
            None => html! { <td /> },
            Some(chip) => {
                html! {
                    <td class="listing-chip">
                        { if self.hide_type { None } else { Some(html! { <div>{chip.kind.as_ref()}</div>}) } }
                        <div>{chip.rom_code.as_ref()}</div>
                        <div>{chip.date_code().calendar_short()}</div>
                        <div>{chip.manufacturer.as_ref()}</div>
                    </td>
                }
            }
        }
    }
}
