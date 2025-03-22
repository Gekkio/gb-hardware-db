// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::{
    input::{LcdScreen, Part, is_not_outlier},
    time::{Jun, Month},
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MgbConsole {
    pub slug: String,
    pub contributor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u16>,
    pub shell: MgbShell,
    pub mainboard: MgbMainboard,
    #[serde(default)]
    #[serde(skip_serializing_if = "LcdScreen::is_unknown")]
    pub screen: LcdScreen,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MgbShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<MgbShellColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum MgbShellColor {
    Silver,
    Black,
    Green,
    Blue,
    Red,
    Yellow,
    Clear,
    Pink,
    ClearPurple,
    OffWhite,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MgbMainboard {
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
    pub jun: Option<Jun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u1: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u2: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u3: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x1: Option<Part>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
