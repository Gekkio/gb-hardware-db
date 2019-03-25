use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Cartridge {
    pub code: String,
    pub slug: String,
    pub contributor: String,
    pub index: u16,
    pub shell: CartridgeShell,
    pub board: CartridgeBoard,
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
    pub year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u32>,
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

fn is_not_outlier(outlier: &bool) -> bool {
    !outlier
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Chip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
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
                month: Some(11),
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
            }
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
            }
        }
    )
}
