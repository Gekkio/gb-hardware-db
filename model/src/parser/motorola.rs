// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser as _, bytes::streaming::tag, sequence::delimited};

use super::{
    Manufacturer, Mapper, MapperChip, NomParser,
    for_nom::{lines4, year2_week2},
};

/// Motorola MBC1B (SOP-24)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::motorola::MOTOROLA_MBC1B.parse("DMG MBC1B Nintendo J9130BR").is_ok());
/// ```
pub static MOTOROLA_MBC1B: NomParser<Mapper> = NomParser {
    name: "Motorola MBC1B",
    f: |input| {
        lines4(
            tag("DMG"),
            tag("MBC1B"),
            tag("Nintendo"),
            delimited(tag("J"), year2_week2, tag("BR")),
        )
        .map(|(_, _, _, date_code)| Mapper {
            kind: MapperChip::Mbc1B,
            manufacturer: Some(Manufacturer::Motorola),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
