use gbhwdb_backend::config::cartridge::{ChipRoleConfig, GameConfig};
use percy_dom::{html, IterableNodes, View, VirtualNode};

use super::{
    listing_chip::ListingChip,
    submission_list::{submission_list_photos, submission_list_submission},
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
            { submission_list_submission("/cartridges", submission) }
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
            { submission_list_photos(submission) }
        </tr>
    }
}
