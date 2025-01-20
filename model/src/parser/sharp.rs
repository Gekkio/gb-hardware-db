// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::{complete::one_of, streaming::char},
    combinator::{consumed, opt, recognize, value},
    error::ParseError,
    sequence::{delimited, separated_pair, terminated, tuple},
    Parser,
};

use super::{
    for_nom::{
        alnum_uppers, alphas, cgb_rom_code, digits, dmg_rom_code, lines3, lines4, lines5,
        satisfy_m_n_complete, uppers, year2_week2,
    },
    GameMaskRom, GameRomType, GenericPart, Manufacturer, Mapper, MapperChip, MaskCode, MaskRom,
    NomParser,
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3E02.parse("DMG-REG IR3E02 9527 CB").is_ok());
/// assert!(parser::sharp::SHARP_IR3E02.parse("DMG-REG IR3E02 9820 n").is_ok());
/// assert!(parser::sharp::SHARP_IR3E02.parse("DMG-REG IR3E02 9024 J").is_ok());
/// ```
pub static SHARP_IR3E02: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3E02",
    f: |input| ir3_old("DMG-REG", "IR3E02").parse(input),
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3E06.parse("CGB-REG IR3E06N 9839 C").is_ok());
/// assert!(parser::sharp::SHARP_IR3E06.parse("CGB-REG IR3E06N 0046 A").is_ok());
/// ```
pub static SHARP_IR3E06: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3E06",
    f: |input| ir3("CGB-REG", "IR3E06", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0104 C").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0141 K").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0204 d").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N AA24 A").is_ok());
/// assert!(parser::sharp::SHARP_IR3E09.parse("AGB-REG IR3E09N 0223 B").is_ok());
/// ```
pub static SHARP_IR3E09: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3E09",
    f: |input| ir3("AGB-REG", "IR3E09", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R40.parse("DMG-AMP IR3R40 9222 AA").is_ok());
/// assert!(parser::sharp::SHARP_IR3R40.parse("DMG-AMP IR3R40 8909 A").is_ok());
/// ```
pub static SHARP_IR3R40: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R40",
    f: |input| ir3_old("DMG-AMP", "IR3R40").parse(input),
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R53.parse("AMP MGB IR3R53N 9806 a").is_ok());
/// assert!(parser::sharp::SHARP_IR3R53.parse("AMP MGB IR3R53N 9724 C").is_ok());
/// ```
pub static SHARP_IR3R53: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R53",
    f: |input| ir3("AMP MGB", "IR3R53", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R56.parse("AMP MGB IR3R56N 0046 A").is_ok());
/// assert!(parser::sharp::SHARP_IR3R56.parse("AMP MGB IR3R56N 0040 C").is_ok());
/// ```
pub static SHARP_IR3R56: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R56",
    f: |input| ir3("AMP MGB", "IR3R56", Package::Ssop18).parse(input),
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_IR3R60.parse("AMP AGB IR3R60N 0103 a").is_ok());
/// assert!(parser::sharp::SHARP_IR3R60.parse("AMP AGB IR3R60N 0240 N").is_ok());
/// ```
pub static SHARP_IR3R60: NomParser<GenericPart> = NomParser {
    name: "Sharp IR3R60",
    f: |input| ir3("AMP AGB", "IR3R60", Package::Ssop18).parse(input),
};

fn ir3<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    kind: &'static str,
    pkg: Package,
) -> impl Parser<&'a str, GenericPart, E> {
    lines3(
        tag(prefix),
        recognize(tag(kind).and(package(pkg))),
        separated_pair(year2_week2, char(' '), alphas(1)),
    )
    .map(|(_, kind, (date_code, _))| GenericPart {
        kind: String::from(kind),
        manufacturer: Some(Manufacturer::Sharp),
        date_code: Some(date_code),
    })
}

fn ir3_old<'a, E: ParseError<&'a str>>(
    prefix: &'static str,
    kind: &'static str,
) -> impl Parser<&'a str, GenericPart, E> {
    lines3(
        tag(prefix),
        tag(kind),
        separated_pair(
            year2_week2,
            char(' '),
            alphas(1).and(opt(nom::character::complete::satisfy(|c| {
                c.is_ascii_uppercase()
            }))),
        ),
    )
    .map(|(_, kind, (date_code, _))| GenericPart {
        kind: String::from(kind),
        manufacturer: Some(Manufacturer::Sharp),
        date_code: Some(date_code),
    })
}

fn package<'a, E: ParseError<&'a str>>(package: Package) -> impl Parser<&'a str, Package, E> {
    value(package, tag(package.code()))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Ssop18,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Ssop18 => "N",
        }
    }
}

/// Sharp unknown mask ROM (glop top, 256 Kibit / 32 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MASK_ROM_GLOP_TOP_28_256_KIBIT.parse("LR0G150 DMG-TRA-1 97141").is_ok());
/// ```
pub static SHARP_MASK_ROM_GLOP_TOP_28_256_KIBIT: NomParser<GameMaskRom> = NomParser {
    name: "Sharp mask ROM",
    f: |input| {
        tuple((
            tag("LR0G150"),
            char(' '),
            dmg_rom_code(),
            char(' '),
            year2_week2,
            digits(1),
        ))
        .map(|(_, _, rom_id, _, date_code, _)| GameMaskRom {
            rom_id: String::from(rom_id),
            rom_type: GameRomType::GlopTop,
            manufacturer: Some(Manufacturer::Sharp),
            chip_type: None,
            mask_code: None,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn lh53_ancient<'a, E: ParseError<&'a str>>(
    kind: Option<&'static str>,
    rom_type: GameRomType,
    unknown: char,
) -> impl Parser<&'a str, GameMaskRom, E> {
    lines4(
        dmg_rom_code(),
        tag("SHARP"),
        tag("JAPAN"),
        tuple((year2_week2, char(' '), alphas(1), char(' '), char(unknown))),
    )
    .map(move |(rom_id, _, _, (date_code, _, _, _, _))| GameMaskRom {
        rom_id: String::from(rom_id),
        rom_type,
        manufacturer: Some(Manufacturer::Sharp),
        chip_type: kind.map(String::from),
        mask_code: None,
        date_code: Some(date_code),
    })
}

fn lh53_old<'a, E: ParseError<&'a str>>(
    kind: Option<&'static str>,
    rom_type: GameRomType,
) -> impl Parser<&'a str, GameMaskRom, E> {
    lines4(
        dmg_rom_code(),
        tag("SHARP"),
        separated_pair(tag("JAPAN"), char(' '), tag(rom_type.as_str())),
        separated_pair(year2_week2, char(' '), alphas(1)),
    )
    .map(move |(rom_id, _, _, (date_code, _))| GameMaskRom {
        rom_id: String::from(rom_id),
        rom_type,
        manufacturer: Some(Manufacturer::Sharp),
        chip_type: kind.map(String::from),
        mask_code: None,
        date_code: Some(date_code),
    })
}

fn lh53_new<'a, E: ParseError<&'a str>>(
    model: impl Parser<&'a str, Option<&'a str>, E>,
    rom_type: GameRomType,
) -> impl Parser<&'a str, GameMaskRom, E> {
    lines4(
        alt((dmg_rom_code(), cgb_rom_code())),
        separated_pair(
            tag("S"),
            char(' '),
            consumed(terminated(model, alnum_uppers(2))),
        ),
        separated_pair(tag("JAPAN"), char(' '), tag(rom_type.as_str())),
        separated_pair(year2_week2, char(' '), alphas(1)),
    )
    .map(
        move |(rom_id, (_, (mask_code, kind)), _, (date_code, _))| GameMaskRom {
            rom_id: String::from(rom_id),
            rom_type,
            manufacturer: Some(Manufacturer::Sharp),
            chip_type: kind.map(String::from),
            mask_code: Some(MaskCode::Sharp(String::from(mask_code))),
            date_code: Some(date_code),
        },
    )
}

/// Sharp LH53259M mask ROM (QFP-44, 256 Kibit / 32 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH53259M.parse("DMG-AWA-0 SHARP JAPAN 8909 D A").is_ok());
/// assert!(parser::sharp::SHARP_LH53259M.parse("DMG-AWA-0 SHARP JAPAN A0 8938 D").is_ok());
/// assert!(parser::sharp::SHARP_LH53259M.parse("DMG-OPX-0 S LH5359UZ JAPAN A0 9722 D").is_ok());
/// ```
pub static SHARP_LH53259M: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH53259",
    f: |input| {
        alt((
            lh53_ancient(Some("LH53259"), GameRomType::A0, 'A'),
            lh53_old(Some("LH53259"), GameRomType::A0),
            lh53_new(
                // Sharp Memory Data Book 1992
                value(Some("LH53259"), tag("LH5359")),
                GameRomType::A0,
            ),
        ))
        .parse(input)
    },
};

/// Sharp LH53515M mask ROM (QFP-44, 512 Kibit / 64 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH53515M.parse("DMG-CVJ-0 SHARP JAPAN B0 8941 D").is_ok());
/// ```
pub static SHARP_LH53515M: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH53515",
    f: |input| lh53_old(Some("LH53515"), GameRomType::B0).parse(input),
};

/// Sharp LH53514Z mask ROM (SOP-32, 512 Kibit / 64 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH53514Z.parse("DMG-AYJ-0 S LH5314H1 JAPAN B1 9014 E").is_ok());
/// ```
pub static SHARP_LH53514Z: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH53514",
    f: |input| {
        lh53_new(
            // reasonable guess
            value(Some("LH53514"), tag("LH5314")),
            GameRomType::B1,
        )
        .parse(input)
    },
};

/// Sharp LH53517Z mask ROM (SOP-32, 512 Kibit / 64 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH53517Z.parse("DMG-AYNP-0 S LH5317VR JAPAN B1 9850 E").is_ok());
/// ```
pub static SHARP_LH53517Z: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH53517",
    f: |input| {
        lh53_new(
            // reasonable guess
            value(Some("LH53517"), tag("LH5317")),
            GameRomType::B1,
        )
        .parse(input)
    },
};

/// Sharp LH530800N (SOP-32, 512 Kibit / 64 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH530800N.parse("DMG-A6W-0 S LH531HF8 JAPAN C1 9709 E").is_ok());
/// ```
pub static SHARP_LH530800N: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH530800",
    f: |input| {
        lh53_new(
            alt((
                // reasonable guess
                value(Some("LH530800"), tag("LH5308")),
                // Sharp Memory Data Book 1992
                value(Some("LH530800A"), tag("LH531H")),
            )),
            GameRomType::C1,
        )
        .parse(input)
    },
};

/// Sharp unknown mask ROM (SOP-32, 1 Mibit / 128 KiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MASK_ROM_SOP_32_1_MIBIT.parse("DMG-NME-0 SHARP JAPAN C1 9009 E").is_ok());
/// ```
pub static SHARP_MASK_ROM_SOP_32_1_MIBIT: NomParser<GameMaskRom> = NomParser {
    name: "Sharp mask ROM",
    f: |input| lh53_old(None, GameRomType::C1).parse(input),
};

/// Sharp LH532100N mask ROM (SOP-32, 2 Mibit / 256 KiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH532100N.parse("DMG-DFJ-0 S LH5321FL JAPAN D1 9249 D").is_ok());
/// ```
pub static SHARP_LH532100N: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH532100N",
    f: |input| lh53_new(value(Some("LH532100"), tag("LH5321")), GameRomType::D1).parse(input),
};

/// Sharp LH532xxxN mask ROM (SOP-32, 2 Mibit / 256 KiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH532XXXN.parse("DMG-DIJ-0 S LH532D17 JAPAN D1 9223 D").is_ok());
/// ```
pub static SHARP_LH532XXXN: NomParser<GameMaskRom> = NomParser {
    // maybe: LH532100 series / LH532300 / LH532700 series
    name: "Sharp LH532???",
    f: |input| {
        lh53_new(
            value(
                None,
                alt((
                    tag("LH532D"),
                    tag("LH532K"),
                    tag("LH532M"),
                    tag("LH532W"),
                    tag("LHMN2E"),
                )),
            ),
            GameRomType::D1,
        )
        .parse(input)
    },
};

/// Sharp LH534xxxN mask ROM (SOP-32, 4 Mibit / 512 KiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH534XXXN.parse("DMG-A3ME-0 S LH534MW1 JAPAN E1 9547 E").is_ok());
/// ```
pub static SHARP_LH534XXXN: NomParser<GameMaskRom> = NomParser {
    // maybe: LH534100 series / LH534300 series / LH534R00
    name: "Sharp LH534???",
    f: |input| {
        lh53_new(
            value(None, alt((tag("LH534M"), tag("LH5S4M"), tag("LHMN4M")))),
            GameRomType::E1,
        )
        .parse(input)
    },
};

/// Sharp LH538xxxN mask ROM (SOP-32, 8 Mibit / 1 MiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH538XXXN.parse("CGB-AHYE-0 S LH538WV9 JAPAN F1 9916 D").is_ok());
/// ```
pub static SHARP_LH538XXXN: NomParser<GameMaskRom> = NomParser {
    // maybe: LH538300 series / LH538400 series / LH538700 / LH538R00 series
    name: "Sharp LH538???",
    f: |input| {
        lh53_new(
            value(
                None,
                alt((
                    tag("LH538M"),
                    tag("LH538W"),
                    tag("LH5S8M"),
                    tag("LHMN8J"),
                    tag("LHMN8M"),
                )),
            ),
            GameRomType::F1,
        )
        .parse(input)
    },
};

/// Sharp LH534xxxS mask ROM (TSOP-I-32, 4 Mibit / 512 KiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH534XXXS.parse("DMG-HFAJ-0 S LHMN4MTI JAPAN E 9838 E").is_ok());
/// ```
pub static SHARP_LH534XXXS: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH534???",
    f: |input| lh53_new(value(None, tag("LHMN4M")), GameRomType::E).parse(input),
};

/// Sharp LH538xxxS mask ROM (TSOP-I-32, 8 Mibit / 1 MiB)
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH538XXXS.parse("DMG-HRCJ-0 S LH5S8MTI JAPAN F 9846 E").is_ok());
/// ```
pub static SHARP_LH538XXXS: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH538???",
    f: |input| lh53_new(value(None, tag("LH5S8M")), GameRomType::F).parse(input),
};

/// Sharp LH5316xxx mask ROM (TSOP-II-44, 16 Mibit / 2 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH5316XXX.parse("CGB-AFIP-0 S LH537MTJ JAPAN G2 9929 D").is_ok());
/// ```
pub static SHARP_LH5316XXX: NomParser<GameMaskRom> = NomParser {
    // maybe: LH5316400 / LH5316500 series / LH5316P00 series
    name: "Sharp LH5316???",
    f: |input| lh53_new(value(None, tag("LH537M")), GameRomType::G2).parse(input),
};

/// Sharp LH5332xxx mask ROM (TSOP-II-44, 32 Mibit / 4 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LH5332XXX.parse("CGB-AYQE-0 S LHMN5MTF JAPAN H2 0010 D").is_ok());
/// ```
pub static SHARP_LH5332XXX: NomParser<GameMaskRom> = NomParser {
    name: "Sharp LH5332???",
    f: |input| lh53_new(value(None, tag("LHMN5M")), GameRomType::H2).parse(input),
};

fn sgb_rom<'a, E: ParseError<&'a str>>(
    model: impl Parser<&'a str, (&'a str, Option<&'a str>), E>,
    rom_id: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    lines4(
        tag(rom_id),
        tag("© 1994 Nintendo"),
        model,
        separated_pair(year2_week2, char(' '), uppers(1)),
    )
    .map(|(rom_id, _, (mask_code, kind), (date_code, _))| MaskRom {
        rom_id: String::from(rom_id),
        chip_type: kind.map(String::from),
        manufacturer: Some(Manufacturer::Sharp),
        mask_code: Some(MaskCode::Sharp(String::from(mask_code))),
        date_code: Some(date_code),
    })
}

/// Sharp SGB mask ROM
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_SGB_ROM.parse("SYS-SGB-2 © 1994 Nintendo LH532M0M 9432 E").is_ok());
/// assert!(parser::sharp::SHARP_SGB_ROM.parse("SYS-SGB-2 © 1994 Nintendo LH532KND 9432 E").is_ok());
/// assert!(parser::sharp::SHARP_SGB_ROM.parse("SYS-SGB-NT © 1994 Nintendo LH532KN8 9416 D").is_ok());
/// ```
pub static SHARP_SGB_ROM: NomParser<MaskRom> = NomParser {
    name: "Sharp SGB ROM",
    f: |input| {
        alt((
            sgb_rom(
                consumed(value(Some("LH532100B"), tag("LH532K").and(tag("N8")))),
                "SYS-SGB-NT",
            ),
            sgb_rom(
                consumed(value(Some("LH532100B"), tag("LH532K").and(tag("ND")))),
                "SYS-SGB-2",
            ),
            sgb_rom(
                consumed(value(None, tag("LH532M").and(tag("0M")))),
                "SYS-SGB-2",
            ),
        ))
        .parse(input)
    },
};

/// Sharp SGB2 mask ROM
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_SGB2_ROM.parse("© 1998 Nintendo SYS-SGB2-10 LH5S4RY4 0003 D").is_ok());
/// ```
pub static SHARP_SGB2_ROM: NomParser<MaskRom> = NomParser {
    name: "Sharp SGB2 ROM",
    f: |input| {
        lines4(
            tag("© 1998 Nintendo"),
            tag("SYS-SGB2-10"),
            consumed(value(Some("LH534R00B"), tag("LH5S4R").and(tag("Y4")))),
            separated_pair(year2_week2, char(' '), uppers(1)),
        )
        .map(|(_, rom_id, (mask_code, kind), (date_code, _))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Sharp),
            chip_type: kind.map(String::from),
            mask_code: Some(MaskCode::Sharp(String::from(mask_code))),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn cic<'a, E: ParseError<&'a str>>(
    model: &'static str,
    copyright: &'static str,
) -> impl Parser<&'a str, GenericPart, E> {
    lines4(
        recognize(tag(model).and(opt(one_of("AB")))),
        tag(copyright),
        tag("Nintendo"),
        separated_pair(year2_week2, char(' '), alphas(1)),
    )
    .map(|(kind, _, _, (date_code, _))| GenericPart {
        kind: String::from(kind),
        manufacturer: Some(Manufacturer::Sharp),
        date_code: Some(date_code),
    })
}

/// Sharp F411
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_F411.parse("F411A © 1990 Nintendo 9428 a").is_ok());
/// ```
pub static SHARP_F411: NomParser<GenericPart> = NomParser {
    name: "Sharp F411",
    f: |input| cic("F411", "© 1990").parse(input),
};

/// Sharp F413
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_F413.parse("F413A © 1992 Nintendo 9425 a").is_ok());
/// ```
pub static SHARP_F413: NomParser<GenericPart> = NomParser {
    name: "Sharp F413",
    f: |input| cic("F413", "© 1992").parse(input),
};

/// Sharp LR35902 (QFP-80)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_LR35902.parse("DMG-CPU LR35902 8907 D").is_ok());
/// ```
pub static SHARP_LR35902: NomParser<GenericPart> = NomParser {
    name: "Sharp LR35902",
    f: |input| {
        lines3(
            tag("DMG-CPU"),
            tag("LR35902"),
            separated_pair(year2_week2, char(' '), uppers(1)),
        )
        .map(|(kind, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp DMG-CPU (QFP-80)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_DMG_CPU.parse("DMG-CPU © 1989 Nintendo JAPAN 8913 D").is_ok());
/// assert!(parser::sharp::SHARP_DMG_CPU.parse("DMG-CPU A © 1989 Nintendo JAPAN 8937 D").is_ok());
/// assert!(parser::sharp::SHARP_DMG_CPU.parse("DMG-CPU B © 1989 Nintendo JAPAN 9207 D").is_ok());
/// assert!(parser::sharp::SHARP_DMG_CPU.parse("DMG-CPU C © 1989 Nintendo JAPAN 9835 D").is_ok());
/// ```
pub static SHARP_DMG_CPU: NomParser<GenericPart> = NomParser {
    name: "Sharp DMG-CPU",
    f: |input| {
        lines4(
            alt((
                tag("DMG-CPU A"),
                tag("DMG-CPU B"),
                tag("DMG-CPU C"),
                tag("DMG-CPU"),
            )),
            tag("© 1989 Nintendo"),
            tag("JAPAN"),
            separated_pair(
                year2_week2,
                char(' '),
                satisfy_m_n_complete(1, 2, |c| c.is_ascii_uppercase()),
            ),
        )
        .map(|(kind, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp DMG-CPU (glop top)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_DMG_CPU_GLOP_TOP.parse("B").is_ok());
/// assert!(parser::sharp::SHARP_DMG_CPU_GLOP_TOP.parse("C").is_ok());
/// ```
pub static SHARP_DMG_CPU_GLOP_TOP: NomParser<GenericPart> = NomParser {
    name: "Sharp DMG-CPU glop top",
    f: |input| {
        alt((
            value("DMG-CPU B (blob)", tag("B")),
            value("DMG-CPU C (blob)", tag("C")),
        ))
        .map(|kind| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: None,
        })
        .parse(input)
    },
};

/// Sharp SGB-CPU (QFP-80)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_SGB_CPU.parse("SGB-CPU 01 © 1994 Nintendo Ⓜ 1989 Nintendo JAPAN 9434 7 D").is_ok());
/// ```
pub static SHARP_SGB_CPU: NomParser<GenericPart> = NomParser {
    name: "Sharp SGB-CPU",
    f: |input| {
        lines5(
            tag("SGB-CPU 01"),
            tag("© 1994 Nintendo"),
            tag("Ⓜ 1989 Nintendo"),
            tag("JAPAN"),
            separated_pair(
                year2_week2,
                char(' '),
                digits(1)
                    .and(opt(nom::character::complete::char(' ')))
                    .and(uppers(1)),
            ),
        )
        .map(|(kind, _, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU MGB (QFP-80)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_MGB.parse("CPU MGB Ⓜ © 1996 Nintendo JAPAN 9629 D").is_ok());
/// ```
pub static SHARP_CPU_MGB: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU MGB",
    f: |input| {
        lines4(
            tag("CPU MGB"),
            tag("Ⓜ © 1996 Nintendo"),
            tag("JAPAN"),
            separated_pair(
                year2_week2,
                char(' '),
                satisfy_m_n_complete(1, 2, |c| c.is_ascii_uppercase()),
            ),
        )
        .map(|(kind, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU SGB2 (QFP-80)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_SGB2.parse("CPU SGB2 Ⓜ 1996 Nintendo © 1997 Nintendo JAPAN 9810 7E").is_ok());
/// ```
pub static SHARP_CPU_SGB2: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU SGB2",
    f: |input| {
        lines5(
            tag("CPU SGB2"),
            tag("Ⓜ 1996 Nintendo"),
            tag("© 1997 Nintendo"),
            tag("JAPAN"),
            separated_pair(
                year2_week2,
                char(' '),
                digits(1)
                    .and(opt(nom::character::complete::char(' ')))
                    .and(uppers(1)),
            ),
        )
        .map(|(kind, _, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU CGB (QFP-128)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_CGB.parse("CPU CGB Ⓜ © 1998 Nintendo JAPAN 9832 I").is_ok());
/// assert!(parser::sharp::SHARP_CPU_CGB.parse("CPU CGB A Ⓜ © 1998 Nintendo JAPAN 9837 I").is_ok());
/// assert!(parser::sharp::SHARP_CPU_CGB.parse("CPU CGB B Ⓜ © 1998 Nintendo JAPAN 9840 I").is_ok());
/// assert!(parser::sharp::SHARP_CPU_CGB.parse("CPU CGB C Ⓜ © 1998 Nintendo JAPAN 9927 IA").is_ok());
/// assert!(parser::sharp::SHARP_CPU_CGB.parse("CPU CGB D Ⓜ © 1998 Nintendo JAPAN 0026 I").is_ok());
/// ```
pub static SHARP_CPU_CGB: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU CGB",
    f: |input| {
        lines4(
            alt((
                tag("CPU CGB A"),
                tag("CPU CGB B"),
                tag("CPU CGB C"),
                tag("CPU CGB D"),
                tag("CPU CGB"),
            )),
            tag("Ⓜ © 1998 Nintendo"),
            tag("JAPAN"),
            separated_pair(
                year2_week2,
                char(' '),
                satisfy_m_n_complete(1, 2, |c| c.is_ascii_uppercase()),
            ),
        )
        .map(|(kind, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU CGB E (QFP-128)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_CGB_E.parse("CPU CGB E Ⓜ © 2000 Nintendo JAPAN 0052 I").is_ok());
/// ```
pub static SHARP_CPU_CGB_E: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU CGB E",
    f: |input| {
        lines4(
            tag("CPU CGB E"),
            tag("Ⓜ © 2000 Nintendo"),
            tag("JAPAN"),
            separated_pair(
                year2_week2,
                char(' '),
                satisfy_m_n_complete(1, 2, |c| c.is_ascii_uppercase()),
            ),
        )
        .map(|(kind, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU AGB (QFP-128)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_AGB.parse("CPU AGB Ⓜ © 2000 Nintendo JAPAN ARM 0104 I").is_ok());
/// assert!(parser::sharp::SHARP_CPU_AGB.parse("CPU AGB A Ⓜ © 2000 Nintendo JAPAN ARM 0228 mE").is_ok());
/// assert!(parser::sharp::SHARP_CPU_AGB.parse("CPU AGB A E Ⓜ © 2000 Nintendo JAPAN ARM 0503 O").is_ok());
/// ```
pub static SHARP_CPU_AGB: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU AGB",
    f: |input| {
        lines4(
            alt((tag("CPU AGB A E"), tag("CPU AGB A"), tag("CPU AGB"))),
            tag("Ⓜ © 2000 Nintendo"),
            tag("JAPAN ARM"),
            separated_pair(
                year2_week2,
                char(' '),
                satisfy_m_n_complete(1, 2, |c| c.is_ascii_alphabetic()),
            ),
        )
        .map(|(kind, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU AGB B (QFP-156)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_AGB_B.parse("CPU AGB B Ⓜ © 2002 Nintendo JAPAN ARM 0311 mB").is_ok());
/// assert!(parser::sharp::SHARP_CPU_AGB_B.parse("CPU AGB B E Ⓜ © 2002 Nintendo JAPAN ARM 0602 UB").is_ok());
/// ```
pub static SHARP_CPU_AGB_B: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU AGB B",
    f: |input| {
        lines4(
            alt((tag("CPU AGB B E"), tag("CPU AGB B"))),
            tag("Ⓜ © 2002 Nintendo"),
            tag("JAPAN ARM"),
            separated_pair(
                year2_week2,
                char(' '),
                satisfy_m_n_complete(1, 2, |c| c.is_ascii_alphabetic()),
            ),
        )
        .map(|(kind, _, _, (date_code, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp CPU AGB E (BGA)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_CPU_AGB_E.parse("0529 2m CPU AGB E Ⓜ © 2004 Nintendo JAPAN ARM").is_ok());
/// ```
pub static SHARP_CPU_AGB_E: NomParser<GenericPart> = NomParser {
    name: "Sharp CPU AGB E",
    f: |input| {
        lines5(
            terminated(year2_week2, tag(" 2m")),
            tag("CPU AGB E"),
            tag("Ⓜ © 2004"),
            tag("Nintendo"),
            tag("JAPAN ARM"),
        )
        .map(|(date_code, kind, _, _, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC1
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC1.parse("DMG MBC1 Nintendo S 8914 T").is_ok());
/// ```
pub static SHARP_MBC1: NomParser<Mapper> = NomParser {
    name: "Sharp MBC1",
    f: |input| {
        lines4(
            tag("DMG"),
            value(MapperChip::Mbc1, tag("MBC1")),
            tag("Nintendo"),
            delimited(tag("S "), year2_week2, char(' ').and(uppers(1))),
        )
        .map(|(_, kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC1A
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC1A.parse("DMG MBC1A Nintendo S 9025 1 A").is_ok());
/// ```
pub static SHARP_MBC1A: NomParser<Mapper> = NomParser {
    name: "Sharp MBC1A",
    f: |input| {
        lines4(
            tag("DMG"),
            value(MapperChip::Mbc1A, tag("MBC1A")),
            tag("Nintendo"),
            delimited(
                tag("S "),
                year2_week2,
                tuple((char(' '), digits(1), char(' '), uppers(1))),
            ),
        )
        .map(|(_, kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC1B
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC1B.parse("DMG MBC1B Nintendo S 9107 5 A").is_ok());
/// ```
pub static SHARP_MBC1B: NomParser<Mapper> = NomParser {
    name: "Sharp MBC1B",
    f: |input| {
        lines4(
            tag("DMG"),
            value(MapperChip::Mbc1B, tag("MBC1B")),
            tag("Nintendo"),
            delimited(
                tag("S "),
                year2_week2,
                tuple((
                    char(' '),
                    digits(1),
                    char(' '),
                    satisfy_m_n_complete(1, 2, |c| c.is_ascii_uppercase()),
                )),
            ),
        )
        .map(|(_, kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC1B1
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC1B1.parse("DMG MBC1B1 Nintendo S 9838 5 A").is_ok());
/// ```
pub static SHARP_MBC1B1: NomParser<Mapper> = NomParser {
    name: "Sharp MBC1B1",
    f: |input| {
        lines4(
            tag("DMG"),
            value(MapperChip::Mbc1B1, tag("MBC1B1")),
            tag("Nintendo"),
            delimited(
                tag("S "),
                year2_week2,
                tuple((char(' '), digits(1), char(' '), uppers(1))),
            ),
        )
        .map(|(_, kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC2A
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC2A.parse("DMG MBC2A Nintendo S 9730 5 AB").is_ok());
/// ```
pub static SHARP_MBC2A: NomParser<Mapper> = NomParser {
    name: "Sharp MBC2A",
    f: |input| {
        lines4(
            tag("DMG"),
            value(MapperChip::Mbc2A, tag("MBC2A")),
            tag("Nintendo"),
            delimited(
                tag("S "),
                year2_week2,
                tuple((
                    char(' '),
                    digits(1),
                    char(' '),
                    satisfy_m_n_complete(1, 2, |c| c.is_ascii_uppercase()),
                )),
            ),
        )
        .map(|(_, kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC3
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC3.parse("MBC3 LR385364 9743 A").is_ok());
/// ```
pub static SHARP_MBC3: NomParser<Mapper> = NomParser {
    name: "Sharp MBC3",
    f: |input| {
        lines3(
            value(MapperChip::Mbc3, tag("MBC3")),
            tag("LR385364"),
            terminated(year2_week2, tuple((char(' '), uppers(1)))),
        )
        .map(|(kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC3A
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC3A.parse("MBC3 A LR38536B 9935 A").is_ok());
/// ```
pub static SHARP_MBC3A: NomParser<Mapper> = NomParser {
    name: "Sharp MBC3A",
    f: |input| {
        lines3(
            value(MapperChip::Mbc3A, tag("MBC3 A")),
            tag("LR38536B"),
            terminated(year2_week2, tuple((char(' '), uppers(1)))),
        )
        .map(|(kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Sharp MBC5
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::sharp::SHARP_MBC5.parse("MBC5 LZ9GB31 AL23 A").is_ok());
/// ```
pub static SHARP_MBC5: NomParser<Mapper> = NomParser {
    name: "Sharp MBC5",
    f: |input| {
        lines3(
            value(MapperChip::Mbc5, tag("MBC5")),
            tag("LZ9GB31"),
            terminated(year2_week2, tuple((char(' '), uppers(1)))),
        )
        .map(|(kind, _, date_code)| Mapper {
            kind,
            manufacturer: Some(Manufacturer::Sharp),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
