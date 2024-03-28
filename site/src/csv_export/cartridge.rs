// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_backend::config::cartridge::PartDesignator;

use crate::{
    csv_export::{part, Builder, Field, ToCsv},
    legacy::cartridge::{LegacyBoard, LegacyMetadata},
};

impl ToCsv for LegacyMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("code", |m| (&m.code).csv())
            .add("stamp", |m| (&m.stamp).csv())
            .nest(
                "mainboard",
                |m| Some(&m.board),
                || {
                    Builder::<LegacyBoard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add("panel_position", |m| (&m.panel_position).csv())
                        .add_date_code(|m| m.date_code)
                },
            )
            .nest("u1", |m| m.board.parts.get(&PartDesignator::U1), part)
            .nest("u2", |m| m.board.parts.get(&PartDesignator::U2), part)
            .nest("u3", |m| m.board.parts.get(&PartDesignator::U3), part)
            .nest("u4", |m| m.board.parts.get(&PartDesignator::U4), part)
            .nest("u5", |m| m.board.parts.get(&PartDesignator::U5), part)
            .nest("u6", |m| m.board.parts.get(&PartDesignator::U6), part)
            .nest("u7", |m| m.board.parts.get(&PartDesignator::U7), part)
            .nest("x1", |m| m.board.parts.get(&PartDesignator::X1), part)
    }
}
