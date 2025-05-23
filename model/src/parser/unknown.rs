// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser,
    branch::alt,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::opt,
    sequence::{delimited, preceded, separated_pair, terminated},
};

use super::{
    Crystal, GameMaskRom, GameRomType, GenericPart, Mapper, MapperChip, MaskRom, NomParser,
    PartDateCode,
    for_nom::{
        alnum_uppers, digits, lines2, lines3, lines4, uppers, year1, year1_month2, year1_week2,
        year2_week2,
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_SGB_ROM.parse("SYS-SGB-2 JAPAN © 1994 Nintendo 427A2 A04 NND").is_ok());
/// assert!(parser::unknown::UNKNOWN_SGB_ROM.parse("SYS-SGB-2 © 1994 Nintendo 9423 E").is_ok());
/// ```
pub static UNKNOWN_SGB_ROM: NomParser<MaskRom> = NomParser {
    name: "Unknown SGB ROM",
    f: |input| {
        alt((
            lines3(
                separated_pair(tag("SYS-SGB-2"), char(' '), tag("JAPAN")),
                tag("© 1994 Nintendo"),
                (
                    alnum_uppers(5),
                    char(' '),
                    alnum_uppers(3),
                    char(' '),
                    uppers(3),
                ),
            )
            .map(|((rom_id, _), _, _)| MaskRom {
                rom_id: String::from(rom_id),
                manufacturer: None,
                chip_type: None,
                mask_code: None,
                date_code: None,
            }),
            lines3(
                tag("SYS-SGB-2"),
                tag("© 1994 Nintendo"),
                separated_pair(year2_week2, char(' '), uppers(1)),
            )
            .map(|(rom_id, _, (date_code, _))| MaskRom {
                rom_id: String::from(rom_id),
                manufacturer: None,
                chip_type: None,
                mask_code: None,
                date_code: Some(date_code),
            }),
        ))
        .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_LCS5_EEPROM.parse("LCS5 040").is_ok());
/// assert!(parser::unknown::UNKNOWN_LCS5_EEPROM.parse("LCS5 435 09").is_ok());
/// ```
pub static UNKNOWN_LCS5_EEPROM: NomParser<GenericPart> = NomParser {
    name: "Unknown LCS5 EEPROM",
    f: |input| {
        (
            tag("LCS5 "),
            year1_week2,
            opt((nom::character::complete::char(' '), digits(2))),
        )
            .map(|(_, date_code, _)| GenericPart {
                kind: "LC56".to_owned(),
                manufacturer: None,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_LC56_EEPROM.parse("LC56 W617 08").is_ok());
/// ```
pub static UNKNOWN_LC56_EEPROM: NomParser<GenericPart> = NomParser {
    name: "Unknown LC56 EEPROM",
    f: |input| {
        lines3(tag("LC56"), uppers(1).and(digits(3)), digits(2))
            .map(|(_, _, _)| GenericPart {
                kind: "LC56".to_owned(),
                manufacturer: None,
                date_code: None,
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_AGS_CHARGE_CONTROLLER.parse("2253B 3129").is_ok());
/// ```
pub static UNKNOWN_AGS_CHARGE_CONTROLLER: NomParser<GenericPart> = NomParser {
    name: "Unknown AGS charge controller",
    f: |input| {
        lines2(tag("2253B"), (digits(1), alnum_uppers(1), digits(2)))
            .map(|(kind, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: None,
                date_code: None,
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_CRYSTAL_32_KIHZ.parse("32K09").is_ok());
/// assert!(parser::unknown::UNKNOWN_CRYSTAL_32_KIHZ.parse("32K0Z").is_ok());
/// ```
pub static UNKNOWN_CRYSTAL_32_KIHZ: NomParser<Crystal> = NomParser {
    name: "Unknown crystal, 32 KiHz",
    f: |input| {
        tag("32K")
            .and(year1.and(alnum_uppers(1)))
            .map(|(_, (year, _))| Crystal {
                manufacturer: None,
                frequency: Crystal::FREQ_32_KIHZ,
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_DMG_CRYSTAL_4_MIHZ.parse("4.19C59").is_ok());
/// ```
pub static UNKNOWN_DMG_CRYSTAL_4_MIHZ: NomParser<Crystal> = NomParser {
    name: "Unknown DMG crystal, 4 MiHz",
    f: |input| {
        tag("4.19C")
            .and(year1.and(alnum_uppers(1)))
            .map(|(_, (year, _))| Crystal {
                manufacturer: None,
                frequency: Crystal::FREQ_4_MIHZ,
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_MGB_CRYSTAL_4_MIHZ.parse("4.1943 RVR 841").is_ok());
/// assert!(parser::unknown::UNKNOWN_MGB_CRYSTAL_4_MIHZ.parse("4.1943 9752").is_ok());
/// ```
pub static UNKNOWN_MGB_CRYSTAL_4_MIHZ: NomParser<Crystal> = NomParser {
    name: "Unknown MGB crystal, 4 MiHz",
    f: |input| {
        lines2(
            tag("4.1943"),
            alt((preceded(tag("RVR "), year1_week2), year2_week2)),
        )
        .map(|(_, date_code)| Crystal {
            manufacturer: None,
            frequency: Crystal::FREQ_4_MIHZ,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_OXY_U4.parse("AKV 522").is_ok());
/// ```
pub static UNKNOWN_OXY_U4: NomParser<GenericPart> = NomParser {
    name: "Unknown OXY U4",
    f: |input| {
        lines2(tag("AKV"), year1_week2)
            .map(|(kind, date_code)| GenericPart {
                kind: String::from(kind),
                manufacturer: None,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_OXY_U5.parse("CP6465 B 02 KOR0531 635963").is_ok());
/// ```
pub static UNKNOWN_OXY_U5: NomParser<GenericPart> = NomParser {
    name: "Unknown OXY U5",
    f: |input| {
        lines4(
            tag("CP6465"),
            separated_pair(tag("B"), char(' '), char('0').and(digits(1))),
            preceded(tag("KOR"), year2_week2),
            digits(6),
        )
        .map(|(kind, _, date_code, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Unknown MBC1B (SOP-24)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_MBC1B.parse("Nintendo DMG MBC1B 8940AJ").is_ok());
/// ```
pub static UNKNOWN_MBC1B: NomParser<Mapper> = NomParser {
    name: "Unknown MBC1B",
    f: |input| {
        lines3(
            tag("Nintendo"),
            tag("DMG MBC1B"),
            terminated(year2_week2, tag("AJ")),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc1B,
            manufacturer: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Unknown MMM01 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_MMM01.parse("MMM01 645 113").is_ok());
/// ```
pub static UNKNOWN_MMM01: NomParser<Mapper> = NomParser {
    name: "Unknown MMM01",
    f: |input| {
        lines2(
            tag("MMM01"),
            terminated(year1_week2, tag(" ").and(digits(3))),
        )
        .map(|(_, date_code)| Mapper {
            kind: MapperChip::Mmm01,
            manufacturer: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Unknown TAMA7
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_TAMA7.parse("TAMA7 B9748 43913A TAIWAN").is_ok());
/// ```
pub static UNKNOWN_TAMA7: NomParser<GameMaskRom> = NomParser {
    name: "Unknown TAMA7",
    f: |input| {
        lines4(
            tag("TAMA7"),
            preceded(uppers(1), year2_week2),
            digits(5).and(uppers(1)),
            tag("TAIWAN"),
        )
        .map(|(_, date_code, _, _)| GameMaskRom {
            rom_id: String::from("DMG-AOMJ-0"),
            rom_type: GameRomType::E1,
            manufacturer: None,
            chip_type: None,
            mask_code: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Unknown LCD Screen
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::unknown::UNKNOWN_LCD_SCREEN.parse("T61102S T61104").is_ok());
/// ```
pub static UNKNOWN_LCD_SCREEN: NomParser<PartDateCode> = NomParser {
    name: "Unknown LCD Screen",
    f: |input| delimited(tag("T61102S T"), year1_month2, digits(2)).parse(input),
};
