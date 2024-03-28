// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::{
    config::cartridge::*,
    input::cartridge::*,
    parser::{
        accelerometer::accelerometer, crystal_32kihz::crystal_32kihz, eeprom::eeprom, flash::flash,
        hex_inverter::hex_inverter, line_decoder::line_decoder, mapper::mapper, mask_rom::mask_rom,
        ram::ram, rtc::rtc, supervisor_reset::supervisor_reset, tama::tama,
    },
};
use std::collections::HashMap;

use crate::{
    process::part::{boxed_parser, BoxedParser, ProcessedPart},
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
    pub layout: BoardLayout,
    pub kind: String,
    pub circled_letters: Option<String>,
    pub panel_position: Option<String>,
    pub date_code: DateCode,
    pub parts: HashMap<PartDesignator, ProcessedPart>,
}

impl LegacyBoard {
    pub fn new(board: CartridgeBoard, layout: BoardLayout) -> Self {
        let roles = PartRoleConfig::from(layout);
        let parts = roles
            .into_iter()
            .filter_map(|(designator, role)| {
                let parser = part_parser(layout, role);
                let part = board[designator].as_ref()?;
                let part = parser(board.year, part).unwrap()?;
                Some((designator, part))
            })
            .collect();
        LegacyBoard {
            layout,
            kind: board.label,
            circled_letters: board.circled_letters,
            panel_position: board.panel_position,
            date_code: DateCode::year_month(board.year, board.month),
            parts,
        }
    }
}

fn part_parser(layout: BoardLayout, role: PartRole) -> BoxedParser<'static> {
    match role {
        PartRole::Rom if layout == BoardLayout::Tama => boxed_parser(tama()),
        PartRole::Rom => boxed_parser(mask_rom()),
        PartRole::Mapper if layout == BoardLayout::Tama => boxed_parser(tama()),
        PartRole::Mapper => boxed_parser(mapper()),
        PartRole::Ram => boxed_parser(ram()),
        PartRole::SupervisorReset => boxed_parser(supervisor_reset()),
        PartRole::Crystal => boxed_parser(crystal_32kihz()),
        PartRole::Flash => boxed_parser(flash()),
        PartRole::Eeprom => boxed_parser(eeprom()),
        PartRole::Accelerometer => boxed_parser(accelerometer()),
        PartRole::LineDecoder => boxed_parser(line_decoder()),
        PartRole::HexInverter => boxed_parser(hex_inverter()),
        PartRole::Rtc => boxed_parser(rtc()),
        PartRole::Mcu => boxed_parser(tama()),
        PartRole::Unknown => Box::new(|_, part| {
            Ok(part.label.clone().map(|label| ProcessedPart {
                label: Some(label),
                ..ProcessedPart::default()
            }))
        }),
    }
}
