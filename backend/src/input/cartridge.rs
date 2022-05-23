use serde::{de::Visitor, Deserialize, Serialize};
use std::{fmt, str};
use time::Date;

use crate::{
    input::{is_not_outlier, Chip},
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
    pub extra_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u1: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u2: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u3: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u6: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u7: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1: Option<Chip>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CartridgeDump {
    pub tool: String,
    #[serde(with = "date_format")]
    pub date: Date,
    pub sha256: Sha256,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sha256([u8; 32]);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for ParseError {}

impl Sha256 {
    pub fn parse(text: &str) -> Result<Sha256, ParseError> {
        if text.len() != 64 {
            return Err(ParseError("invalid SHA256"));
        }
        let bytes = text.as_bytes().chunks(2).map(|chunk| {
            let string = str::from_utf8(chunk).ok()?;
            u8::from_str_radix(string, 16).ok()
        });
        let mut result = Sha256([0; 32]);
        for (byte, parsed) in result.0.iter_mut().zip(bytes) {
            *byte = parsed.ok_or(ParseError("invalid SHA256"))?;
        }
        Ok(result)
    }
}

impl fmt::Display for Sha256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Serialize for Sha256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl<'de> Deserialize<'de> for Sha256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Sha256Visitor;

        impl<'de> Visitor<'de> for Sha256Visitor {
            type Value = Sha256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("hex-formatted SHA256")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Sha256::parse(v).map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_str(Sha256Visitor)
    }
}

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
                "extra_label": "5",
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
                extra_label: Some("5".to_owned()),
                year: Some(1999),
                month: Some(Month::November),
                u1: Some(Chip {
                    label: Some("U1".to_owned()),
                    outlier: true
                }),
                u2: Some(Chip {
                    label: Some("U2".to_owned()),
                    outlier: false
                }),
                u3: Some(Chip {
                    label: Some("U3".to_owned()),
                    outlier: false
                }),
                u4: Some(Chip {
                    label: Some("U4".to_owned()),
                    outlier: false
                }),
                u5: Some(Chip {
                    label: Some("U5".to_owned()),
                    outlier: false
                }),
                u6: None,
                u7: Some(Chip {
                    label: None,
                    outlier: false
                }),
                x1: Some(Chip {
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
                extra_label: None,
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
