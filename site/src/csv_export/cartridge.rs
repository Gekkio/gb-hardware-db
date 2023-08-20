// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::legacy::cartridge::{LegacyBoard, LegacyMetadata};

use super::{chip, Builder, Field, ToCsv};

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
                        .add("extra_label", |m| (&m.extra_label).csv())
                        .add_date_code()
                },
            )
            .nest("u1", |m| m.board.u1.as_ref(), chip)
            .nest("u2", |m| m.board.u2.as_ref(), chip)
            .nest("u3", |m| m.board.u3.as_ref(), chip)
            .nest("u4", |m| m.board.u4.as_ref(), chip)
            .nest("u5", |m| m.board.u5.as_ref(), chip)
            .nest("u6", |m| m.board.u6.as_ref(), chip)
            .nest("u7", |m| m.board.u7.as_ref(), chip)
            .nest("x1", |m| m.board.x1.as_ref(), chip)
    }
}
