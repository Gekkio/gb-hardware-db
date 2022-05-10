use serde::{Deserialize, Serialize};

use crate::{
    input::{is_not_outlier, Chip, LcdScreen},
    time::Month,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MglConsole {
    pub slug: String,
    pub contributor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u16>,
    pub shell: MglShell,
    pub mainboard: MglMainboard,
    pub screen: LcdScreen,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MglShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<MglShellColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MglShellColor {
    Gold,
    Silver,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MglMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
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
    pub x1: Option<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<Chip>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
