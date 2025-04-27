// SPDX-FileCopyrightText: 2017-2024 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{Error, anyhow};
use gbhwdb_model::{
    input::Part,
    parser::{self, LabelParser, Manufacturer, PartDateCode},
};

use crate::process::DateCode;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ProcessedPart {
    pub kind: Option<String>,
    pub label: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub date_code: DateCode,
    pub rom_id: Option<String>,
}

pub trait ParsedPart {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart;
}

pub fn map_part<T: ParsedPart, F: LabelParser<T>>(
    year_hint: Option<u16>,
    part: &Part,
    f: &F,
) -> Option<ProcessedPart> {
    if part.is_unknown() {
        None
    } else {
        Some(
            boxed_parser(f)(year_hint, part)
                .unwrap()
                .unwrap_or_default(),
        )
    }
}

pub type BoxedParser<'a> =
    Box<dyn Fn(Option<u16>, &Part) -> Result<Option<ProcessedPart>, Error> + 'a>;

pub fn boxed_parser<T: ParsedPart, F: LabelParser<T>>(f: &F) -> BoxedParser<'_> {
    Box::new(|year_hint, part| {
        Some(&part.label)
            .filter(|label| !label.is_empty())
            .map(|label| {
                let part = f
                    .parse(label)
                    .map_err(|label| anyhow!("Failed to parse {label}"))?;
                Ok(part.process(year_hint, label.clone()))
            })
            .transpose()
    })
}

impl ParsedPart for parser::UnknownChip {
    fn process(self, _: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            kind: None,
            label: Some(label),
            manufacturer: None,
            date_code: DateCode::default(),
            rom_id: None,
        }
    }
}

impl ParsedPart for parser::Crystal {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: Some(self.format_frequency()),
            manufacturer: self.manufacturer,
            date_code: loose_datecode(year_hint, self.date_code),
            ..ProcessedPart::default()
        }
    }
}

pub fn loose_datecode(year_hint: Option<u16>, date_code: Option<PartDateCode>) -> DateCode {
    match date_code {
        Some(PartDateCode::Year { year }) => DateCode::loose_year_week(year_hint, Some(year), None),
        Some(PartDateCode::YearMonth { year, month }) => {
            DateCode::loose_year_month(year_hint, Some(year), Some(month))
        }
        Some(PartDateCode::YearWeek { year, week }) => {
            DateCode::loose_year_week(year_hint, Some(year), Some(week))
        }
        None => DateCode::loose_year_week(year_hint, None, None),
    }
}

impl ParsedPart for parser::GenericPart {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: Some(self.kind),
            manufacturer: self.manufacturer,
            date_code: loose_datecode(year_hint, self.date_code),
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::GameMaskRom {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: self.chip_type,
            manufacturer: self.manufacturer,
            date_code: loose_datecode(year_hint, self.date_code),
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::MaskRom {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: self.chip_type,
            manufacturer: self.manufacturer,
            date_code: loose_datecode(year_hint, self.date_code),
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::Mapper {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: Some(self.kind.display_name().to_owned()),
            manufacturer: self.manufacturer,
            date_code: loose_datecode(year_hint, self.date_code),
            ..ProcessedPart::default()
        }
    }
}
