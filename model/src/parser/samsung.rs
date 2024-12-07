// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{complete::one_of, streaming::char},
    combinator::{opt, recognize},
    error::ParseError,
    sequence::tuple,
    Parser,
};

use super::{
    for_nom::{alnum_uppers, cgb_rom_code, digits, dmg_rom_code, uppers},
    GameRomType, Manufacturer, MaskRom, NomParser,
};

fn gb_km23c_old<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    package: Package,
    rom_type: GameRomType,
    unknown2: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        tag("SEC "),
        recognize(tuple((
            tag("KM23C"),
            tag(chip_type),
            opt(one_of("ABCD")),
            tag(package.code()),
        ))),
        char(' '),
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        tag(rom_type.as_str()),
        char(' '),
        tag(unknown2)
            .and(digits(1))
            .and(alnum_uppers(2))
            .and(uppers(1)),
    ))
    .map(|(_, kind, _, rom_id, _, _, _, _)| MaskRom {
        rom_id: String::from(rom_id),
        chip_type: Some(String::from(kind)),
        manufacturer: Some(Manufacturer::Samsung),
        date_code: None,
    })
}

fn gb_km23c_new<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    package: Package,
    rom_type: GameRomType,
    unknown2: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        tag("SEC "),
        recognize(tuple((
            tag("KM23C"),
            tag(chip_type),
            opt(one_of("ABCD")),
            tag(package.code()),
        ))),
        char(' '),
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        tag(rom_type.as_str()),
        char(' '),
        recognize(tag(unknown2).and(digits(3)).and(uppers(2))),
    ))
    .map(|(_, kind, _, rom_id, _, _, _, _)| MaskRom {
        rom_id: String::from(rom_id),
        chip_type: Some(String::from(kind)),
        manufacturer: Some(Manufacturer::Samsung),
        date_code: None,
    })
}

/// Samsung KM23C4000 (SOP-32, 5V, 4 Mibit / 512 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::samsung::SAMSUNG_KM23C4000.parse("SEC KM23C4000DG DMG-ATEA-0 E1 KF5304U").is_ok());
/// ```
pub static SAMSUNG_KM23C4000: NomParser<MaskRom> = NomParser {
    name: "Samsung KM23C4000",
    f: |input| gb_km23c_old("4000", Package::Sop, GameRomType::E1, "KF5").parse(input),
};

/// Samsung KM23C8000 (SOP-32, 5V, 8 Mibit / 1 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::samsung::SAMSUNG_KM23C8000.parse("SEC KM23C8000DG DMG-APSJ-0 F1 KFX3ALY").is_ok());
/// assert!(parser::samsung::SAMSUNG_KM23C8000.parse("SEC KM23C8000DG DMG-AAUJ-1 F1 KFX331U").is_ok());
/// ```
pub static SAMSUNG_KM23C8000: NomParser<MaskRom> = NomParser {
    name: "Samsung KM23C8000",
    f: |input| gb_km23c_old("8000", Package::Sop, GameRomType::F1, "KFX").parse(input),
};

/// Samsung KM23C16120 (TSOP-II-44, 5V, 16 Mibit / 2 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::samsung::SAMSUNG_KM23C16120.parse("SEC KM23C16120T DMG-ADQJ-0 G2 KF6402G").is_ok());
/// assert!(parser::samsung::SAMSUNG_KM23C16120.parse("SEC KM23C16120DT DMG-AWLP-0 G2 KF6409G").is_ok());
/// assert!(parser::samsung::SAMSUNG_KM23C16120.parse("SEC KM23C16120DT CGB-BHMJ-0 G2 K3N5C317GD").is_ok());
/// ```
pub static SAMSUNG_KM23C16120: NomParser<MaskRom> = NomParser {
    name: "Samsung KM23C16120",
    f: |input| {
        alt((
            gb_km23c_old("16120", Package::Tsop, GameRomType::G2, "KF6"),
            gb_km23c_new("16120", Package::Tsop, GameRomType::G2, "K3N5C"),
        ))
        .parse(input)
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Sop,
    Tsop,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Sop => "G",
            Package::Tsop => "T",
        }
    }
}
