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
pub struct SgbConsole {
    pub slug: String,
    pub contributor: String,
    pub index: u16,
    pub shell: SgbShell,
    pub mainboard: SgbMainboard,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SgbShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SgbMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_at_top_right: Option<String>,
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
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
