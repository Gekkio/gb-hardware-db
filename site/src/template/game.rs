// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::{ChipRoleConfig, GameConfig};
use percy_dom::{html, IterableNodes, View, VirtualNode};

use super::{
    listing_chip::ListingChip, listing_entry_cell::ListingEntryCell,
    listing_photos_cell::ListingPhotosCell,
};
use crate::legacy::{HasDateCode, LegacyCartridgeSubmission};

#[derive(Clone, Debug)]
pub struct Game<'a> {
    pub cfg: &'a GameConfig,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
}

impl<'a> View for Game<'a> {
    fn render(&self) -> VirtualNode {
        let layout = self.cfg.layouts[0];
        let chips = ChipRoleConfig::from(layout);
        return html! {
            <article>
                <h2>{&self.cfg.name}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"Entry"}</th>
                            <th>{"Release"}</th>
                            <th>{"Board"}</th>
                            { chips.iter().map(|(designator, role)| html! {
                                <th>{format!("{} ({})", role.display(), designator.as_str())}</th>
                            }).collect::<Vec<_>>() }
                            <th>{"Photos"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.submissions.iter().map(|&submission| {
                            render_submission(submission, &chips)
                        }).collect::<Vec<_>>() }
                    </tbody>
                </table>
            </article>
        };
    }
}

fn render_submission(
    submission: &LegacyCartridgeSubmission,
    chips: &ChipRoleConfig,
) -> VirtualNode {
    let metadata = &submission.metadata;
    html! {
        <tr>
            { ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.title,
                secondary_texts: &[],
                submission,
            }.render() }
            <td>{metadata.code.as_deref().unwrap_or_default()}</td>
            <td>
                <div>{&metadata.board.kind}</div>
                <div>{metadata.board.date_code().calendar().unwrap_or_default()}</div>
            </td>
            { chips.iter().map(|(designator, _)|
                ListingChip {
                    chip: metadata.board[designator].as_ref(),
                    hide_type: false,
                }
            ).collect::<Vec<_>>() }
            { ListingPhotosCell {
                submission,
            }.render() }
        </tr>
    }
}
