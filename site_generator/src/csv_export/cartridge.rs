use crate::legacy::cartridge::{LegacyBoard, LegacyMetadata};

use super::{calendar, calendar_short, chip, Builder, Field, ToCsv};

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
                        .add("calendar_short", |b| {
                            calendar_short(b.year, b.month, None).csv()
                        })
                        .add("calendar", |b| calendar(b.year, b.month, None).csv())
                        .add("year", |b| b.year.csv())
                        .add("month", |b| b.month.csv())
                },
            )
            .nest("rom", |m| m.board.rom.as_ref(), chip)
            .nest("mapper", |m| m.board.mapper.as_ref(), chip)
            .nest("ram", |m| m.board.ram.as_ref(), chip)
            .nest("ram_protector", |m| m.board.ram_protector.as_ref(), chip)
            .nest("crystal", |m| m.board.crystal.as_ref(), chip)
            .nest("rom2", |m| m.board.rom2.as_ref(), chip)
            .nest("flash", |m| m.board.flash.as_ref(), chip)
            .nest("line_decoder", |m| m.board.line_decoder.as_ref(), chip)
            .nest("eeprom", |m| m.board.eeprom.as_ref(), chip)
            .nest("accelerometer", |m| m.board.accelerometer.as_ref(), chip)
            .nest("u4", |m| m.board.u4.as_ref(), chip)
            .nest("u5", |m| m.board.u5.as_ref(), chip)
            .nest("crystal", |m| m.board.crystal.as_ref(), chip)
    }
}
