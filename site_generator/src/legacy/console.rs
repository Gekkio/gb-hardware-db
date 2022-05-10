use gbhwdb_backend::{
    input::{LcdChip, LcdScreen},
    parser::LabelParser,
    Console,
};
use serde::Serialize;

use super::{to_legacy_year, HasDateCode, LegacyChip, LegacyPhoto, LegacyPhotos};

pub trait LegacyMainboard {
    fn kind(&self) -> &str;
    fn calendar_short(&self) -> Option<String>;
}

pub trait LegacyConsoleMetadata: 'static {
    type Mainboard: LegacyMainboard;
    const CONSOLE: Console;
    const PLACEHOLDER_SVG: Option<&'static str> = None;

    fn chips() -> Vec<ChipInfo<Self>>;
    fn mainboard(&self) -> &Self::Mainboard;
    fn release_code(&self) -> Option<&str> {
        None
    }
    fn assembled(&self) -> Option<String> {
        None
    }
    fn lcd_panel(&self) -> Option<String> {
        None
    }
}

pub trait LegacyConsolePhotos: 'static {
    fn photos() -> Vec<PhotoInfo<Self>>;
}

impl LegacyConsolePhotos for LegacyPhotos {
    fn photos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new("Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new("Back", Box::new(|p| p.back.as_ref())),
            PhotoInfo::new("PCB front", Box::new(|p| p.pcb_front.as_ref())),
            PhotoInfo::new("PCB back", Box::new(|p| p.pcb_back.as_ref())),
        ]
    }
}

pub struct PhotoInfo<P: ?Sized> {
    pub label: &'static str,
    pub getter: Box<dyn Fn(&P) -> Option<&LegacyPhoto>>,
}

impl<P: ?Sized> PhotoInfo<P> {
    pub fn new(label: &'static str, getter: Box<dyn Fn(&P) -> Option<&LegacyPhoto>>) -> Self {
        PhotoInfo { label, getter }
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

impl LegacyConsolePhotos for LegacyDmgPhotos {
    fn photos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new("Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new("Back", Box::new(|p| p.back.as_ref())),
            PhotoInfo::new("Mainboard front", Box::new(|p| p.mainboard_front.as_ref())),
            PhotoInfo::new("Mainboard back", Box::new(|p| p.mainboard_back.as_ref())),
            PhotoInfo::new("LCD board front", Box::new(|p| p.lcd_board_front.as_ref())),
            PhotoInfo::new("LCD board back", Box::new(|p| p.lcd_board_back.as_ref())),
            PhotoInfo::new(
                "Power board front",
                Box::new(|p| p.power_board_front.as_ref()),
            ),
            PhotoInfo::new(
                "Power board back",
                Box::new(|p| p.power_board_back.as_ref()),
            ),
            PhotoInfo::new(
                "Jack board front",
                Box::new(|p| p.jack_board_front.as_ref()),
            ),
            PhotoInfo::new("Jack board back", Box::new(|p| p.jack_board_back.as_ref())),
        ]
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

impl LegacyConsolePhotos for LegacyAgsPhotos {
    fn photos() -> Vec<PhotoInfo<Self>> {
        vec![
            PhotoInfo::new("Front", Box::new(|p| p.front.as_ref())),
            PhotoInfo::new("Top", Box::new(|p| p.top.as_ref())),
            PhotoInfo::new("Back", Box::new(|p| p.back.as_ref())),
            PhotoInfo::new("PCB front", Box::new(|p| p.pcb_front.as_ref())),
            PhotoInfo::new("PCB back", Box::new(|p| p.pcb_back.as_ref())),
        ]
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

impl LegacyMainboard for LegacyDmgMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        None
    }
}

impl LegacyConsoleMetadata for LegacyDmgMetadata {
    type Mainboard = LegacyDmgMainboard;
    const CONSOLE: Console = Console::Dmg;
    const PLACEHOLDER_SVG: Option<&'static str> = Some("/dmg_placeholder.svg");

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("VRAM", "U2", Box::new(|m| m.mainboard.video_ram.as_ref())),
            ChipInfo::new("WRAM", "U3", Box::new(|m| m.mainboard.work_ram.as_ref())),
        ]
    }

    fn assembled(&self) -> Option<String> {
        self.calendar_short()
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacySgbMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacySgbMetadata {
    type Mainboard = LegacySgbMainboard;
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

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacySgb2Mainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacySgb2Metadata {
    type Mainboard = LegacySgb2Mainboard;
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

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyMgbMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyMgbMetadata {
    type Mainboard = LegacyMgbMainboard;
    const CONSOLE: Console = Console::Mgb;
    const PLACEHOLDER_SVG: Option<&'static str> = Some("/mgb_placeholder.svg");

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new(
                "Regulator",
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

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn assembled(&self) -> Option<String> {
        self.calendar_short()
    }

    fn lcd_panel(&self) -> Option<String> {
        self.lcd_panel
            .as_ref()
            .and_then(|panel| panel.calendar_short())
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyMglMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyMglMetadata {
    type Mainboard = LegacyMglMainboard;
    const CONSOLE: Console = Console::Mgl;
    const PLACEHOLDER_SVG: Option<&'static str> = Some("/mgl_placeholder.svg");

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new(
                "Regulator",
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

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn assembled(&self) -> Option<String> {
        self.calendar_short()
    }

    fn lcd_panel(&self) -> Option<String> {
        self.lcd_panel
            .as_ref()
            .and_then(|panel| panel.calendar_short())
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyCgbMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyCgbMetadata {
    type Mainboard = LegacyCgbMainboard;
    const CONSOLE: Console = Console::Cgb;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Amplifier",
                "U3",
                Box::new(|m| m.mainboard.amplifier.as_ref()),
            ),
            ChipInfo::new(
                "Regulator",
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

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn assembled(&self) -> Option<String> {
        self.calendar_short()
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyAgbMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyAgbMetadata {
    type Mainboard = LegacyAgbMainboard;
    const CONSOLE: Console = Console::Agb;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("WRAM", "U2", Box::new(|m| m.mainboard.work_ram.as_ref())),
            ChipInfo::new(
                "Regulator",
                "U3",
                Box::new(|m| m.mainboard.regulator.as_ref()),
            ),
            ChipInfo::new("?", "U4", Box::new(|m| m.mainboard.u4.as_ref())),
            ChipInfo::new(
                "Amplifier",
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

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn assembled(&self) -> Option<String> {
        self.calendar_short()
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyAgsMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyAgsMetadata {
    type Mainboard = LegacyAgsMainboard;
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

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyGbsMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyGbsMetadata {
    type Mainboard = LegacyGbsMainboard;
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

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn assembled(&self) -> Option<String> {
        self.calendar_short()
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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

impl LegacyMainboard for LegacyOxyMainboard {
    fn kind(&self) -> &str {
        &self.kind
    }
    fn calendar_short(&self) -> Option<String> {
        HasDateCode::calendar_short(self)
    }
}

impl LegacyConsoleMetadata for LegacyOxyMetadata {
    type Mainboard = LegacyOxyMainboard;
    const CONSOLE: Console = Console::Oxy;

    fn chips() -> Vec<ChipInfo<Self>> {
        vec![
            ChipInfo::new("CPU", "U1", Box::new(|m| m.mainboard.cpu.as_ref())),
            ChipInfo::new("?", "U2", Box::new(|m| m.mainboard.u2.as_ref())),
            ChipInfo::new("?", "U4", Box::new(|m| m.mainboard.u4.as_ref())),
            ChipInfo::new("?", "U5", Box::new(|m| m.mainboard.u5.as_ref())),
        ]
    }

    fn release_code(&self) -> Option<&str> {
        self.release_code.as_deref()
    }

    fn mainboard(&self) -> &Self::Mainboard {
        &self.mainboard
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
