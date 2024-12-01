// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, satisfy},
    combinator::{opt, recognize},
    error::ParseError,
    sequence::tuple,
    IResult, Parser,
};

use super::{
    for_nom::{self, agb_rom_code, alnum_uppers, cgb_rom_code, digits, dmg_rom_code, uppers},
    GenericPart, Manufacturer, MaskRom, NomParser, PartDateCode,
};

/// Macronix MX29F008 (TSOP-I-40, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX29F008.parse("E991012 29F008TC-14 21534 TAIWAN").is_ok());
/// ```
pub static MACRONIX_MX29F008: NomParser<GenericPart> = NomParser {
    name: "Macronix MX29F008",
    f: |input| {
        tuple((
            tuple((
                assembly_vendor_code,
                date_code,
                tag("12"), // digits 3 and 4 of "product body" (?)
            )),
            char(' '),
            tag("29F008TC-14"),
            char(' '),
            lot_code_old,
            tag(" TAIWAN"),
        ))
        .map(|((_, date_code, _), _, kind, _, _, _)| GenericPart {
            kind: format!("MX{kind}"),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Macronix MX29L010 (TSOP-I-32, 3.3V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX29L010.parse("B063857G MX29L010TC-15A1 1H4751").is_ok());
/// assert!(parser::macronix::MACRONIX_MX29L010.parse("E032457 MX29L010TC-15A1 1E8980").is_ok());
/// assert!(parser::macronix::MACRONIX_MX29L010.parse("E023057 MX29L010TC-15 1E0290").is_ok());
/// assert!(parser::macronix::MACRONIX_MX29L010.parse("E040257 MX29L010TC-15A1 1F468900A0").is_ok());
/// ```
pub static MACRONIX_MX29L010: NomParser<GenericPart> = NomParser {
    name: "Macronix MX29L010",
    f: |input| {
        tuple((
            tuple((
                assembly_vendor_code,
                date_code,
                tag("57"),     // digits 3 and 4 of "product body" (?)
                opt(tag("G")), // green package?
            )),
            char(' '),
            alt((tag("MX29L010TC-15A1"), tag("MX29L010TC-15"))),
            char(' '),
            lot_code_new,
        ))
        .map(|((_, date_code, _, _), _, kind, _, _)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn agb_mx23l<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    product_body: &'static str,
    unknown: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        assembly_vendor_code,
        date_code,
        tag(product_body),
        tag("-M"),
        opt(tag("G")), // green package?
        char(' '),
        tag(chip_type),
        char(' '),
        agb_rom_code(),
        char(' '),
        tag(unknown),
        char(' '),
        lot_code_new,
    ))
    .map(
        |(_, date_code, _, _, _, _, kind, _, rom_id, _, _, _, _)| MaskRom {
            rom_id: String::from(rom_id),
            chip_type: Some(String::from(kind)),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        },
    )
}

/// Macronix MX23L8006 (TSOP-II-44, 3.3V, 1 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L8006.parse("M042021-M MX23L8006-12B AGB-FBMP-0 F2 2K151900").is_ok());
/// ```
pub static MACRONIX_MX23L8006: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L8006",
    f: |input| agb_mx23l("MX23L8006-12B", "21", "F2").parse(input),
};

/// Macronix MX23L3206 (TSOP-II-44, 3.3V, 4 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L3206.parse("M043821-M MX23L3206-12B AGB-BP9E-0 H2 2K194300").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23L3206.parse("S064421-MG MX23L3206-12B AGB-BG7E-0 H2 2T341304").is_ok());
/// ```
pub static MACRONIX_MX23L3206: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L3206",
    f: |input| agb_mx23l("MX23L3206-12B", "21", "H2").parse(input),
};

/// Macronix MX23L3406 (TSOP-II-44, 3.3V, 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L3406.parse("S035046-M MX23L3406-12C AGB-BBRX-0 I2 2I904402").is_ok());
/// ```
pub static MACRONIX_MX23L3406: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L3406",
    f: |input| agb_mx23l("MX23L3406-12C", "46", "I2").parse(input),
};

/// Macronix MX23L6406 (TSOP-II-44, 3.3V, 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L6406.parse("M022807-M MX23L6406-12B1 AGB-AGSF-0 I2 2E825103").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23L6406.parse("S051746-MG MX23L6406-12C AGB-BRKP-0 I2 2L261801").is_ok());
/// ```
pub static MACRONIX_MX23L6406: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L6406",
    f: |input| {
        alt((
            agb_mx23l("MX23L6406-12B", "07", "I2"),
            agb_mx23l("MX23L6406-12B1", "07", "I2"),
            agb_mx23l("MX23L6406-12C", "46", "I2"),
        ))
        .parse(input)
    },
};

/// Macronix MX23L6407 (TSOP-II-44, 3.3V, 8 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L6407.parse("S024358-M MX23L6407-12C AGB-AXPJ-0 I2 2G447800").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23L6407.parse("M053257-MG MX23L6407-12C1 AGB-KYGP-0 I2 2M219701A1").is_ok());
/// ```
pub static MACRONIX_MX23L6407: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L6407",
    f: |input| {
        alt((
            agb_mx23l("MX23L6407-12C", "58", "I2"),
            agb_mx23l("MX23L6407-12C1", "57", "I2"),
        ))
        .parse(input)
    },
};

/// Macronix MX23L12806 (TSOP-II-44, 3.3V, 16 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L12806.parse("E033938-M MX23L12806-12C AGB-BPPP-0 J2 2F478700").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23L12806.parse("S052638-MG MX23L12806-12C AGB-BPRS-0 J2 2M396503A1").is_ok());
/// ```
pub static MACRONIX_MX23L12806: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L12806",
    f: |input| agb_mx23l("MX23L12806-12C", "38", "J2").parse(input),
};

/// Macronix MX23L12807 (TSOP-II-44, 3.3V, 16 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L12807.parse("E055058-MG MX23L12807-12C AGB-BPES-0 J2 2N422000A1").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23L12807.parse("N032358-M MX23L12807-12C AGB-AXVS-0 J2 2H552600").is_ok());
/// ```
pub static MACRONIX_MX23L12807: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L12807",
    f: |input| agb_mx23l("MX23L12807-12C", "58", "J2").parse(input),
};

/// Macronix MX23L25607 (TSOP-II-44, 3.3V, 32 MiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23L25607.parse("E053953-MG MX23L25607-12D1 AGB-BE8P-0 K2 2N007800").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23L25607.parse("M064053-MG MX23L25607-12D2 AGB-BH3E-0 K2 2T151000").is_ok());
/// ```
pub static MACRONIX_MX23L25607: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23L25607",
    f: |input| {
        alt((
            agb_mx23l("MX23L25607-12D1", "53", "K2"),
            agb_mx23l("MX23L25607-12D2", "53", "K2"),
        ))
        .parse(input)
    },
};

fn dmg_mx23c_old<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    unknown: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        assembly_vendor_code,
        date_code,
        tag("-M"),
        char(' '),
        tag(chip_type),
        char(' '),
        dmg_rom_code(),
        char(' '),
        tag(unknown),
        char(' '),
        lot_code_old,
        uppers(1),
    ))
    .map(
        |(_, date_code, _, _, kind, _, rom_id, _, _, _, _, _)| MaskRom {
            rom_id: String::from(rom_id),
            chip_type: Some(String::from(kind)),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        },
    )
}

fn gb_mx23c<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    product_body: &'static str,
    unknown: &'static str,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        assembly_vendor_code,
        date_code,
        tag(product_body),
        tag("-M"),
        opt(tag("G")), // green package?
        char(' '),
        tag(chip_type),
        char(' '),
        alt((dmg_rom_code(), cgb_rom_code())),
        opt(tuple((char(' '), digits(2)))),
        char(' '),
        tag(unknown),
        char(' '),
        lot_code_new,
    ))
    .map(
        |(_, date_code, _, _, _, _, kind, _, rom_id, _, _, _, _, _)| MaskRom {
            rom_id: String::from(rom_id),
            chip_type: Some(String::from(kind)),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        },
    )
}

/// Macronix MX23C4002 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23C4002.parse("J9720-M MX23C4002-20 DMG-ATAJ-0 E1 43282F").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23C4002.parse("C983938-M MX23C4002-20 DMG-AD3E-1 E1 1P0221Y3").is_ok());
/// ```
pub static MACRONIX_MX23C4002: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23C4002",
    f: |input| {
        alt((
            dmg_mx23c_old("MX23C4002-20", "E1"),
            gb_mx23c("MX23C4002-20", "38", "E1"),
        ))
        .parse(input)
    },
};

/// Macronix MX23C8003 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23C8003.parse("S010649-M MX23C8003-20 DMG-BMAP-0 F1 1C3876A1").is_ok());
/// ```
pub static MACRONIX_MX23C8003: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23C8003",
    f: |input| gb_mx23c("MX23C8003-20", "49", "F1").parse(input),
};

/// Macronix MX23C8005 (SOP-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23C8005.parse("C010649-M MX23C8005-12 CGB-BHFE-0 F1 1C5450LB").is_ok());
/// ```
pub static MACRONIX_MX23C8005: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23C8005",
    f: |input| gb_mx23c("MX23C8005-12", "49", "F1").parse(input),
};

/// Macronix MX23C8006 (TSOP-I-32, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23C8006.parse("T991349-M MX23C8006-12 DMG-VPHJ-0 F 1A4891A2").is_ok());
/// ```
pub static MACRONIX_MX23C8006: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23C8006",
    f: |input| gb_mx23c("MX23C8006-12", "49", "F").parse(input),
};

/// Macronix MX23C1603 (TSOP-II-44, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23C1603.parse("E052804-MG MX23C1603-12A CGB-AAUK-0 G2 1D4499A2A1").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23C1603.parse("M994395-M MX23C1603-12 1 CGB-VYHE-0 G2 1Q6065A1").is_ok());
/// ```
pub static MACRONIX_MX23C1603: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23C1603",
    f: |input| {
        alt((
            gb_mx23c("MX23C1603-12 1", "95", "G2"),
            gb_mx23c("MX23C1603-12A", "04", "G2"),
            gb_mx23c("MX23C1603-12A", "19", "G2"),
        ))
        .parse(input)
    },
};

/// Macronix MX23C3203 (TSOP-II-44, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::macronix::MACRONIX_MX23C3203.parse("E034623-M MX23C3203-12A2 CGB-BY3D-0 H2 2G513304").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23C3203.parse("M004523-M MX23C3203-11A2 CGB-B82J-0 02 H2 2D224301").is_ok());
/// assert!(parser::macronix::MACRONIX_MX23C3203.parse("M002595-M MX23C3203-12 1 CGB-BY3J-0 H2 1R0833A1").is_ok());
/// ```
pub static MACRONIX_MX23C3203: NomParser<MaskRom> = NomParser {
    name: "Macronix MX23C3203",
    f: |input| {
        alt((
            gb_mx23c("MX23C3203-12 1", "95", "H2"),
            gb_mx23c("MX23C3203-12A2", "95", "H2"),
            gb_mx23c("MX23C3203-11A2", "23", "H2"),
            gb_mx23c("MX23C3203-12A2", "23", "H2"),
        ))
        .parse(input)
    },
};

fn assembly_vendor_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, char, E> {
    satisfy(|c| match c {
        'a' => true, // ChipMOS
        'B' => true, // OSE / Orient Semiconductor Electronics
        'C' => true, // ???
        'E' => true, // ???
        'K' => true, // ASEKS
        'J' => true, // ASEJ
        'L' => true, // LINGSEN
        'M' => true, // ???
        'N' => true, // ???
        'S' => true, // SPIL
        'T' => true, // STS
        'X' => true, // ASECL
        _ => false,
    })
    .parse(input)
}

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    for_nom::year2_week2(input)
}

fn lot_code_new<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    recognize(tuple((
        tuple((digits(1), alnum_uppers(1), digits(3), alnum_uppers(1))),
        opt(nom::bytes::complete::take(2_usize).and_then(alnum_uppers(2))),
        opt(nom::bytes::complete::take(2_usize).and_then(tuple((alnum_uppers(1), digits(1))))),
    )))
    .parse(input)
}

fn lot_code_old<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    digits(5).parse(input)
}
