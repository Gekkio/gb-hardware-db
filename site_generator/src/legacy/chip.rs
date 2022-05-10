use gbhwdb_backend::{
    input::Chip,
    parser::{self, LabelParser, Manufacturer, Year},
    time::{Month, Week},
};

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
    fn week(&self) -> Option<Week> {
        None
    }
    fn month(&self) -> Option<Month> {
        None
    }
    fn rom_code(&self) -> Option<String> {
        None
    }
}

pub fn map_legacy_chip<T: ToLegacyChip, F: LabelParser<T>>(
    year_hint: Option<u16>,
    chip: &Option<Chip>,
    f: &F,
) -> Option<LegacyChip> {
    chip.as_ref()
        .map(|chip| to_legacy_chip(year_hint, chip, f).unwrap_or_default())
}

pub fn to_legacy_chip<T: ToLegacyChip, F: LabelParser<T>>(
    year_hint: Option<u16>,
    chip: &Chip,
    f: &F,
) -> Option<LegacyChip> {
    chip.label.as_ref().map(|label| {
        let chip = f.parse(label).unwrap_or_else(|_| panic!("{}", label));
        LegacyChip {
            label: Some(label.to_owned()),
            kind: chip.kind(),
            manufacturer: to_legacy_manufacturer(chip.manufacturer()),
            year: to_legacy_year(year_hint, chip.year()),
            week: chip.week(),
            month: chip.month(),
            rom_code: chip.rom_code(),
        }
    })
}

impl ToLegacyChip for parser::Gen1Soc {
    fn kind(&self) -> Option<String> {
        use gbhwdb_backend::parser::Gen1SocKind::*;
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
        self.year
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
}

impl ToLegacyChip for parser::Gen2Soc {
    fn kind(&self) -> Option<String> {
        use gbhwdb_backend::parser::Gen2SocKind::*;
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
        self.year
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
}

impl ToLegacyChip for parser::StaticRam {
    fn kind(&self) -> Option<String> {
        self.part.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
}

impl ToLegacyChip for parser::Crystal {
    fn kind(&self) -> Option<String> {
        Some(self.format_frequency())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
    fn month(&self) -> Option<Month> {
        self.month
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

impl ToLegacyChip for parser::SgbRom {
    fn kind(&self) -> Option<String> {
        self.chip_type.clone()
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
    fn rom_code(&self) -> Option<String> {
        Some(self.rom_code.clone())
    }
}

impl ToLegacyChip for parser::ChipYearWeek {
    fn kind(&self) -> Option<String> {
        Some(self.kind.clone())
    }
    fn manufacturer(&self) -> Option<Manufacturer> {
        self.manufacturer
    }
    fn year(&self) -> Option<Year> {
        self.year
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
}
