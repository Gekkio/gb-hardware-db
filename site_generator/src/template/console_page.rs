use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::legacy::{
    console::{ChipInfo, LegacyConsoleMetadata},
    HasDateCode, LegacyChip, LegacyPhoto, LegacyPhotos, LegacySubmission, PhotoInfo, PhotoKind,
};

pub struct ConsolePage<'a, M, P> {
    pub submission: &'a LegacySubmission<M, P>,
    pub extra_sections: Vec<Box<dyn Fn(&Self, &M) -> VirtualNode>>,
    pub extra_chips: Vec<Box<dyn Fn(&M) -> (&str, &str, Option<&LegacyChip>)>>,
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> ConsolePage<'a, M, P> {
    pub fn new(submission: &'a LegacySubmission<M, P>) -> Self {
        ConsolePage {
            submission,
            extra_sections: Vec::new(),
            extra_chips: Vec::new(),
        }
    }
    fn render_chip_info(&self, info: &ChipInfo<M>) -> VirtualNode {
        let chip = (info.getter)(&self.submission.metadata);
        render_chip(info.designator, info.label, chip)
    }
    fn render_photo_info(&self, photo: &PhotoInfo<P>) -> Option<VirtualNode> {
        (photo.getter)(&self.submission.photos).map(|photo| self.render_photo(photo))
    }
    pub fn render_photo(&self, photo: &LegacyPhoto) -> VirtualNode {
        let url = format!(
            "/static/{console}/{slug}_{name}",
            console = M::CONSOLE.id(),
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

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> View for ConsolePage<'a, M, P> {
    fn render(&self) -> VirtualNode {
        let metadata = &self.submission.metadata;
        let mainboard = metadata.mainboard();
        html! {
            <article class={format!("page-console page-console--{console}", console = M::CONSOLE.id())}>
                <h2>{format!("{}: {} [{}]", M::CONSOLE.code(), self.submission.title, self.submission.contributor)}</h2>
                <div class="page-console__photo">
                    { P::infos().iter()
                        .filter(|p| p.kind == PhotoKind::MainUnit)
                        .filter_map(|photo| self.render_photo_info(photo))
                        .collect::<Vec<_>>()
                    }
                </div>
                <dl>
                    { metadata.shell().color.into_iter().flat_map(|color| {
                        [
                            html!{ <dt>{"Color"}</dt> },
                            html!{ <dd>{color}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { metadata.shell().release_code.into_iter().flat_map(|release_code| {
                        [
                            html!{ <dt>{"Release code"}</dt> },
                            html!{ <dd>{release_code}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { metadata.shell().date_code.calendar().into_iter().flat_map(|assembled| {
                        [
                            html!{ <dt>{"Assembly date"}</dt> },
                            html!{ <dd>{assembled}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { metadata.shell().stamp.into_iter().flat_map(|stamp| {
                        [
                            html!{ <dt>{"Stamp on case"}</dt> },
                            html!{ <dd>{stamp}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { metadata.lcd_panel().and_then(|panel| panel.label.as_ref()).into_iter().flat_map(|label| {
                        [
                            html!{ <dt>{"LCD panel label"}</dt> },
                            html!{ <dd>{label}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { metadata.lcd_panel().and_then(|panel| panel.date_code().calendar()).into_iter().flat_map(|date| {
                        [
                            html!{ <dt>{"LCD panel date"}</dt> },
                            html!{ <dd>{date}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                </dl>
                <h3>{"Mainboard"}</h3>
                <div class="page-console__photo">
                    { P::infos().iter()
                        .filter(|p| p.kind == PhotoKind::MainBoard)
                        .filter_map(|photo| self.render_photo_info(photo))
                        .collect::<Vec<_>>()
                    }
                </div>
                <dl>
                    <dt>{"Board type"}</dt>
                    <dd>{mainboard.kind}</dd>
                    { mainboard.date_code.calendar().into_iter().flat_map(|date| {
                        [
                            html!{ <dt>{"Manufacture date"}</dt> },
                            html!{ <dd>{date}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.number_pair.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Number pair on board"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.stamp.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Stamp on board"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.stamp_front.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Secondary stamp on board (front)"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.stamp_back.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Secondary stamp on board (back)"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.circled_letters.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Circled letter(s) on board"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.letter_at_top_right.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Letter at top right"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                    { mainboard.extra_label.into_iter().flat_map(|value| {
                        [
                            html!{ <dt>{"Extra label"}</dt> },
                            html!{ <dd>{value}</dd> },
                        ]
                    }).collect::<Vec<_>>() }
                </dl>
                { self.extra_sections.iter().map(|section| (section)(self, metadata)).collect::<Vec<_>>() }
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
                    <tbody>
                        { M::chips()
                            .iter()
                            .map(|info| self.render_chip_info(info))
                            .collect::<Vec<_>>() }
                        { metadata.lcd_panel().iter().flat_map(|panel| {
                            [
                                render_chip("-", "LCD column driver", panel.column_driver.as_ref()),
                                render_chip("-", "LCD row driver", panel.row_driver.as_ref()),
                            ]
                        }).collect::<Vec<_>>() }
                        { self.extra_chips.iter()
                            .map(|f| {
                                let (designator, label, chip) = f(metadata);
                                render_chip(designator, label, chip)
                            })
                            .collect::<Vec<_>>()
                        }
                    </tbody>
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
