use crate::legacy::console::{LegacyOxyMainboard, LegacyOxyMetadata};

use super::{chip, Builder, Field, ToCsv};

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
                        .add_date_code()
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), chip)
            .nest("u2", |m| m.mainboard.u2.as_ref(), chip)
            .nest("u4", |m| m.mainboard.u4.as_ref(), chip)
            .nest("u5", |m| m.mainboard.u5.as_ref(), chip)
    }
}
