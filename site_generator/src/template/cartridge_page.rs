use gbhwdb_backend::config::cartridge::ChipRoleConfig;
use percy_dom::{html, IterableNodes, View, VirtualNode};
use std::convert::identity;

use crate::legacy::{HasDateCode, LegacyCartridgeSubmission, LegacyChip, LegacyPhoto};

pub struct CartridgePage<'a> {
    pub submission: &'a LegacyCartridgeSubmission,
}

impl<'a> CartridgePage<'a> {
    pub fn new(submission: &'a LegacyCartridgeSubmission) -> Self {
        CartridgePage { submission }
    }
    pub fn render_photo(&self, photo: &LegacyPhoto) -> VirtualNode {
        let url = format!(
            "/static/{code}/{slug}_{name}",
            code = self.submission.code,
            slug = self.submission.slug,
            name = photo.name
        );
        html! {
            <a href={&url}>
                <img src={&url}>
            </a>
        }
    }
}

impl<'a> View for CartridgePage<'a> {
    fn render(&self) -> VirtualNode {
        let metadata = &self.submission.metadata;
        let photos = &self.submission.photos;
        let board = &metadata.board;
        html! {
            <article class="page-cartridge">
                <h2>{format!("{}: {} [{}]", metadata.cfg.name, self.submission.title, self.submission.contributor)}</h2>
                <div class="page-cartridge__photo">
                    {
                        [photos.front.as_ref(), photos.back.as_ref()]
                            .into_iter()
                            .filter_map(identity)
                            .map(|photo| self.render_photo(photo))
                            .collect::<Vec<_>>()
                    }
                </div>
                <dl>
                    { metadata.code.as_deref().into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Release"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { metadata.stamp.as_deref().into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Stamp on case"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                </dl>
                <h3>{"Board"}</h3>
                <div class="page-cartridge__photo">
                    {
                        [photos.pcb_front.as_ref(), photos.pcb_back.as_ref()]
                            .into_iter()
                            .filter_map(identity)
                            .map(|photo| self.render_photo(photo))
                            .collect::<Vec<_>>()
                    }
                </div>
                <dl>
                    <dt>{"Board type"}</dt>
                    <dd>{&board.kind}</dd>
                    { board.date_code().calendar().into_iter().flat_map(|date| {
                        [
                            html!{ <dt>{"Manufacture date"}</dt> },
                            html!{ <dd>{date}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { board.circled_letters.as_deref().into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Circled letter(s) on board"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { board.extra_label.as_deref().into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Extra label"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                </dl>
                <h3>Chips</h3>
                <table>
                    <thead>
                        <tr>
                            <th />
                            <th>{"Chip"}</th>
                            <th>{"Type"}</th>
                            <th>{"Manufacturer"}</th>
                            <th>{"Date"}</th>
                            <th>{"Label"}</th>
                        </tr>
                    </thead>
                    { ChipRoleConfig::from(board.layout).iter()
                        .map(|(designator, role)|
                            render_chip(designator.as_str(), role.display(), metadata.board[designator].as_ref())
                        )
                        .collect::<Vec<_>>()
                    }
                </table>
            </article>
        }
    }
}

fn render_chip(designator: &str, label: &str, chip: Option<&LegacyChip>) -> VirtualNode {
    html! {
        <tr class="console-page-chip">
            <td>{designator}</td>
            <td>{label}</td>
            <td>{chip.and_then(|chip| chip.kind.as_ref())}</td>
            <td>{chip.and_then(|chip| chip.manufacturer.as_ref())}</td>
            <td>{chip.and_then(|chip| chip.date_code().calendar())}</td>
            <td>{chip.and_then(|chip| chip.label.as_ref())}</td>
        </tr>
    }
}
