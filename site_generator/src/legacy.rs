use gbhwdb_backend::parser::*;
use serde::{Deserialize, Serialize};

pub mod cartridge;
pub mod console;

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
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<u8>,
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
            Manufacturer::Analog => "analog",
            Manufacturer::AtT => "at_t",
            Manufacturer::Bsi => "bsi",
            Manufacturer::Fujitsu => "fujitsu",
            Manufacturer::Hudson => "hudson",
            Manufacturer::Hyundai => "hyundai",
            Manufacturer::Kds => "kds",
            Manufacturer::Lgs => "lgs",
            Manufacturer::LsiLogic => "lsi-logic",
            Manufacturer::Macronix => "macronix",
            Manufacturer::Mitsubishi => "mitsubishi",
            Manufacturer::Mitsumi => "mitsumi",
            Manufacturer::MoselVitelic => "mosel-vitelic",
            Manufacturer::Motorola => "motorola",
            Manufacturer::Nec => "nec",
            Manufacturer::Oki => "oki",
            Manufacturer::Rohm => "rohm",
            Manufacturer::Samsung => "samsung",
            Manufacturer::Sanyo => "sanyo",
            Manufacturer::Sharp => "sharp",
            Manufacturer::Smsc => "smsc",
            Manufacturer::TexasInstruments => "texas-instruments",
            Manufacturer::Toshiba => "toshiba",
            Manufacturer::Victronix => "victronix",
            Manufacturer::Winbond => "winbond",
        })
        .to_owned()
    })
}

pub fn to_legacy_year(year_hint: Option<u16>, chip_year: Option<Year>) -> Option<u16> {
    match (year_hint, chip_year) {
        (_, Some(Year::Full(year))) => Some(year),
        (Some(year_hint), Some(Year::Partial(year))) => {
            let adjust_80 = (
                (year_hint as i32 - (1980 + year as i32)).abs(),
                1980 + year as u16,
            );
            let adjust_90 = (
                (year_hint as i32 - (1990 + year as i32)).abs(),
                1990 + year as u16,
            );
            let adjust_00 = (
                (year_hint as i32 - (2000 + year as i32)).abs(),
                2000 + year as u16,
            );
            let candidates = [adjust_80, adjust_90, adjust_00];
            candidates
                .iter()
                .min_by_key(|(score, _)| score)
                .map(|&(_, year)| {
                    assert!(year >= 1989 && year < 2010);
                    year
                })
        }
        _ => None,
    }
}

#[test]
fn test_to_legacy_year() {
    assert_eq!(
        to_legacy_year(Some(1992), Some(Year::Partial(2))),
        Some(1992)
    );
    assert_eq!(
        to_legacy_year(Some(1989), Some(Year::Partial(9))),
        Some(1989)
    );
    assert_eq!(
        to_legacy_year(Some(1998), Some(Year::Partial(9))),
        Some(1999)
    );
}
