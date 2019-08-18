use gbhwdb_backend::input::Chip;
use gbhwdb_backend::parser::{self, Manufacturer, Year};

use crate::legacy::{to_legacy_manufacturer, to_legacy_year, LegacyChip};

pub trait ToLegacyChip {
    fn kind(&self) -> Option<String> {
        None
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        None
    }
    fn year(&self) -> Option<Year> {
        None
    }
    fn week(&self) -> Option<u8> {
        None
    }
    fn month(&self) -> Option<u8> {
        None
    }
}

pub fn map_legacy_chip<T: ToLegacyChip, F: FnOnce(&str) -> Result<T, ()>>(
    year_hint: Option<u16>,
    chip: &Option<Chip>,
    f: F,
) -> Option<LegacyChip> {
    chip.as_ref()
        .map(|chip| to_legacy_chip(year_hint, chip, f).unwrap_or_default())
}

pub fn to_legacy_chip<T: ToLegacyChip, F: FnOnce(&str) -> Result<T, ()>>(
    year_hint: Option<u16>,
    chip: &Chip,
    f: F,
) -> Option<LegacyChip> {
    chip.label.as_ref().map(|label| {
        let chip = f(label).unwrap_or_else(|_| panic!("{}", label));
        LegacyChip {
            label: Some(label.to_owned()),
            kind: chip.kind(),
            manufacturer: to_legacy_manufacturer(chip.manufacturer()),
            year: to_legacy_year(year_hint, chip.year()),
            week: chip.week(),
            month: chip.month(),
        }
    })
}

impl ToLegacyChip for parser::Gen1Cpu {
    fn kind(&self) -> Option<String> {
        use gbhwdb_backend::parser::Gen1CpuKind::*;
        Some(
            (match self.kind {
                Dmg0 => "DMG-CPU",
                DmgA => "DMG-CPU A",
                DmgB => "DMG-CPU B",
                DmgC => "DMG-CPU C",
                DmgBlobB => "DMG-CPU B (blob)",
                DmgBlobC => "DMG-CPU C (blob)",
                Sgb => "SGB-CPU 01",
            })
            .to_owned(),
        )
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year.map(Year::Full)
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::Gen2Cpu {
    fn kind(&self) -> Option<String> {
        use gbhwdb_backend::parser::Gen2CpuKind::*;
        Some(
            (match self.kind {
                Mgb => "CPU MGB",
                Sgb2 => "CPU SGB2",
            })
            .to_owned(),
        )
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year.map(Year::Full)
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::CgbCpu {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year.map(Year::Full)
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgbCpu {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year.map(Year::Full)
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::Ram {
    fn kind(&self) -> Option<String> {
        self.chip_type.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgbRam {
    fn kind(&self) -> Option<String> {
        self.kind.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::Crystal {
    fn kind(&self) -> Option<String> {
        None
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

impl ToLegacyChip for parser::DmgAmp {
    fn kind(&self) -> Option<String> {
        Some("IR3R40".to_owned())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::DmgReg {
    fn kind(&self) -> Option<String> {
        Some("IR3E02".to_owned())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::MgbAmp {
    fn kind(&self) -> Option<String> {
        Some(self.kind.to_owned())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::CgbReg {
    fn kind(&self) -> Option<String> {
        Some("IR3E06N".to_owned())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgbReg {
    fn kind(&self) -> Option<String> {
        Some("IR3E09N".to_owned())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::Coil {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
}

impl ToLegacyChip for parser::Transformer {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
}

impl ToLegacyChip for parser::Cic {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        Some(Manufacturer::Sharp)
    }
    fn year(&self) -> Option<Year> {
        self.year.map(Year::Full)
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::Icd2 {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::SgbRom {
    fn kind(&self) -> Option<String> {
        Some(match &self.chip_type {
            Some(chip_type) => format!("{} ({})", self.rom_code, chip_type),
            None => self.rom_code.to_owned(),
        })
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgbU4 {
    fn kind(&self) -> Option<String> {
        self.kind.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgbAmp {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgsU4 {
    fn kind(&self) -> Option<String> {
        self.kind.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::AgsU5 {
    fn kind(&self) -> Option<String> {
        self.kind.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::GbsDol {
    fn kind(&self) -> Option<String> {
        Some("GBS-DOL".to_owned())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

impl ToLegacyChip for parser::GbsReg {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}
