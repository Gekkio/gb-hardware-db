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
pub struct SgbConsole {
    pub slug: String,
    pub contributor: String,
    pub index: u16,
    pub shell: SgbShell,
    pub mainboard: SgbMainboard,
}

pub type SgbPhotoKind = DefaultPhotoKind;

impl SubmissionMetadata for SgbConsole {
    type PhotoKind = SgbPhotoKind;

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
pub struct SgbShell {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp: String,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SgbMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub letter_at_top_right: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub circled_letters: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u1: Part,
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
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}
