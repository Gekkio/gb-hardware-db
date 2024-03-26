// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::legacy::console::{LegacyGbsMainboard, LegacyGbsMetadata};

use super::{part, Builder, Field, ToCsv};

impl ToCsv for LegacyGbsMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .add("release_code", |m| (&m.release_code).csv())
            .add_date_code()
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacyGbsMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("number_pair", |m| (&m.number_pair).csv())
                        .add("stamp", |m| (&m.stamp).csv())
                        .add("stamp_front", |m| (&m.stamp_front).csv())
                        .add("stamp_back", |m| (&m.stamp_back).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add_date_code()
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), part)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), part)
            .nest("u4", |m| m.mainboard.u4.as_ref(), part)
            .nest("u5", |m| m.mainboard.u5.as_ref(), part)
            .nest("u6", |m| m.mainboard.u6.as_ref(), part)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), part)
    }
}
