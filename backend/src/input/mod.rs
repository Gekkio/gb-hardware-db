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
pub struct Chip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}

impl Chip {
    pub fn from_label(label: Option<String>) -> Chip {
        Chip {
            label,
            outlier: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LcdChip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ribbon_label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_not_outlier")]
    pub outlier: bool,
}
