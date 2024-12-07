// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt, bytes::streaming::tag, character::streaming::char, error::ParseError,
    sequence::tuple, Parser,
};

use super::{
    for_nom::{agb_rom_code, digits},
    GameMaskRom, GameRomType,
};
use crate::parser::{Manufacturer, NomParser};

fn ac23v<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    rom_type: GameRomType,
) -> impl Parser<&'a str, GameMaskRom, E> {
    tuple((
        tag("MAGNACHIP "),
        tag(chip_type),
        char(' '),
        agb_rom_code(),
        char(' '),
        tag(rom_type.as_str()),
        char(' '),
        alt((tag("GB"), tag("SP"))),
        digits(4),
        tag(" PS"),
    ))
    .map(move |(_, kind, _, rom_id, _, _, _, _, _, _)| GameMaskRom {
        rom_id: String::from(rom_id),
        rom_type,
        manufacturer: Some(Manufacturer::Magnachip),
        chip_type: Some(String::from(kind)),
        mask_code: None,
        date_code: None,
    })
}

/// Magnachip AC23V32101 (TSOP-II-44, 3.3V, 32 Mibit / 4 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::magnachip::MAGNACHIP_AC23V32101.parse("MAGNACHIP AC23V32101 AGB-BCRP-0 H2 GB1191 PS").is_ok());
/// ```
pub static MAGNACHIP_AC23V32101: NomParser<GameMaskRom> = NomParser {
    name: "Magnachip AC23V32101",
    f: |input| ac23v("AC23V32101", GameRomType::H2).parse(input),
};

/// Magnachip AC23V64101 (TSOP-II-44, 3.3V, 64 Mibit / 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::magnachip::MAGNACHIP_AC23V64101.parse("MAGNACHIP AC23V64101 AGB-BQQX-0 I2 GB0249 PS").is_ok());
/// ```
pub static MAGNACHIP_AC23V64101: NomParser<GameMaskRom> = NomParser {
    name: "Magnachip AC23V64101",
    f: |input| ac23v("AC23V64101", GameRomType::I2).parse(input),
};

/// Magnachip AC23V128111 (TSOP-II-44, 3.3V, 128 Mibit / 16 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::magnachip::MAGNACHIP_AC23V128111.parse("MAGNACHIP AC23V128111 AGB-BPRE-1 J2 SP0730 PS").is_ok());
/// ```
pub static MAGNACHIP_AC23V128111: NomParser<GameMaskRom> = NomParser {
    name: "Magnachip AC23V128111",
    f: |input| ac23v("AC23V128111", GameRomType::J2).parse(input),
};
