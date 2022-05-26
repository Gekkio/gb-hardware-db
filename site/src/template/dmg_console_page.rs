use percy_dom::{html, IterableNodes, View, VirtualNode};

use super::console_page::ConsolePage;
use crate::legacy::{console::LegacyDmgMetadata, HasDateCode, LegacyDmgSubmission};

pub struct DmgConsolePage<'a> {
    pub submission: &'a LegacyDmgSubmission,
}

impl<'a> DmgConsolePage<'a> {
    pub fn new(submission: &'a LegacyDmgSubmission) -> Self {
        DmgConsolePage { submission }
    }
}

impl<'a> View for DmgConsolePage<'a> {
    fn render(&self) -> VirtualNode {
        ConsolePage {
            submission: self.submission,
            extra_sections: vec![Box::new(|page, m: &LegacyDmgMetadata| {
                html! {
                    <div>
                        <h3>{"LCD board"}</h3>
                        <div class="page-console__photo">
                            { page.submission.photos.lcd_board_front.as_ref().map(|photo| page.render_photo(photo)) }
                            { page.submission.photos.lcd_board_back.as_ref().map(|photo| page.render_photo(photo)) }
                        </div>
                        { m.lcd_board.as_ref().map(|board| {
                            html! {
                                <dl>
                                    <dt>{"Board type"}</dt>
                                    <dd>{&board.kind}</dd>
                                    {board.date_code().calendar().into_iter().flat_map(|date| {
                                        [
                                            html!{ <dt>{"Manufacture date"}</dt> },
                                            html!{ <dd>{date}</dd> },
                                        ]
                                    }).collect::<Vec<_>>() }
                                    {board.stamp.as_ref().into_iter().flat_map(|value| {
                                        [
                                            html!{ <dt>{"Stamp"}</dt> },
                                            html!{ <dd>{value}</dd> },
                                        ]
                                    }).collect::<Vec<_>>() }
                                    {board.circled_letters.as_ref().into_iter().flat_map(|value| {
                                        [
                                            html!{ <dt>{"CIrcled letter(s) on board"}</dt> },
                                            html!{ <dd>{value}</dd> },
                                        ]
                                    }).collect::<Vec<_>>() }
                                </dl>
                            }
                        }) }
                        <h3>{"Power board"}</h3>
                        <div class="page-console__photo">
                            { page.submission.photos.power_board_front.as_ref().map(|photo| page.render_photo(photo)) }
                            { page.submission.photos.power_board_back.as_ref().map(|photo| page.render_photo(photo)) }
                        </div>
                        { m.power_board.as_ref().map(|board| {
                            html! {
                                <dl>
                                    <dt>{"Board type"}</dt>
                                    <dd>{&board.kind}</dd>
                                    {board.date_code().calendar().into_iter().flat_map(|date| {
                                        [
                                            html!{ <dt>{"Manufacture date"}</dt> },
                                            html!{ <dd>{date}</dd> },
                                        ]
                                    }).collect::<Vec<_>>() }
                                    <dt>{"Label"}</dt>
                                    <dd>{&board.label}</dd>
                                </dl>
                            }
                        }) }
                        <h3>{"Jack board"}</h3>
                        <div class="page-console__photo">
                            { page.submission.photos.jack_board_front.as_ref().map(|photo| page.render_photo(photo)) }
                            { page.submission.photos.jack_board_back.as_ref().map(|photo| page.render_photo(photo)) }
                        </div>
                        { m.jack_board.as_ref().map(|board| {
                            html! {
                                <dl>
                                    <dt>{"Board type"}</dt>
                                    <dd>{&board.kind}</dd>
                                    {board.extra_label.as_ref().into_iter().flat_map(|value| {
                                        [
                                            html!{ <dt>{"Extra label"}</dt> },
                                            html!{ <dd>{value}</dd> },
                                        ]
                                    }).collect::<Vec<_>>() }
                                </dl>
                            }
                        }) }
                    </div>
                }
            })],
            extra_chips: vec![Box::new(|m: &LegacyDmgMetadata| {
                (
                    "-",
                    "LCD regulator",
                    m.lcd_board
                        .as_ref()
                        .and_then(|board| board.regulator.as_ref()),
                )
            })],
        }
        .render()
    }
}
