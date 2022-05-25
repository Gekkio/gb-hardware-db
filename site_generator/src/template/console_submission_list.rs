use percy_dom::{html, IterableNodes, View, VirtualNode};

use super::submission_list::{submission_list_photos, submission_list_submission};
use crate::{
    legacy::{
        console::{ChipInfo, LegacyConsoleMetadata},
        HasDateCode, LegacyPhotos, LegacySubmission,
    },
    template::chip::ConsoleListingChip,
};

pub struct ConsoleSubmissionList<'a, M, P> {
    pub submissions: &'a [LegacySubmission<M, P>],
    pub board_column_name: &'static str,
    pub extra_columns: &'static [&'static str],
    pub extra_cells: Vec<Box<dyn Fn(&M) -> Option<VirtualNode>>>,
}

impl<'a, M, P> ConsoleSubmissionList<'a, M, P> {
    pub fn new(submissions: &'a [LegacySubmission<M, P>]) -> Self {
        ConsoleSubmissionList {
            submissions,
            board_column_name: "Board",
            extra_columns: &[],
            extra_cells: vec![],
        }
    }
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> View for ConsoleSubmissionList<'a, M, P> {
    fn render(&self) -> VirtualNode {
        let console = M::CONSOLE;
        let chips = M::chips();
        html! {
            <article>
                <h2>{format!("{} ({})", console.name(), console.code())}</h2>
                <table>
                    <thead>
                        <tr>
                            <th>{"ID"}</th>
                            <th>{self.board_column_name}</th>
                            { chips.iter().map(|chip|
                                html! {
                                    <th>{format!("{} ({})", chip.label, chip.designator)}</th>
                                }
                            ).collect::<Vec<_>>() }
                            { self.extra_columns.iter().map(|&column|
                                html! {
                                    <th>{column}</th>
                                }
                            ).collect::<Vec<_>>() }
                            <th>{"Photos"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        { self.submissions.iter().map(|submission|
                            Submission { submission, chips: &chips, extra_cells: &self.extra_cells }.render()
                        ).collect::<Vec<_>>() }
                    </tbody>
                </table>
                <h3>{"Data dumps"}</h3>
                <a href={format!("/static/export/consoles/{id}.csv", id=console.id())}>{"UTF-8 encoded CSV"}</a>
            </article>
        }
    }
}

struct Submission<'a, M: LegacyConsoleMetadata, P> {
    pub submission: &'a LegacySubmission<M, P>,
    pub chips: &'a [ChipInfo<M>],
    pub extra_cells: &'a [Box<dyn Fn(&M) -> Option<VirtualNode>>],
}

impl<'a, M: LegacyConsoleMetadata, P: LegacyPhotos> View for Submission<'a, M, P> {
    fn render(&self) -> VirtualNode {
        let metadata = &self.submission.metadata;
        html! {
            <tr>
                { submission_list_submission("/consoles", &self.submission) }
                <td>
                    <div>{metadata.mainboard().kind}</div>
                    {metadata.mainboard().date_code.calendar_short().map(|date_code| {
                        html! {
                            <div>{format!("{date_code}")}</div>
                        }
                    })}
                    {metadata.shell().date_code.calendar_short().map(|date_code| {
                        html! {
                            <div>{format!("Assembled: {date_code}")}</div>
                        }
                    })}
                    {metadata.shell().release_code.map(|release_code| {
                        html! {
                            <div>{format!("Release: {release_code}")}</div>
                        }
                    })}
                    {metadata.lcd_panel().and_then(|panel| panel.date_code().calendar_short()).map(|date_code| {
                        html! {
                            <div>{format!("LCD panel: {date_code}")}</div>
                        }
                    })}
                </td>
                { self.chips.iter().map(|chip| {
                    ConsoleListingChip {
                        chip: (chip.getter)(&metadata),
                        hide_type: chip.hide_type,
                    }.render()
                }).collect::<Vec<_>>() }
                { self.extra_cells.iter().map(|cell| html! {
                    <td>{cell(&metadata)}</td>
                }).collect::<Vec<_>>() }
                { submission_list_photos(self.submission) }
            </tr>
        }
    }
}
