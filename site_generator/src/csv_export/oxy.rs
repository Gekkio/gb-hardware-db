use crate::legacy::console::{LegacyOxyMainboard, LegacyOxyMetadata};

use super::{calendar, calendar_short, chip, Builder, Field, ToCsv};

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
                        .add("calendar_short", |b| {
                            calendar_short(b.year, b.month, None).csv()
                        })
                        .add("calendar", |b| calendar(b.year, b.month, None).csv())
                        .add("year", |b| b.year.csv())
                        .add("month", |b| b.month.csv())
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), chip)
            .nest("u2", |m| m.mainboard.u2.as_ref(), chip)
            .nest("u4", |m| m.mainboard.u4.as_ref(), chip)
            .nest("u5", |m| m.mainboard.u5.as_ref(), chip)
    }
}
