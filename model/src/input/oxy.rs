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
pub struct OxyConsole {
    pub slug: String,
    pub contributor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u16>,
    pub shell: OxyShell,
    pub mainboard: OxyMainboard,
}

pub type OxyPhotoKind = DefaultPhotoKind;

impl SubmissionMetadata for OxyConsole {
    type PhotoKind = OxyPhotoKind;

    fn contributor(&self) -> &str {
        &self.contributor
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn identifier(&self) -> SubmissionIdentifier {
        SubmissionIdentifier::new(&self.shell.serial, self.index)
    }

    fn set_contributor(&mut self, contributor: &str) {
        self.contributor = contributor.to_string();
    }

    fn update_identifier(&mut self, contributor_slug: &str, index: u16) {
        if self.shell.serial.is_empty() {
            self.slug = format!("{}-{}", contributor_slug, index);
            self.index = Some(index as u16);
        } else {
            self.slug = self.shell.serial.clone();
            self.index = None;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OxyShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<OxyShellColor>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub release_code: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub serial: String,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
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
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub x1: Part,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}
