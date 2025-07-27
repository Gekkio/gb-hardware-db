// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::borrow::Cow;
use strum::VariantArray;

use crate::{
    csv_export::{Builder, Field, ToCsv, part},
    legacy::cartridge::{LegacyBoard, LegacyMetadata},
};
use gbhwdb_model::{config::cartridge::PartDesignator, input::cartridge::CartridgeDump};

impl ToCsv for LegacyMetadata {
    fn csv_builder() -> Builder<Self> {
        let mut builder = Builder::<Self>::new()
            .add("game_name", |m| (&m.cfg.name).csv())
            .add("code", |m| (&m.code).csv())
            .add("stamp", |m| (&m.stamp).csv())
            .nest(
                "mainboard",
                |m| Some(&m.board),
                || {
                    Builder::<LegacyBoard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                        .add("panel_position", |m| (&m.panel_position).csv())
                        .add_date_code(|m| m.date_code)
                },
            );
        for &designator in PartDesignator::VARIANTS {
            builder = builder.add(format!("{}_role", designator.as_lower_str()), move |m| {
                m.board
                    .cfg
                    .part(designator)
                    .map(|part| Cow::from(part.role().display()))
                    .unwrap_or_default()
            });
            builder = builder.nest(
                designator.as_lower_str(),
                move |m| m.board.parts.get(&designator),
                part,
            )
        }
        builder
            .nest("battery", |m| m.board.battery.as_ref(), part)
            .nest("dump", |m| m.dump.as_ref(), dump)
    }
}

fn dump() -> Builder<CartridgeDump> {
    Builder::<CartridgeDump>::new()
        .add("crc32", |c| (&c.crc32).csv())
        .add("md5", |c| (&c.md5).csv())
        .add("sha1", |c| (&c.sha1).csv())
        .add("sha256", |c| (&c.sha256).csv())
}
