// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::input::dmg::DmgJackBoard;

use crate::{
    HasDateCode,
    csv_export::{Builder, Field, ToCsv, part},
    legacy::console::{
        LegacyDmgLcdBoard, LegacyDmgMainboard, LegacyDmgMetadata, LegacyDmgPowerBoard,
        LegacyLcdPanel,
    },
};

impl ToCsv for LegacyDmgMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .add_date_code(|m| m.date_code())
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacyDmgMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("extra_label", |m| (&m.extra_label).csv())
                        .add("stamp", |m| (&m.stamp).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), part)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), part)
            .nest("video_ram", |m| m.mainboard.video_ram.as_ref(), part)
            .nest("amplifier", |m| m.mainboard.amplifier.as_ref(), part)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), part)
            .nest(
                "lcd_board",
                |m| m.lcd_board.as_ref(),
                || {
                    Builder::<LegacyDmgLcdBoard>::new()
                        .add("type", |b| (&b.kind).csv())
                        .add("circled_letters", |b| (&b.circled_letters).csv())
                        .add("stamp", |b| (&b.stamp).csv())
                        .add_date_code(|m| m.date_code())
                },
            )
            .nest(
                "lcd_panel",
                |m| m.lcd_board.as_ref().and_then(|b| b.lcd_panel.as_ref()),
                || {
                    Builder::<LegacyLcdPanel>::new()
                        .add("label", |p| (&p.label).csv())
                        .add_date_code(|m| m.date_code())
                },
            )
            .nest(
                "column_driver",
                |m| {
                    m.lcd_board
                        .as_ref()
                        .and_then(|b| b.lcd_panel.as_ref().and_then(|p| p.column_driver.as_ref()))
                },
                part,
            )
            .nest(
                "row_driver",
                |m| {
                    m.lcd_board
                        .as_ref()
                        .and_then(|b| b.lcd_panel.as_ref().and_then(|p| p.row_driver.as_ref()))
                },
                part,
            )
            .nest(
                "regulator",
                |m| m.lcd_board.as_ref().and_then(|b| b.regulator.as_ref()),
                part,
            )
            .nest(
                "power_board",
                |m| m.power_board.as_ref(),
                || {
                    Builder::<LegacyDmgPowerBoard>::new()
                        .add("type", |b| (&b.kind).csv())
                        .add("label", |b| (&b.label).csv())
                        .add_date_code(|m| m.date_code())
                },
            )
            .nest(
                "jack_board",
                |m| {
                    if m.jack_board.is_unknown() {
                        None
                    } else {
                        Some(&m.jack_board)
                    }
                },
                || {
                    Builder::<DmgJackBoard>::new()
                        .add("type", |b| (&b.kind).csv())
                        .add("extra_label", |b| (&b.extra_label).csv())
                },
            )
    }
}
