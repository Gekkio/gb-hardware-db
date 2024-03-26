// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::legacy::console::{LegacyCgbMainboard, LegacyCgbMetadata};

use super::{part, Builder, Field, ToCsv};

impl ToCsv for LegacyCgbMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .add("release_code", |m| (&m.release_code).csv())
            .add_date_code()
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacyCgbMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("number_pair", |m| (&m.number_pair).csv())
                        .add("stamp", |m| (&m.stamp).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add_date_code()
                    // TODO: date_range?
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), part)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), part)
            .nest("amplifier", |m| m.mainboard.amplifier.as_ref(), part)
            .nest("regulator", |m| m.mainboard.regulator.as_ref(), part)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), part)
    }
}
