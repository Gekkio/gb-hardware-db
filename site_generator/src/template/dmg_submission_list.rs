use percy_dom::{html, IterableNodes, View, VirtualNode};

use super::console_submission_list::ConsoleSubmissionList;
use crate::legacy::{console::LegacyDmgMetadata, HasDateCode, LegacyDmgSubmission};

pub struct DmgSubmissionList<'a> {
    pub submissions: &'a [LegacyDmgSubmission],
}

impl<'a> DmgSubmissionList<'a> {
    pub fn new(submissions: &'a [LegacyDmgSubmission]) -> Self {
        DmgSubmissionList { submissions }
    }
}

impl<'a> View for DmgSubmissionList<'a> {
    fn render(&self) -> VirtualNode {
        ConsoleSubmissionList {
            submissions: self.submissions,
            board_column_name: "Mainboard",
            extra_columns: &["LCD board", "Power board", "Jack board"],
            extra_cells: vec![
                Box::new(|m: &LegacyDmgMetadata| {
                    m.lcd_board.as_ref().map(|board| {
                        html! {
                            <div>
                                <div>{&board.kind}</div>
                                <div>{board.date_code().calendar_short()}</div>
                            </div>
                        }
                    })
                }),
                Box::new(|m: &LegacyDmgMetadata| {
                    m.power_board.as_ref().map(|board| {
                        html! {
                            <div>
                                <div>{format!("Type {}", board.kind)}</div>
                                <div>{board.date_code().calendar_short()}</div>
                            </div>
                        }
                    })
                }),
                Box::new(|m: &LegacyDmgMetadata| {
                    m.jack_board
                        .as_ref()
                        .map(|board| board.kind.as_str().into())
                }),
            ],
        }
        .render()
    }
}
