// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::{
    config::cartridge::*,
    input::cartridge::*,
    parser::{Accelerometer, Crystal, Eeprom, GameMaskRom, GenericPart, Mapper, Tama, UnknownChip},
};
use std::{any::Any, collections::HashMap};

use crate::{
    process::part::{ParsedPart, ProcessedPart},
    process::DateCode,
};

#[derive(Clone, Debug)]
pub struct LegacyMetadata {
    pub cfg: GameConfig,
    pub code: Option<String>,
    pub stamp: Option<String>,
    pub board: LegacyBoard,
    pub dump: Option<CartridgeDump>,
}

impl super::LegacyMetadata for LegacyMetadata {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyBoard {
    pub cfg: BoardConfig,
    pub kind: String,
    pub circled_letters: Option<String>,
    pub panel_position: Option<String>,
    pub date_code: DateCode,
    pub parts: HashMap<PartDesignator, ProcessedPart>,
}

impl LegacyBoard {
    pub fn new(board: CartridgeBoard, cfg: BoardConfig) -> Self {
        let parts = cfg
            .parts()
            .filter_map(|(designator, part)| {
                let submission_part = &board[designator];
                let label = &submission_part.as_ref()?.label.as_ref()?;

                let parsed = (part.parse_any)(label)
                    .unwrap_or_else(|_| panic!("Failed to parse {designator:?}:{label}"));

                let part = try_process::<GameMaskRom>(board.year, label, parsed)
                    .or_else(|parsed| try_process::<Mapper>(board.year, label, parsed))
                    .or_else(|parsed| try_process::<GenericPart>(board.year, label, parsed))
                    .or_else(|parsed| try_process::<Crystal>(board.year, label, parsed))
                    .or_else(|parsed| try_process::<Eeprom>(board.year, label, parsed))
                    .or_else(|parsed| try_process::<Accelerometer>(board.year, label, parsed))
                    .or_else(|parsed| try_process::<Tama>(board.year, label, parsed))
                    .or_else(|parsed| try_process::<UnknownChip>(board.year, label, parsed))
                    .unwrap_or_else(|_| {
                        println!("{board:?}");
                        panic!("Failed to process {designator:?}")
                    });

                Some((designator, part))
            })
            .collect();
        LegacyBoard {
            cfg,
            kind: board.label,
            circled_letters: board.circled_letters,
            panel_position: board.panel_position,
            date_code: DateCode::year_month(board.year, board.month),
            parts,
        }
    }
    pub fn mapper(&self) -> Option<&ProcessedPart> {
        self.cfg
            .parts()
            .find(|(_, part)| part.role == PartRole::Mapper)
            .and_then(|(designator, _)| self.parts.get(&designator))
    }
}

fn try_process<T: ParsedPart + 'static>(
    year: Option<u16>,
    label: &str,
    parsed: Box<dyn Any>,
) -> Result<ProcessedPart, Box<dyn Any>> {
    parsed
        .downcast::<T>()
        .map(|m| m.process(year, String::from(label)))
}
