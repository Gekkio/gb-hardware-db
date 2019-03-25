use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    pub board: LegacyBoard,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyBoard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom2: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapper: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_protector: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_decoder: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eeprom: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerometer: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
    #[serde(skip_serializing)]
    pub battery: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyChip {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<u16>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySubmission {
    #[serde(rename = "type")]
    pub code: String,
    pub title: String,
    pub slug: String,
    pub contributor: String,
    pub metadata: LegacyMetadata,
    pub photos: LegacyPhotos,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyPhotos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pcbFront")]
    pub pcb_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pcbBack")]
    pub pcb_back: Option<LegacyPhoto>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyPhoto {
    pub path: String,
    pub name: String,
}
