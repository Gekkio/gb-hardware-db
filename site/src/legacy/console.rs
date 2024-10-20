// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{
    input::{LcdChip, LcdScreen},
    parser::{LabelParser, Manufacturer},
    time::{Jun, Month, Week},
    Console,
};

use crate::{
    legacy::{HasDateCode, LegacyMetadata, LegacyPhoto, LegacyPhotos, PhotoInfo, PhotoKind},
    process::part::ProcessedPart,
    process::{to_full_year, DateCode},
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

    fn parts() -> Vec<ChipInfo<Self>>;
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
    pub getter: Box<dyn Fn(&M) -> Option<&ProcessedPart>>,
}

impl<M: ?Sized> ChipInfo<M> {
    pub fn new(
        label: &'static str,
        designator: &'static str,
        getter: Box<dyn Fn(&M) -> Option<&ProcessedPart>>,
    ) -> Self {
        ChipInfo {
            label,
            designator,
            hide_type: false,
            getter,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LegacyDmgPhotos {
    pub front: Option<LegacyPhoto>,
    pub back: Option<LegacyPhoto>,
    pub mainboard_front: Option<LegacyPhoto>,
    pub mainboard_back: Option<LegacyPhoto>,
    pub lcd_board_front: Option<LegacyPhoto>,
    pub lcd_board_back: Option<LegacyPhoto>,
    pub power_board_front: Option<LegacyPhoto>,
    pub power_board_back: Option<LegacyPhoto>,
    pub jack_board_front: Option<LegacyPhoto>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyAgsPhotos {
    pub front: Option<LegacyPhoto>,
    pub top: Option<LegacyPhoto>,
    pub back: Option<LegacyPhoto>,
    pub pcb_front: Option<LegacyPhoto>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyDmgMetadata {
    pub color: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub mainboard: LegacyDmgMainboard,
    pub lcd_board: Option<LegacyDmgLcdBoard>,
    pub power_board: Option<LegacyDmgPowerBoard>,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyDmgMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub extra_label: Option<String>,
    pub stamp: Option<String>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub video_ram: Option<ProcessedPart>,
    pub amplifier: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
}

#[derive(Clone, Debug, Default)]
pub struct LegacyDmgLcdBoard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub stamp: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub lcd_panel: Option<LegacyLcdPanel>,
    pub regulator: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyDmgPowerBoard {
    pub kind: String,
    pub label: String,
    pub year: Option<u16>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyDmgJackBoard {
    pub kind: String,
    pub extra_label: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LegacySgbMetadata {
    pub stamp: Option<String>,
    pub mainboard: LegacySgbMainboard,
}

#[derive(Clone, Debug, Default)]
pub struct LegacySgbMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub letter_at_top_right: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub cpu: Option<ProcessedPart>,
    pub icd2: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub video_ram: Option<ProcessedPart>,
    pub rom: Option<ProcessedPart>,
    pub cic: Option<ProcessedPart>,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacySgb2Metadata {
    pub stamp: Option<String>,
    pub mainboard: LegacySgb2Mainboard,
}

#[derive(Clone, Debug, Default)]
pub struct LegacySgb2Mainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub letter_at_top_right: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub cpu: Option<ProcessedPart>,
    pub icd2: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub rom: Option<ProcessedPart>,
    pub cic: Option<ProcessedPart>,
    pub coil: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyMgbMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub mainboard: LegacyMgbMainboard,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyMgbMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub number_pair: Option<String>,
    pub stamp: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub jun: Option<Jun>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub amplifier: Option<ProcessedPart>,
    pub regulator: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyMglMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub year: Option<u16>,
    pub week: Option<Week>,
    pub mainboard: LegacyMglMainboard,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyMglMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub number_pair: Option<String>,
    pub stamp: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub jun: Option<Jun>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub amplifier: Option<ProcessedPart>,
    pub regulator: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
    pub t1: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyCgbMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyCgbMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub number_pair: Option<String>,
    pub stamp: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub jun: Option<Jun>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub amplifier: Option<ProcessedPart>,
    pub regulator: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyAgbMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub year: Option<u16>,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyAgbMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub number_pair: Option<String>,
    pub stamp: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub amplifier: Option<ProcessedPart>,
    pub regulator: Option<ProcessedPart>,
    pub u4: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyAgsMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub mainboard: LegacyAgsMainboard,
}

impl LegacyMetadata for LegacyAgsMetadata {}

impl LegacyConsoleMetadata for LegacyAgsMetadata {
    const CONSOLE: Console = Console::Ags;

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyAgsMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub number_pair: Option<String>,
    pub stamp: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub amplifier: Option<ProcessedPart>,
    pub u4: Option<ProcessedPart>,
    pub u5: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyGbsMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub year: Option<u16>,
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

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyGbsMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub number_pair: Option<String>,
    pub stamp: Option<String>,
    pub stamp_front: Option<String>,
    pub stamp_back: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub cpu: Option<ProcessedPart>,
    pub work_ram: Option<ProcessedPart>,
    pub u4: Option<ProcessedPart>,
    pub u5: Option<ProcessedPart>,
    pub u6: Option<ProcessedPart>,
    pub crystal: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyOxyMetadata {
    pub color: Option<String>,
    pub release_code: Option<String>,
    pub mainboard: LegacyOxyMainboard,
}

impl LegacyMetadata for LegacyOxyMetadata {}

impl LegacyConsoleMetadata for LegacyOxyMetadata {
    const CONSOLE: Console = Console::Oxy;

    fn parts() -> Vec<ChipInfo<Self>> {
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

#[derive(Clone, Debug, Default)]
pub struct LegacyOxyMainboard {
    pub kind: String,
    pub circled_letters: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub cpu: Option<ProcessedPart>,
    pub u2: Option<ProcessedPart>,
    pub u4: Option<ProcessedPart>,
    pub u5: Option<ProcessedPart>,
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

#[derive(Clone, Debug, Default)]
pub struct LegacyLcdPanel {
    pub label: Option<String>,
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub column_driver: Option<ProcessedPart>,
    pub row_driver: Option<ProcessedPart>,
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

pub fn to_legacy_lcd_chip(year_hint: Option<u16>, chip: &LcdChip) -> ProcessedPart {
    let ribbon_label = &chip.ribbon_label;
    if let Some(label) = &chip.label {
        let chip = gbhwdb_backend::parser::lcd_chip::lcd_chip()
            .parse(label)
            .unwrap_or_else(|_| panic!("{}", label));
        ProcessedPart {
            label: Some(match &ribbon_label {
                Some(ribbon_label) => format!("{} {}", ribbon_label, label),
                None => label.to_owned(),
            }),
            kind: ribbon_label.clone(),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: DateCode {
                year: to_full_year(year_hint, chip.year),
                week: chip.week,
                month: chip.month,
                jun: None,
            },
            rom_id: None,
        }
    } else {
        ProcessedPart {
            label: ribbon_label.clone(),
            kind: ribbon_label.clone(),
            manufacturer: Some(Manufacturer::Sharp),
            ..ProcessedPart::default()
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
            .and_then(|screen| to_full_year(year_hint, screen.year)),
        month: screen.as_ref().and_then(|screen| screen.month),
        column_driver,
        row_driver,
    })
}
