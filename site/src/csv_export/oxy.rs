// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::{
    csv_export::{part, Builder, Field, ToCsv},
    legacy::console::{LegacyOxyMainboard, LegacyOxyMetadata},
    HasDateCode,
};

impl ToCsv for LegacyOxyMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .add("release_code", |m| (&m.release_code).csv())
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacyOxyMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add_date_code(|m| m.date_code())
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), part)
            .nest("u2", |m| m.mainboard.u2.as_ref(), part)
            .nest("u4", |m| m.mainboard.u4.as_ref(), part)
            .nest("u5", |m| m.mainboard.u5.as_ref(), part)
    }
}
