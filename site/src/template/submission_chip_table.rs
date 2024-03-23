use maud::{html, Markup};

use crate::{template::Optional, HasDateCode, LegacyChip};

pub struct SubmissionChip<'a> {
    pub designator: &'a str,
    pub label: &'a str,
    pub chip: Option<&'a LegacyChip>,
}

pub fn submission_chip_table<'a>(chips: impl Iterator<Item = SubmissionChip<'a>>) -> Markup {
    html! {
        table {
            thead {
                tr {
                    th;
                    th { "Chip" }
                    th { "Type" }
                    th { "Manufacturer" }
                    th { "Date" }
                    th { "Label" }
                }
            }
            tbody {
                @for chip in chips {
                    tr.submission-chip {
                        td { (chip.designator) }
                        td { (chip.label) }
                        @if let Some(chip) = chip.chip {
                            td { (Optional(chip.kind.as_ref())) }
                            td { (Optional(chip.manufacturer.as_ref())) }
                            td { (Optional(chip.date_code().calendar())) }
                            td { (Optional(chip.label.as_ref())) }
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
