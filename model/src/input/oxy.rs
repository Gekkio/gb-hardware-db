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
pub struct OxyConsole {
    pub slug: String,
    pub contributor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u16>,
    pub shell: OxyShell,
    pub mainboard: OxyMainboard,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OxyShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<OxyShellColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum OxyShellColor {
    Silver,
    Black,
    Purple,
    PearlBlue,
    Pink,
    Green,
    Blue,
    Red,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OxyMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
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
    pub x1: Option<Part>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
