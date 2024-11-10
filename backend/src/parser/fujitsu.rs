// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{complete::one_of, streaming::char},
    combinator::{opt, recognize},
    sequence::{terminated, tuple},
    Parser as _,
};

use crate::parser::{
    for_nom::{digits, uppers, year2_week2},
    Manufacturer, NomParser,
};

use super::{
    for_nom::{alnum_uppers, cgb_rom_code, dmg_rom_code},
    GenericPart, MaskRom,
};

/// Fujitsu MB85R256 (SOP-28, 3.0-3.6V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::fujitsu::FUJITSU_MB85R256.parse("JAPAN MB85R256A 0412 M88").is_ok());
/// assert!(parser::fujitsu::FUJITSU_MB85R256.parse("JAPAN MB85R256S 0511 M22 E1").is_ok());
/// ```
pub static FUJITSU_MB85R256: NomParser<GenericPart> = NomParser {
    name: "Fujitsu MB85R256",
    f: |input| {
        tuple((
            tag("JAPAN "),
            recognize(tag("MB85R256").and(opt(one_of("AS")))),
            char(' '),
            year2_week2,
            char(' '),
            uppers(1).and(digits(2)),
            opt(nom::bytes::complete::tag(" E1")),
        ))
        .map(|(_, kind, _, date_code, _, _, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Fujitsu),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Fujitsu MB82D12160 (TSOP-I-48)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::fujitsu::FUJITSU_MB82D12160.parse("JAPAN 82D12160-10FN 0238 M88N").is_ok());
/// ```
pub static FUJITSU_MB82D12160: NomParser<GenericPart> = NomParser {
    name: "Fujitsu MB82D12160",
    f: |input| {
        tuple((
            tag("JAPAN "),
            tag("82D12160-10FN"),
            char(' '),
            year2_week2,
            char(' '),
            uppers(1).and(digits(2)).and(uppers(1)),
        ))
        .map(|(_, kind, _, date_code, _, _)| GenericPart {
            kind: format!("MB{kind}"),
            manufacturer: Some(Manufacturer::Fujitsu),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Fujitsu mask ROM (SOP-32, 5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::fujitsu::FUJITSU_MASK_ROM.parse("JAPAN DMG-GKX-0 D1 1P0 AK 9328 R09").is_ok());
/// assert!(parser::fujitsu::FUJITSU_MASK_ROM.parse("JAPAN DMG-WJA-0 E1 3NH AK 9401 R17").is_ok());
/// ```
pub static FUJITSU_MASK_ROM: NomParser<MaskRom> = NomParser {
    name: "Fujitsu mask ROM",
    f: |input| {
        tuple((
            tag("JAPAN "),
            alt((dmg_rom_code(), cgb_rom_code())),
            char(' '),
            uppers(1).and(digits(1)),
            char(' '),
            digits(1).and(uppers(1)).and(alnum_uppers(1)),
            char(' '),
            tag("AK"),
            char(' '),
            year2_week2,
            char(' '),
            uppers(1).and(digits(2)),
        ))
        .map(
            |(_, rom_id, _, _, _, _, _, _, _, date_code, _, _)| MaskRom {
                rom_id: String::from(rom_id),
                manufacturer: Some(Manufacturer::Fujitsu),
                chip_type: None,
                date_code: Some(date_code),
            },
        )
        .parse(input)
    },
};

/// Fujitsu SGB mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::fujitsu::FUJITSU_SGB_ROM.parse("SYS-SGB-2 © 1994 Nintendo 9429 R77").is_ok());
/// ```
pub static FUJITSU_SGB_ROM: NomParser<MaskRom> = NomParser {
    name: "Fujitsu SGB ROM",
    f: |input| {
        tuple((
            terminated(tag("SYS-SGB-2"), tag(" © 1994 Nintendo ")),
            year2_week2,
            char(' '),
            uppers(1).and(digits(2)),
        ))
        .map(|(rom_id, date_code, _, _)| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Fujitsu),
            chip_type: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
