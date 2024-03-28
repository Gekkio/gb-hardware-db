// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::{anyhow, Error};
use gbhwdb_backend::{
    input::Part,
    parser::{self, LabelParser, Manufacturer},
};

use crate::{
    legacy::{to_legacy_year, LegacyPart},
    DateCode,
};

pub trait ToLegacyPart {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart;
}

pub fn map_legacy_part<T: ToLegacyPart, F: LabelParser<T>>(
    year_hint: Option<u16>,
    part: &Option<Part>,
    f: &F,
) -> Option<LegacyPart> {
    part.as_ref().map(|part| {
        boxed_parser(f)(year_hint, part)
            .unwrap()
            .unwrap_or_default()
    })
}

pub type BoxedParser<'a> =
    Box<dyn Fn(Option<u16>, &Part) -> Result<Option<LegacyPart>, Error> + 'a>;

pub fn boxed_parser<'a, T: ToLegacyPart, F: LabelParser<T>>(f: &'a F) -> BoxedParser<'a> {
    Box::new(|year_hint, part| {
        part.label
            .as_ref()
            .map(|label| {
                let part = f
                    .parse(label)
                    .map_err(|label| anyhow!("Failed to parse {label}"))?;
                Ok(part.to_legacy_part(year_hint, label.clone()))
            })
            .transpose()
    })
}

impl ToLegacyPart for parser::Gen1Soc {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        use gbhwdb_backend::parser::Gen1SocKind::*;
        LegacyPart {
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
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::Gen2Soc {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        use gbhwdb_backend::parser::Gen2SocKind::*;
        LegacyPart {
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
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::StaticRam {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: self.part,
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::Crystal {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: Some(self.format_frequency()),
            manufacturer: self.manufacturer,
            date_code: DateCode {
                year: to_legacy_year(year_hint, self.year),
                week: self.week,
                month: self.month,
                ..DateCode::default()
            },
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::Coil {
    fn to_legacy_part(self, _: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: Some(self.kind),
            manufacturer: self.manufacturer,
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::Transformer {
    fn to_legacy_part(self, _: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: Some(self.kind),
            manufacturer: self.manufacturer,
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::SgbRom {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: self.chip_type,
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            rom_code: Some(self.rom_code),
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::ChipYearWeek {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: Some(self.kind),
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::MaskRom {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: self.chip_type,
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::Mapper {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: Some(self.mbc_type.display_name().to_owned()),
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::SupervisorReset {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: Some(self.chip_type),
            manufacturer: self.manufacturer,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..LegacyPart::default()
        }
    }
}

impl ToLegacyPart for parser::Tama {
    fn to_legacy_part(self, year_hint: Option<u16>, label: String) -> LegacyPart {
        LegacyPart {
            label: Some(label),
            kind: match self.tama_type {
                parser::TamaType::Tama5 => Some("TAMA5".to_owned()),
                parser::TamaType::Tama6 => Some("TAMA6".to_owned()),
                parser::TamaType::Tama7 => Some("TAMA7".to_owned()),
            },
            manufacturer: None,
            date_code: DateCode::loose_year_week(year_hint, self.year, self.week),
            ..LegacyPart::default()
        }
    }
}
