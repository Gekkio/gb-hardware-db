use gbhwdb_backend::config::cartridge::{BoardLayout, ChipRole, ChipRoleConfig};
use percy_dom::{html, IterableNodes, View, VirtualNode};

use super::{
    listing_chip::ListingChip, listing_entry_cell::ListingEntryCell,
    listing_photos_cell::ListingPhotosCell,
};
use crate::legacy::{HasDateCode, LegacyCartridgeSubmission, LegacyChip};

pub struct MapperCfg {
    pub id: &'static str,
    pub name: &'static str,
    pub chips: &'static [ChipRole],
    pub match_fn: Box<dyn Fn(BoardLayout, Option<&LegacyChip>) -> bool + Send + Sync>,
}

pub struct Mapper<'a> {
    pub cfg: &'a MapperCfg,
    pub submissions: Vec<&'a LegacyCartridgeSubmission>,
}

impl<'a> View for Mapper<'a> {
    fn render(&self) -> VirtualNode {
        return html! {
            <article>
                <h2>{format!("Cartridges by mapper: {}", self.cfg.name)}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"Entry"}</th>
                            <th>{"Release"}</th>
                            <th>{"Board"}</th>
                            { self.cfg.chips.iter().map(|role| html! {
                                <th>{role.display()}</th>
                            }).collect::<Vec<_>>() }
                            <th>{"Photos"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.submissions.iter().map(|&submission| {
                            render_submission(self.cfg, submission)
                        }).collect::<Vec<_>>() }
                    </tbody>
                </table>
            </article>
        };
    }
}

fn render_submission(cfg: &MapperCfg, submission: &LegacyCartridgeSubmission) -> VirtualNode {
    let metadata = &submission.metadata;
    let chips = ChipRoleConfig::from(submission.metadata.board.layout);
    html! {
        <tr>
            { ListingEntryCell {
                url_prefix: "/cartridges",
                primary_text: &submission.metadata.cfg.name,
                secondary_texts: &[&submission.code, &submission.title],
                submission,
            }.render() }
            <td>{metadata.code.as_deref().unwrap_or_default()}</td>
            <td>
                <div>{&metadata.board.kind}</div>
                <div>{metadata.board.date_code().calendar().unwrap_or_default()}</div>
            </td>
            { cfg.chips.iter().map(|&role| {
                let chip = chips.iter().find(|&(_, candidate)| candidate == role)
                    .and_then(|(designator, _)| submission.metadata.board[designator].as_ref());
                ListingChip {
                    chip,
                    hide_type: false,
                }.render()
            }).collect::<Vec<_>>() }
            { ListingPhotosCell {
                submission,
            }.render() }
        </tr>
    }
}
