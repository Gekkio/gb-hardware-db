// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::{
    input::{Part, is_not_outlier},
    time::Month,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GbsConsole {
    pub slug: String,
    pub contributor: String,
    pub index: u16,
    pub shell: GbsShell,
    pub mainboard: GbsMainboard,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GbsShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<GbsShellColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum GbsShellColor {
    Indigo,
    Black,
    Spice,
    Platinum,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GbsMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_front: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_back: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
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
    pub y1: Option<Part>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
