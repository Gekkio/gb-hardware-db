// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::legacy::console::{LegacyAgsMainboard, LegacyAgsMetadata};

use super::{chip, Builder, Field, ToCsv};

impl ToCsv for LegacyAgsMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacyAgsMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("number_pair", |m| (&m.number_pair).csv())
                        .add("stamp", |m| (&m.stamp).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add_date_code()
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), chip)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), chip)
            .nest("amplifier", |m| m.mainboard.amplifier.as_ref(), chip)
            .nest("u4", |m| m.mainboard.u4.as_ref(), chip)
            .nest("u5", |m| m.mainboard.u5.as_ref(), chip)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), chip)
    }
}
