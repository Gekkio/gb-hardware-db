use crate::legacy::console::{LegacyGbsMainboard, LegacyGbsMetadata};

use super::{calendar, calendar_short, chip, Builder, Field, ToCsv};

impl ToCsv for LegacyGbsMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .add("release_code", |m| (&m.release_code).csv())
            .add("calendar_short", |m| {
                calendar_short(m.year, None, m.week).csv()
            })
            .add("calendar", |m| calendar(m.year, None, m.week).csv())
            .add("year", |m| m.year.csv())
            .add("week", |m| m.week.csv())
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
                        .add("calendar_short", |b| {
                            calendar_short(b.year, b.month, None).csv()
                        })
                        .add("calendar", |b| calendar(b.year, b.month, None).csv())
                        .add("year", |b| b.year.csv())
                        .add("month", |b| b.month.csv())
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), chip)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), chip)
            .nest("u4", |m| m.mainboard.u4.as_ref(), chip)
            .nest("u5", |m| m.mainboard.u5.as_ref(), chip)
            .nest("u6", |m| m.mainboard.u6.as_ref(), chip)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), chip)
    }
}
