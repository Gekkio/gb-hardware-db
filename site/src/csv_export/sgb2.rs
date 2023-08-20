// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::legacy::console::{LegacySgb2Mainboard, LegacySgb2Metadata};

use super::{chip, Builder, Field, ToCsv};

impl ToCsv for LegacySgb2Metadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("stamp", |m| (&m.stamp).csv())
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacySgb2Mainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add("letter_at_top_right", |m| (&m.letter_at_top_right).csv())
                        .add_date_code()
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), chip)
            .nest("icd2", |m| m.mainboard.icd2.as_ref(), chip)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), chip)
            .nest("rom", |m| m.mainboard.rom.as_ref(), chip)
            .nest("cic", |m| m.mainboard.cic.as_ref(), chip)
            .nest("coil", |m| m.mainboard.coil.as_ref(), chip)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), chip)
    }
}
