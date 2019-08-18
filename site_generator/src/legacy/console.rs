use serde::Serialize;

use super::{to_legacy_year, LegacyChip, LegacyPhoto};
use gbhwdb_backend::input::{LcdChip, LcdScreen};

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
    #[serde(rename = "type")]
    pub kind: String,
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
    pub column_driver: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_driver: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
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
    #[serde(rename = "type")]
    pub kind: String,
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

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacySgb2Metadata {
    #[serde(rename = "type")]
    pub kind: String,
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

pub fn to_legacy_lcd_chip(year_hint: Option<u16>, chip: &LcdChip) -> LegacyChip {
    let ribbon_label = &chip.ribbon_label;
    if let Some(label) = &chip.label {
        let chip =
            gbhwdb_backend::parser::parse_lcd_chip(&label).unwrap_or_else(|_| panic!("{}", label));
        LegacyChip {
            label: Some(match &ribbon_label {
                Some(ribbon_label) => format!("{} {}", ribbon_label, label),
                None => label.clone(),
            }),
            kind: ribbon_label.clone(),
            manufacturer: Some("sharp".to_owned()),
            year: to_legacy_year(year_hint, chip.year),
            week: chip.week,
            month: chip.month,
        }
    } else {
        LegacyChip {
            label: ribbon_label.clone(),
            kind: ribbon_label.clone(),
            manufacturer: Some("sharp".to_owned()),
            ..LegacyChip::default()
        }
    }
}

pub fn to_legacy_lcd_panel(screen: &LcdScreen) -> Option<LegacyLcdPanel> {
    screen.label.as_ref().map(|label| {
        let screen =
            gbhwdb_backend::parser::parse_lcd_screen(label).unwrap_or_else(|_| panic!("{}", label));
        LegacyLcdPanel {
            label: Some(label.clone()),
            year: screen.year,
            month: screen.month,
        }
    })
}
