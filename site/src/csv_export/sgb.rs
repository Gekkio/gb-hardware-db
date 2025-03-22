// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::{
    HasDateCode,
    csv_export::{Builder, Field, ToCsv, part},
    legacy::console::{LegacySgbMainboard, LegacySgbMetadata},
};

impl ToCsv for LegacySgbMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("stamp", |m| (&m.stamp).csv())
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacySgbMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add("letter_at_top_right", |m| (&m.letter_at_top_right).csv())
                        .add_date_code(|m| m.date_code())
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), part)
            .nest("icd2", |m| m.mainboard.icd2.as_ref(), part)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), part)
            .nest("video_ram", |m| m.mainboard.video_ram.as_ref(), part)
            .nest("rom", |m| m.mainboard.rom.as_ref(), part)
            .nest("cic", |m| m.mainboard.cic.as_ref(), part)
    }
}
