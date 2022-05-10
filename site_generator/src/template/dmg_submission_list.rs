use percy_dom::{html, IterableNodes, View, VirtualNode};

use crate::legacy::{console::LegacyDmgMetadata, HasDateCode, LegacyDmgSubmission};

use super::console_submission_list::ConsoleSubmissionList;

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
                                <div>{board.calendar_short()}</div>
                                { board.lcd_panel.as_ref().and_then(|panel| panel.calendar_short()).map(|date_code| {
                                    html! {
                                        <div>{format!("LCD panel: {}", date_code)}</div>
                                    }
                                })}
                            </div>
                        }
                    })
                }),
                Box::new(|m: &LegacyDmgMetadata| {
                    m.power_board.as_ref().map(|board| {
                        html! {
                            <div>
                                <div>{format!("Type {}", board.kind)}</div>
                                <div>{board.calendar_short()}</div>
                            </div>
                        }
                    })
                }),
                Box::new(|m: &LegacyDmgMetadata| {
                    m.jack_board.as_ref().map(|board| {
                        board.kind.as_str().into()
                    })
                }),
            ],
        }.render()
    }
}
