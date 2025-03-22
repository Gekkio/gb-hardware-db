// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

pub mod agb;
pub mod ags;
pub mod cartridge;
pub mod cgb;
pub mod dmg;
pub mod gbs;
pub mod mgb;
pub mod mgl;
pub mod oxy;
pub mod sgb;
pub mod sgb2;

pub(crate) fn is_not_outlier(outlier: &bool) -> bool {
    !outlier
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Part {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

impl Part {
    pub fn is_unknown(&self) -> bool {
        self == &Part::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcdChip {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ribbon_label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

impl LcdChip {
    pub fn is_unknown(&self) -> bool {
        self == &LcdChip::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcdScreen {
    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "LcdChip::is_unknown")]
    pub column_driver: LcdChip,
    #[serde(default)]
    #[serde(skip_serializing_if = "LcdChip::is_unknown")]
    pub row_driver: LcdChip,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

impl LcdScreen {
    pub fn is_unknown(&self) -> bool {
        self == &LcdScreen::default()
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
pub enum PhotoKind {
    #[strum(serialize = "front")]
    Front,
    #[strum(serialize = "back")]
    Back,
    #[strum(serialize = "pcb_front")]
    PcbFront,
    #[strum(serialize = "pcb_back")]
    PcbBack,
}
