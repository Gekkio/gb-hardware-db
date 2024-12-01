// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{recognize, value},
    error::ParseError,
    sequence::{terminated, tuple},
    Parser,
};

use super::{
    for_nom::{cgb_rom_code, digits, dmg_rom_code, uppers, week2, year1, year2_week2},
    sram::Ram,
    GenericPart, Manufacturer, MaskRom, NomParser, PartDateCode,
};

/// Toshiba TC8521AM (SOP-20)
///
/// Source:
///   "TOSHIBA TC8521AP, TC8521AM (Real Time Clock II)"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC8521AM.parse("T 9722HB 8521AM").is_ok());
/// ```
pub static TOSHIBA_TC8521AM: NomParser<GenericPart> = NomParser {
    name: "Toshiba TC8521AM",
    f: |input| toshiba_tc8521a(Package::SOP20).parse(input),
};

fn toshiba_tc8521a<'a, E: ParseError<&'a str>>(
    package: Package,
) -> impl Parser<&'a str, GenericPart, E> {
    tuple((
        tag("T "),
        tuple((year2_week2, tag("HB"))),
        char(' '),
        tag("8521A").and(nom::character::streaming::char(package.code_char())),
    ))
    .map(
        move |(_, (date_code, _), _, (_, package_code))| GenericPart {
            kind: format!("TC8521A{package_code}"),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(date_code),
        },
    )
}

/// Toshiba TC7W139F
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC7W139F.parse("7W139 0J").is_ok());
/// ```
pub static TOSHIBA_TC7W139F: NomParser<GenericPart> = NomParser {
    name: "Toshiba TC7W139F",
    f: |input| {
        tuple((
            alt((
                value("TC7W139FU", tag("7W139")),
                value("TC7W139F", tag("7W139F")),
            )),
            char(' '),
            year1.and(uppers(1)),
        ))
        .map(|(kind, _, (year, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(PartDateCode::Year { year }),
        })
        .parse(input)
    },
};

/// Toshiba TC74LVX04FT
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC74LVX04FT.parse("LVX 04 8 45").is_ok());
/// ```
pub static TOSHIBA_TC74LVX04FT: NomParser<GenericPart> = NomParser {
    name: "Toshiba TC74LVX04FT",
    f: |input| {
        tuple((tag("LVX 04 "), year1, char(' '), week2))
            .map(|(_, year, _, week)| GenericPart {
                kind: "TC74LVX04FT".to_owned(),
                manufacturer: Some(Manufacturer::Toshiba),
                date_code: Some(PartDateCode::YearWeek { year, week }),
            })
            .parse(input)
    },
};

fn tc53<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    unknown: &'static str,
    package: Package,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        tag("TOSHIBA "),
        tuple((
            year2_week2,
            tag("EAI "),
            tag(chip_type),
            char(package.code_char()),
        )),
        char(' '),
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        tag(unknown),
        char(' '),
        uppers(1).and(digits(3)),
        tag(" JAPAN"),
    ))
    .map(
        |(_, (date_code, _, kind, package), _, rom_id, _, _, _, _, _)| MaskRom {
            rom_id: String::from(rom_id),
            chip_type: Some(format!("{kind}{package}")),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(date_code),
        },
    )
}

/// Toshiba TC531001 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC531001.parse("TOSHIBA 9144EAI TC531001CF DMG-FAE-0 C1 J619 JAPAN").is_ok());
/// ```
pub static TOSHIBA_TC531001: NomParser<MaskRom> = NomParser {
    name: "Toshiba TC531001",
    f: |input| tc53("TC531001C", "C1", Package::SOP32).parse(input),
};

/// Toshiba TC532000 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC532000.parse("TOSHIBA 9114EAI TC532000BF DMG-GWJ-0 D1 J542 JAPAN").is_ok());
/// ```
pub static TOSHIBA_TC532000: NomParser<MaskRom> = NomParser {
    name: "Toshiba TC532000",
    f: |input| tc53("TC532000B", "D1", Package::SOP32).parse(input),
};

/// Toshiba TC534000 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC534000.parse("TOSHIBA 9301EAI TC534000BF DMG-MQE-2 E1 N516 JAPAN").is_ok());
/// assert!(parser::toshiba::TOSHIBA_TC534000.parse("TOSHIBA 9614EAI TC534000DF DMG-WJA-0 E1 N750 JAPAN").is_ok());
/// ```
pub static TOSHIBA_TC534000: NomParser<MaskRom> = NomParser {
    name: "Toshiba TC534000",
    f: |input| {
        alt((
            tc53("TC534000B", "E1", Package::SOP32),
            tc53("TC534000D", "E1", Package::SOP32),
        ))
        .parse(input)
    },
};

/// Toshiba TC55V200 (TSOP-I-48, 2.7-3.6V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC55V200.parse("K13529 JAPAN 0106 MAD TC55V200 FT-70").is_ok());
/// ```
pub static TOSHIBA_TC55V200: NomParser<Ram> = NomParser {
    name: "Toshiba TC55V200",
    f: |input| {
        tuple((
            uppers(1).and(digits(5)),
            tag(" JAPAN "),
            year2_week2,
            tag(" MAD "),
            tag("TC55V200"),
            char(' '),
            tag("FT-").and(alt((tag("70"), tag("85"), tag("10")))),
        ))
        .map(|(_, _, date_code, _, kind, _, (_, speed))| Ram {
            kind: format!("{kind}FT-{speed}"),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Toshiba SGB mask ROM
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_SGB_ROM.parse("SYS-SGB-2 © 1994 Nintendo TC532000BF-N807 JAPAN 9431EAI").is_ok());
/// ```
pub static TOSHIBA_SGB_ROM: NomParser<MaskRom> = NomParser {
    name: "Toshiba SGB ROM",
    f: |input| {
        tuple((
            terminated(tag("SYS-SGB-2"), tag(" © 1994 Nintendo")),
            char(' '),
            recognize(tag("TC532000B").and(char(Package::SOP32.code_char())))
                .and(char('-').and(uppers(1)).and(digits(3))),
            tag(" JAPAN "),
            year2_week2,
            tag("EAI"),
        ))
        .map(|(rom_id, _, (kind, _), _, date_code, _)| MaskRom {
            rom_id: String::from(rom_id),
            chip_type: Some(String::from(kind)),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Package {
    SOP20,
    SOP32,
}

impl Package {
    pub const fn code_char(&self) -> char {
        match self {
            Package::SOP20 => 'M',
            Package::SOP32 => 'F',
        }
    }
}
