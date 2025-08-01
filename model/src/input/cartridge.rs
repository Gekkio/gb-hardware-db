// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::{ops::Index, str};
use time::Date;

use crate::parser::Manufacturer;
use crate::{
    ParseError, SubmissionIdentifier, SubmissionMetadata,
    config::cartridge::PartDesignator,
    hash::{Crc32, Md5, Sha1, Sha256},
    input::{Part, is_not_outlier},
    time::Month,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cartridge {
    pub code: String,
    pub slug: String,
    pub contributor: String,
    pub index: u16,
    pub shell: CartridgeShell,
    pub board: CartridgeBoard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dump: Option<CartridgeDump>,
}

impl SubmissionMetadata for Cartridge {
    type PhotoKind = CartridgePhotoKind;

    fn contributor(&self) -> &str {
        &self.contributor
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn identifier(&self) -> SubmissionIdentifier {
        SubmissionIdentifier::Index(self.index)
    }

    fn set_contributor(&mut self, contributor: &str) {
        self.contributor = contributor.to_string();
    }

    fn update_identifier(&mut self, contributor_slug: &str, index: u16) {
        self.slug = format!("{}-{}", contributor_slug, index);
        self.index = index;
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize,
    Serialize,
    strum::VariantArray,
    strum::EnumString,
    strum::IntoStaticStr,
)]
pub enum CartridgePhotoKind {
    #[strum(serialize = "front")]
    Front,
    #[strum(serialize = "pcb_front")]
    PcbFront,
    #[strum(serialize = "pcb_back")]
    PcbBack,
    #[strum(serialize = "without_battery")]
    WithoutBattery,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeShell {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub code: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp: String,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeBoard {
    pub label: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub circled_letters: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub panel_position: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u1: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u2: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u3: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u4: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u5: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u6: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u7: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub x1: Part,
    #[serde(skip_serializing_if = "CartridgeBattery::is_unknown", default)]
    pub battery: CartridgeBattery,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

impl Index<PartDesignator> for CartridgeBoard {
    type Output = Part;

    fn index(&self, index: PartDesignator) -> &Self::Output {
        match index {
            PartDesignator::U1 => &self.u1,
            PartDesignator::U2 => &self.u2,
            PartDesignator::U3 => &self.u3,
            PartDesignator::U4 => &self.u4,
            PartDesignator::U5 => &self.u5,
            PartDesignator::U6 => &self.u6,
            PartDesignator::U7 => &self.u7,
            PartDesignator::X1 => &self.x1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeBattery {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Manufacturer>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

impl CartridgeBattery {
    pub fn is_unknown(&self) -> bool {
        self == &CartridgeBattery::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeDump {
    pub tool: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub log: String,
    #[serde(with = "date_format")]
    pub date: Date,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32: Option<Crc32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<Md5>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<Sha1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<Sha256>,
}

impl std::error::Error for ParseError {}

mod date_format {
    use serde::{Deserializer, Serializer, de::Visitor};
    use time::{Date, format_description::FormatItem, macros::format_description};

    static DATE_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]");

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = date
            .format(DATE_FORMAT)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&string)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("date in YYYY-MM-DD format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Date::parse(v, DATE_FORMAT).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

#[test]
fn test_deserialize() {
    let cart: Cartridge = serde_json::from_str(
        r#"{
            "code": "DMG-ASDF-0",
            "slug": "dude-1",
            "contributor": "dude",
            "index": 1,
            "shell": {
                "code": "DMG-123",
                "stamp": "00A",
                "outlier": true
            },
            "board": {
                "label": "ASDF",
                "circled_letters": "M",
                "panel_position": "5",
                "year": 1999,
                "month": 11,
                "u1": {
                    "label": "U1",
                    "outlier": true
                },
                "u2": {
                    "label": "U2"
                },
                "u3": {
                    "label": "U3"
                },
                "u4": {
                    "label": "U4"
                },
                "u5": {
                    "label": "U5"
                },
                "u7": {},
                "x1": {
                    "label": "KDS"
                },
                "battery": {
                    "manufacturer": "Panasonic",
                    "label": "98-11"
                },
                "outlier": true
            },
            "dump": {
                "tool": "MeGa DumPer",
                "log": "Did the thing",
                "date": "1999-01-01",
                "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            }
        }"#,
    )
    .unwrap();
    assert_eq!(
        cart,
        Cartridge {
            code: "DMG-ASDF-0".to_owned(),
            slug: "dude-1".to_owned(),
            contributor: "dude".to_owned(),
            index: 1,
            shell: CartridgeShell {
                code: "DMG-123".to_owned(),
                stamp: "00A".to_owned(),
                outlier: true
            },
            board: CartridgeBoard {
                label: "ASDF".to_owned(),
                circled_letters: "M".to_owned(),
                panel_position: "5".to_owned(),
                year: Some(1999),
                month: Some(Month::November),
                u1: Part {
                    label: "U1".to_owned(),
                    outlier: true
                },
                u2: Part {
                    label: "U2".to_owned(),
                    outlier: false
                },
                u3: Part {
                    label: "U3".to_owned(),
                    outlier: false
                },
                u4: Part {
                    label: "U4".to_owned(),
                    outlier: false
                },
                u5: Part {
                    label: "U5".to_owned(),
                    outlier: false
                },
                u6: Part::default(),
                u7: Part::default(),
                x1: Part {
                    label: "KDS".to_owned(),
                    outlier: false,
                },
                battery: CartridgeBattery {
                    manufacturer: Some(Manufacturer::Panasonic),
                    label: "98-11".to_owned(),
                    outlier: false,
                },
                outlier: true
            },
            dump: Some(CartridgeDump {
                tool: "MeGa DumPer".to_owned(),
                log: "Did the thing".to_owned(),
                date: Date::from_calendar_date(1999, time::Month::January, 1).unwrap(),
                crc32: None,
                md5: None,
                sha1: None,
                sha256: Some(
                    Sha256::parse(
                        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                    )
                    .unwrap()
                ),
            })
        }
    );
}

#[test]
fn test_deserialize_minimal() {
    let cart: Cartridge = serde_json::from_str(
        r#"{
            "code": "DMG-ASDF-0",
            "slug": "dude-1",
            "contributor": "dude",
            "index": 1,
            "shell": {},
            "board": {
                "label": "ASDF"
            }
        }"#,
    )
    .unwrap();
    assert_eq!(
        cart,
        Cartridge {
            code: "DMG-ASDF-0".to_owned(),
            slug: "dude-1".to_owned(),
            contributor: "dude".to_owned(),
            index: 1,
            shell: CartridgeShell {
                code: "".to_owned(),
                stamp: "".to_owned(),
                outlier: false,
            },
            board: CartridgeBoard {
                label: "ASDF".to_owned(),
                circled_letters: "".to_owned(),
                panel_position: "".to_owned(),
                year: None,
                month: None,
                u1: Part::default(),
                u2: Part::default(),
                u3: Part::default(),
                u4: Part::default(),
                u5: Part::default(),
                u6: Part::default(),
                u7: Part::default(),
                x1: Part::default(),
                battery: CartridgeBattery::default(),
                outlier: false
            },
            dump: None,
        }
    )
}
