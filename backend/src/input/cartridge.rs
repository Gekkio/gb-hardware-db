// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::{ops::Index, str};
use time::Date;

use crate::{
    config::cartridge::PartDesignator,
    input::{is_not_outlier, Part},
    sha256::Sha256,
    time::Month,
    ParseError,
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

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeBoard {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u1: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u2: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u3: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u6: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u7: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1: Option<Part>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

impl Index<PartDesignator> for CartridgeBoard {
    type Output = Option<Part>;

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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeDump {
    pub tool: String,
    #[serde(default)]
    pub log: String,
    #[serde(with = "date_format")]
    pub date: Date,
    pub sha256: Sha256,
}

impl std::error::Error for ParseError {}

mod date_format {
    use serde::{de::Visitor, Deserializer, Serializer};
    use time::{format_description::FormatItem, macros::format_description, Date};

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
                "outlier": true
            },
            "dump": {
                "tool": "MeGa DumPer",
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
                code: Some("DMG-123".to_owned()),
                stamp: Some("00A".to_owned()),
                outlier: true
            },
            board: CartridgeBoard {
                label: "ASDF".to_owned(),
                circled_letters: Some("M".to_owned()),
                panel_position: Some("5".to_owned()),
                year: Some(1999),
                month: Some(Month::November),
                u1: Some(Part {
                    label: Some("U1".to_owned()),
                    outlier: true
                }),
                u2: Some(Part {
                    label: Some("U2".to_owned()),
                    outlier: false
                }),
                u3: Some(Part {
                    label: Some("U3".to_owned()),
                    outlier: false
                }),
                u4: Some(Part {
                    label: Some("U4".to_owned()),
                    outlier: false
                }),
                u5: Some(Part {
                    label: Some("U5".to_owned()),
                    outlier: false
                }),
                u6: None,
                u7: Some(Part {
                    label: None,
                    outlier: false
                }),
                x1: Some(Part {
                    label: Some("KDS".to_owned()),
                    outlier: false,
                }),
                outlier: true
            },
            dump: Some(CartridgeDump {
                tool: "MeGa DumPer".to_owned(),
                date: Date::from_calendar_date(1999, time::Month::January, 1).unwrap(),
                sha256: Sha256::parse(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                )
                .unwrap(),
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
                code: None,
                stamp: None,
                outlier: false,
            },
            board: CartridgeBoard {
                label: "ASDF".to_owned(),
                circled_letters: None,
                panel_position: None,
                year: None,
                month: None,
                u1: None,
                u2: None,
                u3: None,
                u4: None,
                u5: None,
                u6: None,
                u7: None,
                x1: None,
                outlier: false
            },
            dump: None,
        }
    )
}
