use gbhwdb_backend::{
    input::{LcdChip, LcdScreen},
    parser::LabelParser,
};
use serde::Serialize;

use super::{to_legacy_year, HasDateCode, LegacyChip, LegacyPhoto};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgPhotos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mainboardFront")]
    pub mainboard_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mainboardBack")]
    pub mainboard_back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lcdBoardFront")]
    pub lcd_board_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lcdBoardBack")]
    pub lcd_board_back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "powerBoardFront")]
    pub power_board_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "powerBoardBack")]
    pub power_board_back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jackBoardFront")]
    pub jack_board_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "jackBoardBack")]
    pub jack_board_back: Option<LegacyPhoto>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyAgsPhotos {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pcbFront")]
    pub pcb_front: Option<LegacyPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pcbBack")]
    pub pcb_back: Option<LegacyPhoto>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    pub mainboard: LegacyDmgMainboard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_board: Option<LegacyDmgLcdBoard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_board: Option<LegacyDmgPowerBoard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jack_board: Option<LegacyDmgJackBoard>,
}

impl HasDateCode for LegacyDmgMetadata {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgLcdBoard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_panel: Option<LegacyLcdPanel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
}

impl HasDateCode for LegacyDmgLcdBoard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgPowerBoard {
    #[serde(rename = "type")]
    pub kind: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
}

impl HasDateCode for LegacyDmgPowerBoard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgJackBoard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_label: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySgbMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    pub mainboard: LegacySgbMainboard,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySgbMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_at_top_right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd2: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cic: Option<LegacyChip>,
}

impl HasDateCode for LegacySgbMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySgb2Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    pub mainboard: LegacySgb2Mainboard,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySgb2Mainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_at_top_right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd2: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cic: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coil: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

impl HasDateCode for LegacySgb2Mainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMgbMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    pub mainboard: LegacyMgbMainboard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_panel: Option<LegacyLcdPanel>,
}

impl HasDateCode for LegacyMgbMetadata {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMgbMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

impl HasDateCode for LegacyMgbMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMglMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<u8>,
    pub mainboard: LegacyMglMainboard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_panel: Option<LegacyLcdPanel>,
}

impl HasDateCode for LegacyMglMetadata {
    const YEAR: bool = true;
    const WEEK: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyMglMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<LegacyChip>,
}

impl HasDateCode for LegacyMglMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyCgbMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<u8>,
    pub mainboard: LegacyCgbMainboard,
}

impl HasDateCode for LegacyCgbMetadata {
    const YEAR: bool = true;
    const MONTH: bool = true;
    const WEEK: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyCgbMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

impl HasDateCode for LegacyCgbMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyAgbMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<u8>,
    pub mainboard: LegacyAgbMainboard,
}

impl HasDateCode for LegacyAgbMetadata {
    const YEAR: bool = true;
    const WEEK: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyAgbMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

impl HasDateCode for LegacyAgbMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyAgsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    pub mainboard: LegacyAgsMainboard,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyAgsMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplifier: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

impl HasDateCode for LegacyAgsMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyGbsMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<u8>,
    pub mainboard: LegacyGbsMainboard,
}

impl HasDateCode for LegacyGbsMetadata {
    const YEAR: bool = true;
    const WEEK: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn week(&self) -> Option<u8> {
        self.week
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyGbsMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_pair: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_front: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stamp_back: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_ram: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u6: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crystal: Option<LegacyChip>,
}

impl HasDateCode for LegacyGbsMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyOxyMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_code: Option<String>,
    pub mainboard: LegacyOxyMainboard,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyOxyMainboard {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circled_letters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u2: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u4: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u5: Option<LegacyChip>,
}

impl HasDateCode for LegacyOxyMainboard {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyLcdPanel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_driver: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_driver: Option<LegacyChip>,
}

impl HasDateCode for LegacyLcdPanel {
    const YEAR: bool = true;
    const MONTH: bool = true;

    fn year(&self) -> Option<u16> {
        self.year
    }
    fn month(&self) -> Option<u8> {
        self.month
    }
}

pub fn to_legacy_lcd_chip(year_hint: Option<u16>, chip: &LcdChip) -> LegacyChip {
    let ribbon_label = &chip.ribbon_label;
    if let Some(label) = &chip.label {
        let chip = gbhwdb_backend::parser::lcd_chip::lcd_chip()
            .parse(&label)
            .unwrap_or_else(|_| panic!("{}", label));
        LegacyChip {
            label: Some(match &ribbon_label {
                Some(ribbon_label) => format!("{} {}", ribbon_label, label),
                None => label.to_owned(),
            }),
            kind: ribbon_label.clone(),
            manufacturer: Some("Sharp".to_owned()),
            year: to_legacy_year(year_hint, chip.year),
            week: chip.week,
            month: chip.month,
            rom_code: None,
        }
    } else {
        LegacyChip {
            label: ribbon_label.clone(),
            kind: ribbon_label.clone(),
            manufacturer: Some("Sharp".to_owned()),
            ..LegacyChip::default()
        }
    }
}

pub fn to_legacy_lcd_panel(year_hint: Option<u16>, screen: &LcdScreen) -> Option<LegacyLcdPanel> {
    let column_driver = screen
        .column_driver
        .as_ref()
        .map(|chip| to_legacy_lcd_chip(year_hint, chip));
    let row_driver = screen
        .row_driver
        .as_ref()
        .map(|chip| to_legacy_lcd_chip(year_hint, chip));
    let label = screen.label.clone();
    let screen = screen.label.as_ref().map(|label| {
        gbhwdb_backend::parser::lcd_screen::lcd_screen()
            .parse(label)
            .unwrap_or_else(|_| panic!("{}", label))
    });
    Some(LegacyLcdPanel {
        label,
        year: screen
            .as_ref()
            .and_then(|screen| to_legacy_year(year_hint, screen.year)),
        month: screen.as_ref().and_then(|screen| screen.month),
        column_driver,
        row_driver,
    })
}
