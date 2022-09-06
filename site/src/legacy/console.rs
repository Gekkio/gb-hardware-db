// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{
    input::{LcdChip, LcdScreen},
    parser::LabelParser,
    time::{Jun, Month, Week},
    Console,
};
use serde::Serialize;

use super::{
    to_legacy_year, DateCode, HasDateCode, LegacyChip, LegacyMetadata, LegacyPhoto, LegacyPhotos,
    PhotoInfo, PhotoKind,
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct LegacyMainboard<'a> {
    pub kind: &'a str,
    pub date_code: DateCode,
    pub number_pair: Option<&'a str>,
    pub stamp: Option<&'a str>,
    pub stamp_front: Option<&'a str>,
    pub stamp_back: Option<&'a str>,
    pub circled_letters: Option<&'a str>,
    pub letter_at_top_right: Option<&'a str>,
    pub extra_label: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct LegacyConsoleShell<'a> {
    pub color: Option<&'a str>,
    pub release_code: Option<&'a str>,
    pub stamp: Option<&'a str>,
    pub date_code: DateCode,
}

pub trait LegacyConsoleMetadata: LegacyMetadata {
    const CONSOLE: Console;

    fn chips() -> Vec<ChipInfo<Self>>;
    fn shell(&self) -> LegacyConsoleShell;
    fn mainboard(&self) -> LegacyMainboard;
    fn lcd_panel(&self) -> Option<&LegacyLcdPanel> {
        None
    }
}

pub struct ChipInfo<M: ?Sized> {
    pub label: &'static str,
    pub designator: &'static str,
    pub hide_type: bool,
    pub getter: Box<dyn Fn(&M) -> Option<&LegacyChip>>,
}

impl<M: ?Sized> ChipInfo<M> {
    pub fn new(
        label: &'static str,
        designator: &'static str,
        getter: Box<dyn Fn(&M) -> Option<&LegacyChip>>,
    ) -> Self {
        ChipInfo {
            label,
            designator,
            hide_type: false,
            getter,
        }
    }
}

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

impl LegacyPhotos for LegacyDmgPhotos {
    fn infos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new(PhotoKind::MainUnit, "Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new(PhotoKind::MainUnit, "Back", Box::new(|p| p.back.as_ref())),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "Mainboard front",
                Box::new(|p| p.mainboard_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "Mainboard back",
                Box::new(|p| p.mainboard_back.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::Other,
                "LCD board front",
                Box::new(|p| p.lcd_board_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::Other,
                "LCD board back",
                Box::new(|p| p.lcd_board_back.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::Other,
                "Power board front",
                Box::new(|p| p.power_board_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::Other,
                "Power board back",
                Box::new(|p| p.power_board_back.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::Other,
                "Jack board front",
                Box::new(|p| p.jack_board_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::Other,
                "Jack board back",
                Box::new(|p| p.jack_board_back.as_ref()),
            ),
        ]
    }
    fn front(&self) -> Option<&LegacyPhoto> {
        self.front.as_ref()
    }
    fn photos(&self) -> Vec<&LegacyPhoto> {
        [
            &self.front,
            &self.back,
            &self.mainboard_front,
            &self.mainboard_back,
            &self.lcd_board_front,
            &self.lcd_board_back,
            &self.power_board_front,
            &self.power_board_back,
            &self.jack_board_front,
            &self.jack_board_back,
        ]
        .iter()
        .filter_map(|photo| photo.as_ref())
        .collect()
    }
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

impl LegacyPhotos for LegacyAgsPhotos {
    fn infos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new(PhotoKind::MainUnit, "Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new(PhotoKind::MainUnit, "Top", Box::new(|p| p.top.as_ref())),
            PhotoInfo::new(PhotoKind::MainUnit, "Back", Box::new(|p| p.back.as_ref())),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "PCB front",
                Box::new(|p| p.pcb_front.as_ref()),
            ),
            PhotoInfo::new(
                PhotoKind::MainBoard,
                "PCB back",
                Box::new(|p| p.pcb_back.as_ref()),
            ),
        ]
    }
    fn front(&self) -> Option<&LegacyPhoto> {
        self.front.as_ref()
    }
    fn photos(&self) -> Vec<&LegacyPhoto> {
        [
            &self.front,
            &self.top,
            &self.back,
            &self.pcb_front,
            &self.pcb_back,
        ]
        .iter()
        .filter_map(|photo| photo.as_ref())
        .collect()
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyDmgMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Month>,
    pub mainboard: LegacyDmgMainboard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_board: Option<LegacyDmgLcdBoard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_board: Option<LegacyDmgPowerBoard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jack_board: Option<LegacyDmgJackBoard>,
}

impl HasDateCode for LegacyDmgMainboard {
    fn date_code(&self) -> DateCode {
        DateCode::default()
    }
}

impl HasDateCode for LegacyDmgMetadata {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
    }
}

impl LegacyMetadata for LegacyDmgMetadata {
    const PLACEHOLDER_SVG: Option<&'static str> = Some("/dmg_placeholder.svg");
}

impl LegacyConsoleMetadata for LegacyDmgMetadata {
    const CONSOLE: Console = Console::Dmg;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("VRAM", "U2", Box::new(|m| m.mainboard.video_ram.as_ref())),
            ChipInfo::new("WRAM", "U3", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Audio amplifier",
                "U4",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo {
                label: "Crystal",
                designator: "X1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            date_code: self.date_code(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            stamp: self.mainboard.stamp.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            extra_label: self.mainboard.extra_label.as_deref(),
            ..LegacyMainboard::default()
        }
    }

    fn lcd_panel(&self) -> Option<&LegacyLcdPanel> {
        self.lcd_board
            .as_ref()
            .and_then(|board| board.lcd_panel.as_ref())
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
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_panel: Option<LegacyLcdPanel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulator: Option<LegacyChip>,
}

impl HasDateCode for LegacyDmgLcdBoard {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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
    pub month: Option<Month>,
}

impl HasDateCode for LegacyDmgPowerBoard {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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
    pub month: Option<Month>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
    }
}

impl LegacyMetadata for LegacySgbMetadata {}

impl LegacyConsoleMetadata for LegacySgbMetadata {
    const CONSOLE: Console = Console::Sgb;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("ICD2", "U2", Box::new(|m| m.mainboard.icd2.as_ref())),
            ChipInfo::new("WRAM", "U3", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new("VRAM", "U4", Box::new(|m| m.mainboard.video_ram.as_ref())),
            ChipInfo::new("ROM", "U5", Box::new(|m| m.mainboard.rom.as_ref())),
            ChipInfo::new("CIC", "U6", Box::new(|m| m.mainboard.cic.as_ref())),
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            stamp: self.stamp.as_deref(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            letter_at_top_right: self.mainboard.letter_at_top_right.as_deref(),
            ..LegacyMainboard::default()
        }
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
    pub month: Option<Month>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
    }
}

impl LegacyMetadata for LegacySgb2Metadata {}

impl LegacyConsoleMetadata for LegacySgb2Metadata {
    const CONSOLE: Console = Console::Sgb2;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("ICD2", "U2", Box::new(|m| m.mainboard.icd2.as_ref())),
            ChipInfo::new("WRAM", "U3", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new("ROM", "U4", Box::new(|m| m.mainboard.rom.as_ref())),
            ChipInfo::new("CIC", "U5", Box::new(|m| m.mainboard.cic.as_ref())),
            ChipInfo {
                label: "Crystal",
                designator: "XTAL1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
            ChipInfo::new("Coil", "COIL1", Box::new(|m| m.mainboard.coil.as_ref())),
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            stamp: self.stamp.as_deref(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            letter_at_top_right: self.mainboard.letter_at_top_right.as_deref(),
            ..LegacyMainboard::default()
        }
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
    pub month: Option<Month>,
    pub mainboard: LegacyMgbMainboard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_panel: Option<LegacyLcdPanel>,
}

impl HasDateCode for LegacyMgbMetadata {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
    }
}

impl LegacyMetadata for LegacyMgbMetadata {
    const PLACEHOLDER_SVG: Option<&'static str> = Some("/mgb_placeholder.svg");
}

impl LegacyConsoleMetadata for LegacyMgbMetadata {
    const CONSOLE: Console = Console::Mgb;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Audio amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new(
                "LCD bias generator",
                "U4",
                Box::new(|m| m.mainboard.regulator.as_ref()),
            ),
            ChipInfo {
                label: "Crystal",
                designator: "X1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            date_code: self.date_code(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            number_pair: self.mainboard.number_pair.as_deref(),
            stamp: self.mainboard.stamp.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
    }

    fn lcd_panel(&self) -> Option<&LegacyLcdPanel> {
        self.lcd_panel.as_ref()
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
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jun: Option<Jun>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: self.jun,
            week: None,
        }
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
    pub week: Option<Week>,
    pub mainboard: LegacyMglMainboard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcd_panel: Option<LegacyLcdPanel>,
}

impl HasDateCode for LegacyMglMetadata {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: None,
            jun: None,
            week: self.week,
        }
    }
}

impl LegacyMetadata for LegacyMglMetadata {
    const PLACEHOLDER_SVG: Option<&'static str> = Some("/mgl_placeholder.svg");
}

impl LegacyConsoleMetadata for LegacyMglMetadata {
    const CONSOLE: Console = Console::Mgl;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Audio amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new(
                "LCD bias generator",
                "U4",
                Box::new(|m| m.mainboard.regulator.as_ref()),
            ),
            ChipInfo {
                label: "Crystal",
                designator: "X1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
            ChipInfo::new("Transformer", "T1", Box::new(|m| m.mainboard.t1.as_ref())),
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            date_code: self.date_code(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            number_pair: self.mainboard.number_pair.as_deref(),
            stamp: self.mainboard.stamp.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
    }

    fn lcd_panel(&self) -> Option<&LegacyLcdPanel> {
        self.lcd_panel.as_ref()
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
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jun: Option<Jun>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: self.jun,
            week: None,
        }
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
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<Week>,
    pub mainboard: LegacyCgbMainboard,
}

impl HasDateCode for LegacyCgbMetadata {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: self.week,
        }
    }
}

impl LegacyMetadata for LegacyCgbMetadata {}

impl LegacyConsoleMetadata for LegacyCgbMetadata {
    const CONSOLE: Console = Console::Cgb;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Audio amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new(
                "LCD bias generator",
                "U4",
                Box::new(|m| m.mainboard.regulator.as_ref()),
            ),
            ChipInfo {
                label: "Crystal",
                designator: "X1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            date_code: self.date_code(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            number_pair: self.mainboard.number_pair.as_deref(),
            stamp: self.mainboard.stamp.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
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
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jun: Option<Jun>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: self.jun,
            week: None,
        }
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
    pub week: Option<Week>,
    pub mainboard: LegacyAgbMainboard,
}

impl HasDateCode for LegacyAgbMetadata {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: None,
            jun: None,
            week: self.week,
        }
    }
}

impl LegacyMetadata for LegacyAgbMetadata {}

impl LegacyConsoleMetadata for LegacyAgbMetadata {
    const CONSOLE: Console = Console::Agb;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "LCD bias generator",
                "U3",
                Box::new(|m| m.mainboard.regulator.as_ref()),
            ),
            ChipInfo::new("?", "U4", Box::new(|m| m.mainboard.u4.as_ref())),
            ChipInfo::new(
                "Audio amplifier",
                "U6",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo {
                label: "Crystal",
                designator: "X1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            date_code: self.date_code(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            number_pair: self.mainboard.number_pair.as_deref(),
            stamp: self.mainboard.stamp.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
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
    pub month: Option<Month>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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

impl LegacyMetadata for LegacyAgsMetadata {}

impl LegacyConsoleMetadata for LegacyAgsMetadata {
    const CONSOLE: Console = Console::Ags;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new("?", "U4", Box::new(|m| m.mainboard.u4.as_ref())),
            ChipInfo::new(
                "Battery controller",
                "U5",
                Box::new(|m| m.mainboard.u5.as_ref()),
            ),
            ChipInfo {
                label: "Crystal",
                designator: "X1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            number_pair: self.mainboard.number_pair.as_deref(),
            stamp: self.mainboard.stamp.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
    }
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
    pub month: Option<Month>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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
    pub week: Option<Week>,
    pub mainboard: LegacyGbsMainboard,
}

impl HasDateCode for LegacyGbsMetadata {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: None,
            jun: None,
            week: self.week,
        }
    }
}

impl LegacyMetadata for LegacyGbsMetadata {}

impl LegacyConsoleMetadata for LegacyGbsMetadata {
    const CONSOLE: Console = Console::Gbs;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new("?", "U4", Box::new(|m| m.mainboard.u4.as_ref())),
            ChipInfo::new("Regulator", "U5", Box::new(|m| m.mainboard.u5.as_ref())),
            ChipInfo::new("Regulator", "U6", Box::new(|m| m.mainboard.u5.as_ref())),
            ChipInfo {
                label: "Crystal",
                designator: "Y1",
                hide_type: true,
                getter: Box::new(|m| m.mainboard.crystal.as_ref()),
            },
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            date_code: self.date_code(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            number_pair: self.mainboard.number_pair.as_deref(),
            stamp: self.mainboard.stamp.as_deref(),
            stamp_front: self.mainboard.stamp_front.as_deref(),
            stamp_back: self.mainboard.stamp_back.as_deref(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
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
    pub month: Option<Month>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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

impl LegacyMetadata for LegacyOxyMetadata {}

impl LegacyConsoleMetadata for LegacyOxyMetadata {
    const CONSOLE: Console = Console::Oxy;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("?", "U2", Box::new(|m| m.mainboard.u2.as_ref())),
            ChipInfo::new("?", "U4", Box::new(|m| m.mainboard.u4.as_ref())),
            ChipInfo::new("?", "U5", Box::new(|m| m.mainboard.u5.as_ref())),
        ]
    }

    fn shell(&self) -> LegacyConsoleShell {
        LegacyConsoleShell {
            color: self.color.as_deref(),
            release_code: self.release_code.as_deref(),
            ..LegacyConsoleShell::default()
        }
    }

    fn mainboard(&self) -> LegacyMainboard {
        LegacyMainboard {
            kind: &self.mainboard.kind,
            date_code: self.mainboard.date_code(),
            circled_letters: self.mainboard.circled_letters.as_deref(),
            ..LegacyMainboard::default()
        }
    }
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
    pub month: Option<Month>,
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
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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
    pub month: Option<Month>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_driver: Option<LegacyChip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_driver: Option<LegacyChip>,
}

impl HasDateCode for LegacyLcdPanel {
    fn date_code(&self) -> DateCode {
        DateCode {
            year: self.year,
            month: self.month,
            jun: None,
            week: None,
        }
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
