// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr, VariantArray};

use crate::{
    SubmissionIdentifier, SubmissionMetadata,
    input::{LcdScreen, Part, is_not_outlier},
    time::Month,
};

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmgConsole {
    pub slug: String,
    pub contributor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u16>,
    pub shell: DmgShell,
    pub mainboard: DmgMainboard,
    #[serde(skip_serializing_if = "DmgLcdBoard::is_unknown", default)]
    pub lcd_board: DmgLcdBoard,
    #[serde(skip_serializing_if = "DmgPowerBoard::is_unknown", default)]
    pub power_board: DmgPowerBoard,
    #[serde(skip_serializing_if = "DmgJackBoard::is_unknown", default)]
    pub jack_board: DmgJackBoard,
}

impl SubmissionMetadata for DmgConsole {
    type PhotoKind = DmgPhotoKind;

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
pub enum DmgPhotoKind {
    #[strum(serialize = "front")]
    Front,
    #[strum(serialize = "back")]
    Back,
    #[strum(serialize = "mainboard_front")]
    MainboardFront,
    #[strum(serialize = "mainboard_back")]
    MainboardBack,
    #[strum(serialize = "lcd_board_front")]
    LcdBoardFront,
    #[strum(serialize = "lcd_board_back")]
    LcdBoardBack,
    #[strum(serialize = "power_board_front")]
    PowerBoardFront,
    #[strum(serialize = "power_board_back")]
    PowerBoardBack,
    #[strum(serialize = "jack_board_front")]
    JackBoardFront,
    #[strum(serialize = "jack_board_back")]
    JackBoardBack,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmgShell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<DmgShellColor>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub serial: String,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Deserialize,
    Serialize,
    VariantArray,
    IntoStaticStr,
    EnumString,
)]
pub enum DmgShellColor {
    OffWhite,
    DeepBlack,
    GorgeousGreen,
    RadiantRed,
    VibrantYellow,
    HighTechTransparent,
    TraditionalWhite,
    CoolBlue,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmgMainboard {
    pub label: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub circled_letters: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub extra_label: String,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u1: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u2: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u3: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub u4: Part,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub x1: Part,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmgLcdBoard {
    pub label: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub stamp: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub circled_letters: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub extra_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Part::is_unknown", default)]
    pub chip: Part,
    #[serde(skip_serializing_if = "LcdScreen::is_unknown", default)]
    pub screen: LcdScreen,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

impl DmgLcdBoard {
    pub fn is_unknown(&self) -> bool {
        self == &DmgLcdBoard::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmgPowerBoard {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

impl DmgPowerBoard {
    pub fn is_unknown(&self) -> bool {
        self == &DmgPowerBoard::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmgJackBoard {
    pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub extra_label: String,
    #[serde(skip_serializing_if = "is_not_outlier", default)]
    pub outlier: bool,
}

impl DmgJackBoard {
    pub fn is_unknown(&self) -> bool {
        self == &DmgJackBoard::default()
    }
}
