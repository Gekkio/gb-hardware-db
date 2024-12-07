// SPDX-FileCopyrightText: 2017-2024 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Error};
use gbhwdb_model::{
    input::Part,
    parser::{self, LabelParser, Manufacturer, PartDateCode},
};

use crate::{process::to_full_year, process::DateCode};

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
    part: &Option<Part>,
    f: &F,
) -> Option<ProcessedPart> {
    part.as_ref().map(|part| {
        boxed_parser(f)(year_hint, part)
            .unwrap()
            .unwrap_or_default()
    })
}

pub type BoxedParser<'a> =
    Box<dyn Fn(Option<u16>, &Part) -> Result<Option<ProcessedPart>, Error> + 'a>;

pub fn boxed_parser<T: ParsedPart, F: LabelParser<T>>(f: &F) -> BoxedParser<'_> {
    Box::new(|year_hint, part| {
        part.label
            .as_ref()
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

impl ParsedPart for parser::Gen1Soc {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        use gbhwdb_model::parser::Gen1SocKind::*;
        ProcessedPart {
            label: Some(label),
            kind: Some(
                match self.kind {
                    Dmg0 => "DMG-CPU",
                    DmgA => "DMG-CPU A",
                    DmgB => "DMG-CPU B",
                    DmgC => "DMG-CPU C",
                    DmgBlobB => "DMG-CPU B (blob)",
                    DmgBlobC => "DMG-CPU C (blob)",
                    Sgb => "SGB-CPU 01",
                }
                .to_owned(),
            ),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::Gen2Soc {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        use gbhwdb_model::parser::Gen2SocKind::*;
        ProcessedPart {
            label: Some(label),
            kind: Some(
                match self.kind {
                    Mgb => "CPU MGB",
                    Sgb2 => "CPU SGB2",
                }
                .to_owned(),
            ),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::Crystal {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: Some(self.format_frequency()),
            manufacturer: self.manufacturer,
            date_code: DateCode {
                year: to_full_year(year_hint, self.year),
                week: self.week,
                month: self.month,
                ..DateCode::default()
            },
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::Coil {
    fn process(self, _: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: Some(self.kind),
            manufacturer: self.manufacturer,
            ..ProcessedPart::default()
        }
    }
}

impl ParsedPart for parser::ChipYearWeek {
    fn process(self, year_hint: Option<u16>, label: String) -> ProcessedPart {
        ProcessedPart {
            label: Some(label),
            kind: Some(self.kind),
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..ProcessedPart::default()
        }
    }
}

fn loose_datecode(year_hint: Option<u16>, date_code: Option<PartDateCode>) -> DateCode {
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
            kind: Some(self.mbc_type.display_name().to_owned()),
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..ProcessedPart::default()
        }
    }
}
