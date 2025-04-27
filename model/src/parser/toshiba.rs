// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser,
    branch::alt,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{recognize, value},
    error::ParseError,
    sequence::{delimited, separated_pair, terminated},
};

use super::{
    GameMaskRom, GameRomType, GenericPart, Manufacturer, Mapper, MapperChip, MaskRom, NomParser,
    PartDateCode,
    for_nom::{
        cgb_rom_code, digits, dmg_rom_code, lines2, lines3, lines4, uppers, week2, year1,
        year2_week2,
    },
};

/// Toshiba TC8521AM RTC (SOP-20)
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
) -> impl Parser<&'a str, Output = GenericPart, Error = E> {
    lines2(
        delimited(tag("T "), year2_week2, tag("HB")),
        tag("8521A").and(nom::character::streaming::char(package.code_char())),
    )
    .map(move |(date_code, (_, package_code))| GenericPart {
        kind: format!("TC8521A{package_code}"),
        manufacturer: Some(Manufacturer::Toshiba),
        date_code: Some(date_code),
    })
}

/// Toshiba TC7W139F line decoder
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC7W139F.parse("7W139 0J").is_ok());
/// ```
pub static TOSHIBA_TC7W139F: NomParser<GenericPart> = NomParser {
    name: "Toshiba TC7W139F",
    f: |input| {
        lines2(
            alt((
                value("TC7W139FU", tag("7W139")),
                value("TC7W139F", tag("7W139F")),
            )),
            year1.and(uppers(1)),
        )
        .map(|(kind, (year, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(PartDateCode::Year { year }),
        })
        .parse(input)
    },
};

/// Toshiba TC74LVX04FT hex inverter
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC74LVX04FT.parse("LVX 04 8 45").is_ok());
/// ```
pub static TOSHIBA_TC74LVX04FT: NomParser<GenericPart> = NomParser {
    name: "Toshiba TC74LVX04FT",
    f: |input| {
        lines3(
            tag("LVX"),
            tag("04"),
            separated_pair(year1, char(' '), week2),
        )
        .map(|(_, _, (year, week))| GenericPart {
            kind: "TC74LVX04FT".to_owned(),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(PartDateCode::YearWeek { year, week }),
        })
        .parse(input)
    },
};

fn tc53<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    rom_type: GameRomType,
    package: Package,
) -> impl Parser<&'a str, Output = GameMaskRom, Error = E> {
    lines4(
        separated_pair(
            tag("TOSHIBA"),
            char(' '),
            terminated(year2_week2, tag("EAI")),
        ),
        recognize(tag(chip_type).and(char(package.code_char()))),
        separated_pair(
            alt((dmg_rom_code(), cgb_rom_code())),
            char(' '),
            tag(rom_type.as_str()),
        ),
        separated_pair(uppers(1).and(digits(3)), char(' '), tag("JAPAN")),
    )
    .map(move |((_, date_code), kind, (rom_id, _), _)| GameMaskRom {
        rom_id: String::from(rom_id),
        rom_type,
        manufacturer: Some(Manufacturer::Toshiba),
        chip_type: Some(String::from(kind)),
        mask_code: None,
        date_code: Some(date_code),
    })
}

/// Toshiba TC531001 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC531001.parse("TOSHIBA 9144EAI TC531001CF DMG-FAE-0 C1 J619 JAPAN").is_ok());
/// ```
pub static TOSHIBA_TC531001: NomParser<GameMaskRom> = NomParser {
    name: "Toshiba TC531001",
    f: |input| tc53("TC531001C", GameRomType::C1, Package::SOP32).parse(input),
};

/// Toshiba TC532000 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC532000.parse("TOSHIBA 9114EAI TC532000BF DMG-GWJ-0 D1 J542 JAPAN").is_ok());
/// ```
pub static TOSHIBA_TC532000: NomParser<GameMaskRom> = NomParser {
    name: "Toshiba TC532000",
    f: |input| tc53("TC532000B", GameRomType::D1, Package::SOP32).parse(input),
};

/// Toshiba TC534000 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC534000.parse("TOSHIBA 9301EAI TC534000BF DMG-MQE-2 E1 N516 JAPAN").is_ok());
/// assert!(parser::toshiba::TOSHIBA_TC534000.parse("TOSHIBA 9614EAI TC534000DF DMG-WJA-0 E1 N750 JAPAN").is_ok());
/// ```
pub static TOSHIBA_TC534000: NomParser<GameMaskRom> = NomParser {
    name: "Toshiba TC534000",
    f: |input| {
        alt((
            tc53("TC534000B", GameRomType::E1, Package::SOP32),
            tc53("TC534000D", GameRomType::E1, Package::SOP32),
        ))
        .parse(input)
    },
};

/// Toshiba TC55V200 SRAM (TSOP-I-48, 2.7-3.6V, 2 Mibit / 256 KiB / 128x16)
///
/// Source:
///     "TC55V200FT/TR/UB-70,-85,-10 - 131,072-WORD BY 16-BIT FULL CMOS STATIC RAM"
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TC55V200.parse("K13529 JAPAN 0106 MAD TC55V200 FT-70").is_ok());
/// ```
pub static TOSHIBA_TC55V200: NomParser<GenericPart> = NomParser {
    name: "Toshiba TC55V200",
    f: |input| {
        lines4(
            uppers(1).and(digits(5)),
            (tag("JAPAN"), char(' '), year2_week2, char(' '), tag("MAD")),
            tag("TC55V200"),
            tag("FT-").and(alt((tag("70"), tag("85"), tag("10")))),
        )
        .map(
            |(_, (_, _, date_code, _, _), kind, (_, speed))| GenericPart {
                kind: format!("{kind}FT-{speed}"),
                manufacturer: Some(Manufacturer::Toshiba),
                date_code: Some(date_code),
            },
        )
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
        lines4(
            tag("SYS-SGB-2"),
            tag("© 1994 Nintendo"),
            recognize(tag("TC532000B").and(char(Package::SOP32.code_char())))
                .and(char('-').and(uppers(1)).and(digits(3))),
            separated_pair(tag("JAPAN"), char(' '), terminated(year2_week2, tag("EAI"))),
        )
        .map(|(rom_id, _, (kind, _), (_, date_code))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Toshiba),
            chip_type: Some(String::from(kind)),
            mask_code: None,
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

/// Toshiba TAMA5
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TAMA5.parse("TAMA5 9726 EAD1").is_ok());
/// ```
pub static TOSHIBA_TAMA5: NomParser<Mapper> = NomParser {
    name: "Toshiba TAMA5",
    f: |input| {
        lines2(
            tag("TAMA5"),
            terminated(year2_week2, tag(" EA").and(uppers(1)).and(tag("1"))),
        )
        .map(|(_, date_code)| Mapper {
            kind: MapperChip::Tama5,
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Toshiba TAMA6
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::toshiba::TOSHIBA_TAMA6.parse("TAMA6 JAPAN 47C243M FV61 9751H").is_ok());
/// ```
pub static TOSHIBA_TAMA6: NomParser<GenericPart> = NomParser {
    name: "Toshiba TAMA6",
    f: |input| {
        lines2(
            tag("TAMA6 JAPAN"),
            delimited(tag("47C243M FV61 "), year2_week2, tag("H")),
        )
        .map(|(_, date_code)| GenericPart {
            kind: "TAMA6".to_owned(),
            manufacturer: Some(Manufacturer::Toshiba),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
