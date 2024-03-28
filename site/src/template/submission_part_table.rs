use maud::{html, Markup};

use crate::{template::Optional, LegacyPart};

pub struct SubmissionPart<'a> {
    pub designator: &'a str,
    pub label: &'a str,
    pub part: Option<&'a LegacyPart>,
}

pub fn submission_part_table<'a>(parts: impl Iterator<Item = SubmissionPart<'a>>) -> Markup {
    html! {
        table {
            thead {
                tr {
                    th;
                    th { "Part" }
                    th { "Type" }
                    th { "Manufacturer" }
                    th { "Date" }
                    th { "Label" }
                }
            }
            tbody {
                @for SubmissionPart { designator, label, part } in parts {
                    tr.submission-part {
                        td { (designator) }
                        td { (label) }
                        @if let Some(part) = part {
                            td { (Optional(part.kind.as_ref())) }
                            td { (Optional(part.manufacturer.as_ref().map(|m| m.name()))) }
                            td { (Optional(part.date_code.calendar())) }
                            td { (Optional(part.label.as_ref())) }
                        } @else {
                            td;
                            td;
                            td;
                            td;
                        }
                    }
                }
            }
        }
    }
}
