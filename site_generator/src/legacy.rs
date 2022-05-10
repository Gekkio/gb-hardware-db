use gbhwdb_backend::{
    parser::*,
    time::{Month, Week},
};
use serde::{Deserialize, Serialize};

use crate::format::{calendar, calendar_short};

use self::console::{
    LegacyAgbMetadata, LegacyAgsMetadata, LegacyAgsPhotos, LegacyCgbMetadata, LegacyDmgMetadata,
    LegacyDmgPhotos, LegacyGbsMetadata, LegacyMgbMetadata, LegacyMglMetadata, LegacyOxyMetadata,
    LegacySgb2Metadata, LegacySgbMetadata,
};

pub mod cartridge;
pub mod chip;
pub mod console;

pub type LegacyCartridgeSubmission = LegacySubmission<cartridge::LegacyMetadata, LegacyPhotos>;
pub type LegacyDmgSubmission = LegacySubmission<LegacyDmgMetadata, LegacyDmgPhotos>;
pub type LegacySgbSubmission = LegacySubmission<LegacySgbMetadata, LegacyPhotos>;
pub type LegacyMgbSubmission = LegacySubmission<LegacyMgbMetadata, LegacyPhotos>;
pub type LegacyMglSubmission = LegacySubmission<LegacyMglMetadata, LegacyPhotos>;
pub type LegacySgb2Submission = LegacySubmission<LegacySgb2Metadata, LegacyPhotos>;
pub type LegacyCgbSubmission = LegacySubmission<LegacyCgbMetadata, LegacyPhotos>;
pub type LegacyAgbSubmission = LegacySubmission<LegacyAgbMetadata, LegacyPhotos>;
pub type LegacyAgsSubmission = LegacySubmission<LegacyAgsMetadata, LegacyAgsPhotos>;
pub type LegacyGbsSubmission = LegacySubmission<LegacyGbsMetadata, LegacyPhotos>;
pub type LegacyOxySubmission = LegacySubmission<LegacyOxyMetadata, LegacyPhotos>;

pub trait HasDateCode {
    const YEAR: bool = false;
    const MONTH: bool = false;
    const WEEK: bool = false;
    fn year(&self) -> Option<u16> {
        None
    }
    fn month(&self) -> Option<Month> {
        None
    }
    fn week(&self) -> Option<Week> {
        None
    }
    fn calendar_short(&self) -> Option<String> {
        Some(calendar_short(self.year(), self.month(), self.week())).filter(|text| !text.is_empty())
    }
    fn calendar(&self) -> Option<String> {
        Some(calendar(self.year(), self.month(), self.week())).filter(|text| !text.is_empty())
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySubmission<M, P> {
    #[serde(rename = "type")]
    pub code: String,
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_group: Option<String>,
    pub contributor: String,
    pub metadata: M,
    pub photos: P,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyChip {
    pub kind: Option<String>,
    pub label: Option<String>,
    pub manufacturer: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub week: Option<Week>,
    pub rom_code: Option<String>,
}

impl HasDateCode for LegacyChip {
    const YEAR: bool = true;
    const MONTH: bool = true;
    const WEEK: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<Month> {
        self.month
    }
    fn week(&self) -> Option<Week> {
        self.week
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyPhotos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pcbFront")]
    pub pcb_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pcbBack")]
    pub pcb_back: Option<LegacyPhoto>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyPhoto {
    pub path: String,
    pub name: String,
}

pub fn to_legacy_manufacturer(manufacturer: Option<Manufacturer>) -> Option<String> {
    manufacturer.map(|manufacturer| {
        (match manufacturer {
            Manufacturer::Amic => "AMIC Technology",
            Manufacturer::Analog => "Analog Devices",
            Manufacturer::AtT => "AT&T Technologies",
            Manufacturer::Bsi => "BSI",
            Manufacturer::Crosslink => "Crosslink Semiconductor",
            Manufacturer::Fujitsu => "Fujitsu",
            Manufacturer::Hudson => "Hudson",
            Manufacturer::Hynix => "Hynix",
            Manufacturer::Hyundai => "Hyundai",
            Manufacturer::Kds => "Daishinku",
            Manufacturer::Kinseki => "Kinseki",
            Manufacturer::Lgs => "Lucky GoldStar",
            Manufacturer::LsiLogic => "LSI Logic",
            Manufacturer::Macronix => "Macronix",
            Manufacturer::Mitsubishi => "Mitsubishi",
            Manufacturer::Mitsumi => "Mitsumi",
            Manufacturer::MoselVitelic => "Mosel-Vitelic",
            Manufacturer::Motorola => "Motorola",
            Manufacturer::Nec => "NEC",
            Manufacturer::Oki => "OKI",
            Manufacturer::Rohm => "ROHM",
            Manufacturer::Samsung => "Samsung",
            Manufacturer::Sanyo => "Sanyo",
            Manufacturer::Sharp => "Sharp",
            Manufacturer::Smsc => "Standard Microsystems Corporation",
            Manufacturer::StMicro => "STMicroelectronics",
            Manufacturer::Tdk => "TDK",
            Manufacturer::TexasInstruments => "Texas Instruments",
            Manufacturer::Toshiba => "Toshiba",
            Manufacturer::Victronix => "Victronix",
            Manufacturer::Winbond => "Winbond",
        })
        .to_owned()
    })
}

pub fn to_legacy_year(year_hint: Option<u16>, chip_year: Option<Year>) -> Option<u16> {
    (match (year_hint, chip_year) {
        (_, Some(Year::Full(year))) => Some(year),
        (Some(year_hint), Some(Year::Partial(year))) => Some(guess_full_year(year_hint, year)),
        _ => None,
    })
    .map(|year| {
        assert!(year >= 1988 && year < 2010);
        year
    })
}

pub fn guess_full_year(hint: u16, partial_year: u8) -> u16 {
    let decades = [1980, 1990, 2000];
    decades
        .iter()
        .map(|decade| (decade, (hint as i32 - (decade + partial_year as i32)).abs()))
        .min_by_key(|&(_, distance)| distance)
        .map(|(&decade, _)| decade as u16 + partial_year as u16)
        .unwrap_or(0)
}

#[test]
fn test_guess_full_year() {
    assert_eq!(1992, guess_full_year(1992, 2));
    assert_eq!(1989, guess_full_year(1989, 9));
    assert_eq!(1999, guess_full_year(1998, 9));
}
