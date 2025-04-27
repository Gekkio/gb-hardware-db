// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use super::DefaultPhotoKind;
use crate::{
    SubmissionIdentifier, SubmissionMetadata,
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

pub type GbsPhotoKind = DefaultPhotoKind;

impl SubmissionMetadata for GbsConsole {
    type PhotoKind = GbsPhotoKind;

    fn contributor(&self) -> &str {
        &self.contributor
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn identifier(&self) -> SubmissionIdentifier {
        SubmissionIdentifier::Index(self.index)
    }

    fn set_contributor(&mut self, contributor: &str) {
        self.contributor = contributor.to_string();
    }

    fn update_identifier(&mut self, contributor_slug: &str, index: u16) {
        self.slug = format!("{}-{}", contributor_slug, index);
        self.index = index;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GbsShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<GbsShellColor>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub release_code: String,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
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
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub number_pair: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp_front: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp_back: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub circled_letters: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u2: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u3: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u4: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u5: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u6: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub y1: Part,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}
