use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::legacy::{HasDateCode, LegacyChip};

#[derive(Copy, Clone, Debug)]
pub struct ConsoleListingChip<'a> {
    pub chip: Option<&'a LegacyChip>,
    pub hide_type: bool,
}

impl<'a> View for ConsoleListingChip<'a> {
    fn render(&self) -> VirtualNode {
        match self.chip {
            None => html! { <td /> },
            Some(chip) => {
                html! {
                    <td class="console-listing-chip">
                        { if self.hide_type { None } else { Some(html! { <div>{chip.kind.as_ref()}</div>}) } }
                        <div>{chip.rom_code.as_ref()}</div>
                        <div>{chip.calendar_short()}</div>
                        <div>{chip.manufacturer.as_ref()}</div>
                    </td>
                }
            }
        }
    }
}
