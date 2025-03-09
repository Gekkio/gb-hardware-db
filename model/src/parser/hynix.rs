// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize, value},
    error::ParseError,
    sequence::separated_pair,
    IResult, Parser,
};

use super::{
    for_nom::{agb_rom_code, digits, lines3, lines4, uppers, year2_week2},
    sram::Ram,
    GameMaskRom, GameRomType, PartDateCode,
};
use crate::parser::{Manufacturer, NomParser};

/// Hynix HY62LF16206 (TSOP-I-48, 2.3-2.7V)
///
/// Source:
///   "hynix HY62LF16206A-LT12C 128kx16bit full CMOS SRAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_HY62LF16206.parse("Hynix KOREA HY62LF16206A 0223A LT12C").is_ok());
/// ```
pub static HYNIX_HY62LF16206: NomParser<Ram> = NomParser {
    name: "Hynix HY62LF16206",
    f: |input| {
        lines3(
            separated_pair(tag("Hynix"), char(' '), tag("KOREA")),
            recognize(tag("HY62LF16206").and(opt(one_of("AB")))),
            separated_pair(
                date_code.and(process_code),
                char(' '),
                (
                    tag("L"),  // power
                    tag("T"),  // package
                    tag("12"), // speed
                    tag("C"),  // temperature
                ),
            ),
        )
        .map(
            |(_, kind, ((date_code, _), (power, package, speed, temp)))| Ram {
                kind: format!("{kind}-{power}{package}{speed}{temp}"),
                manufacturer: Some(Manufacturer::Hynix),
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};

/// Hynix HY62WT08081 (SOP-28, 2.7-5.5V)
///
/// Source:
///   "hynix HY62WT08081E Series 32Kx8bit CMOS SRAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_HY62WT08081.parse("hynix 0231A HY62WT081ED70C KOREA").is_ok());
/// ```
pub static HYNIX_HY62WT08081: NomParser<Ram> = NomParser {
    name: "Hynix HY62WT08081",
    f: |input| {
        lines3(
            separated_pair(tag("hynix"), char(' '), date_code.and(process_code)),
            (
                recognize(value("HY62WT08081", tag("HY62WT081")).and(opt(one_of("ABCDE")))),
                alt((tag("L"), tag("D"))),           // power
                alt((tag("50"), tag("70"))),         // speed
                alt((tag("C"), tag("E"), tag("I"))), // temperature
            ),
            tag("KOREA"),
        )
        .map(|((_, (date_code, _)), (kind, power, speed, temp), _)| Ram {
            kind: format!("{kind}{power}{speed}{temp}"),
            manufacturer: Some(Manufacturer::Hynix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn ac23v<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    rom_type: GameRomType,
) -> impl Parser<&'a str, Output = GameMaskRom, Error = E> {
    lines4(
        tag("HYNIX"),
        tag(chip_type),
        separated_pair(agb_rom_code(), char(' '), tag(rom_type.as_str())),
        alt((tag("NL"), tag("ZBR"))).and(digits(4)),
    )
    .map(move |(_, kind, (rom_id, _), _)| GameMaskRom {
        rom_id: String::from(rom_id),
        rom_type,
        manufacturer: Some(Manufacturer::Hynix),
        chip_type: Some(String::from(kind)),
        mask_code: None,
        date_code: None,
    })
}

/// Hynix AC23V32101 (TSOP-II-44, 3.3V, 32 Mibit / 4 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_AC23V32101.parse("HYNIX AC23V32101 AGB-BAUE-0 H2 ZBR4079").is_ok());
/// ```
pub static HYNIX_AC23V32101: NomParser<GameMaskRom> = NomParser {
    name: "Hynix AC23V32101",
    f: |input| ac23v("AC23V32101", GameRomType::H2).parse(input),
};

/// Hynix AC23V64101 (TSOP-II-44, 3.3V, 64 Mibit / 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_AC23V64101.parse("HYNIX AC23V64101 AGB-AZLP-0 I2 ZBR1467").is_ok());
/// ```
pub static HYNIX_AC23V64101: NomParser<GameMaskRom> = NomParser {
    name: "Hynix AC23V64101",
    f: |input| ac23v("AC23V64101", GameRomType::I2).parse(input),
};

/// Hynix AC23V128111 (TSOP-II-44, 3.3V, 128 Mibit / 16 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hynix::HYNIX_AC23V128111.parse("HYNIX AC23V128111 AGB-AY7E-0 J2 NL0013").is_ok());
/// ```
pub static HYNIX_AC23V128111: NomParser<GameMaskRom> = NomParser {
    name: "Hynix AC23V128111",
    f: |input| ac23v("AC23V128111", GameRomType::J2).parse(input),
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    year2_week2(input)
}

fn process_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    uppers(1).parse(input)
}
