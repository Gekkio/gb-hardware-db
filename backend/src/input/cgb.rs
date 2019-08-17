use serde::{Deserialize, Serialize};

use crate::input::{is_not_outlier, Chip};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CgbConsole {
    pub slug: String,
    pub contributor: String,
    pub index: u16,
    pub shell: CgbShell,
    pub mainboard: CgbMainboard,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CgbShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CgbMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
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
    pub x1: Option<Chip>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
