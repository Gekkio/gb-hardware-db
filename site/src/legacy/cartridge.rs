// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::process::part::loose_datecode;
use crate::{
    process::DateCode,
    process::part::{ParsedPart, ProcessedPart},
};
use gbhwdb_model::{config::cartridge::*, input::cartridge::*, parser, parser::LabelParser};
use std::collections::HashMap;

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
    pub battery: Option<ProcessedPart>,
}

impl LegacyBoard {
    pub fn new(board: CartridgeBoard, cfg: BoardConfig) -> Self {
        let parts = cfg
            .parts()
            .filter_map(|(designator, part)| {
                fn parse<T: ParsedPart>(
                    board: &CartridgeBoard,
                    designator: PartDesignator,
                    parser: &dyn LabelParser<T>,
                ) -> Option<(PartDesignator, ProcessedPart)> {
                    let submission_part = &board[designator];
                    let label = Some(&submission_part.label).filter(|label| !label.is_empty())?;

                    let parsed = parser
                        .parse(label)
                        .unwrap_or_else(|_| panic!("Failed to parse {designator:?}:{label}"));

                    Some((designator, parsed.process(board.year, String::from(label))))
                }
                match part {
                    BoardPart::Unknown(parser) => parse(&board, designator, parser),
                    BoardPart::Rom(parser) => parse(&board, designator, parser),
                    BoardPart::Mapper(parser) => parse(&board, designator, parser),
                    BoardPart::Ram(parser) => parse(&board, designator, parser),
                    BoardPart::SupervisorReset(parser) => parse(&board, designator, parser),
                    BoardPart::Crystal(parser) => parse(&board, designator, parser),
                    BoardPart::Flash(parser) => parse(&board, designator, parser),
                    BoardPart::Eeprom(parser) => parse(&board, designator, parser),
                    BoardPart::Accelerometer(parser) => parse(&board, designator, parser),
                    BoardPart::LineDecoder(parser) => parse(&board, designator, parser),
                    BoardPart::HexInverter(parser) => parse(&board, designator, parser),
                    BoardPart::Mcu(parser) => parse(&board, designator, parser),
                    BoardPart::Rtc(parser) => parse(&board, designator, parser),
                }
            })
            .collect();
        LegacyBoard {
            cfg,
            kind: board.label,
            circled_letters: Some(board.circled_letters).filter(|letters| !letters.is_empty()),
            panel_position: Some(board.panel_position).filter(|position| !position.is_empty()),
            date_code: DateCode::year_month(board.year, board.month),
            parts,
            battery: cfg.battery_type().map(|kind| {
                let label = Some(&board.battery.label).filter(|label| !label.is_empty());
                ProcessedPart {
                    kind: Some(match kind {
                        BatteryType::Cr1616 => String::from("CR1616"),
                        BatteryType::Cr2025 => String::from("CR2025"),
                    }),
                    label: label.cloned(),
                    manufacturer: board.battery.manufacturer,
                    date_code: loose_datecode(
                        board.year,
                        label.as_ref().map(|label| {
                            parser::battery()
                                .parse(&label)
                                .unwrap_or_else(|_| panic!("Failed to parse battery label:{label}"))
                        }),
                    ),
                    rom_id: None,
                }
            }),
        }
    }
    pub fn mapper(&self) -> Option<&ProcessedPart> {
        self.cfg
            .parts()
            .find(|(_, part)| matches!(part, BoardPart::Mapper(_)))
            .and_then(|(designator, _)| self.parts.get(&designator))
    }
}
