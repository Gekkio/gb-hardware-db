// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::char,
    combinator::{cond, recognize, value},
    error::ParseError,
    sequence::tuple,
    Parser,
};

use super::{
    for_nom::{cgb_rom_code, digits, dmg_rom_code, uppers},
    GenericPart, MaskRom,
};
use crate::parser::{for_nom::year2_week2, Manufacturer, NomParser};

/// NEC μPD442012A-X (TSOP-I-48)
///
/// Source:
///   "NEC data sheet - MOS integrated circuit μPD442012A-X - 2M-bit CMOS static RAM 128k-word by 16-bit extended temperature operation"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD442012A_X.parse("NEC JAPAN D442012AGY-BB85X-MJH 0037K7027").is_ok());
/// assert!(parser::nec::NEC_UPD442012A_X.parse("NEC JAPAN D442012AGY-BC85X-MJH 0330K7043").is_ok());
/// ```
pub static NEC_UPD442012A_X: NomParser<GenericPart> = NomParser {
    name: "NEC μPD442012A-X",
    f: |input| {
        tuple((
            tag("NEC JAPAN "),
            recognize(tuple((
                tag("D442012A"),
                tag("GY"), // package
                char('-'),
                alt((tag("BB"), tag("BC"))), // voltage
                tag("85"),                   // speed
                char('X'),                   // temperature
                char('-'),
                tag("MJH"),
            ))),
            char(' '),
            tuple((year2_week2, uppers(1), digits(4))),
        ))
        .map(|(_, kind, _, (date_code, _, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Nec),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// NEC μPD442012L-X (TSOP-I-48)
///
/// Source:
///   "NEC data sheet - MOS integrated circuit μPD442012L-X - 2M-bit CMOS static RAM 128k-word by 16-bit extended temperature operation"
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD442012L_X.parse("NEC JAPAN D442012LGY-B85X-MJH 0138K7037").is_ok());
/// ```
pub static NEC_UPD442012L_X: NomParser<GenericPart> = NomParser {
    name: "NEC μPD442012L-X",
    f: |input| {
        tuple((
            tag("NEC JAPAN "),
            recognize(tuple((
                tag("D442012L"),
                tag("GY"), // package
                char('-'),
                alt((tag("B"), tag("C"))), // voltage
                tag("85"),                 // speed
                char('X'),                 // temperature
                char('-'),
                tag("MJH"),
            ))),
            char(' '),
            tuple((year2_week2, uppers(1), digits(4))),
        ))
        .map(|(_, kind, _, (date_code, _, _))| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Nec),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn upd23c<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    package: Package,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        uppers(1).and(digits(1)),
        char(' '),
        tuple((
            value("μPD23C", tag("N-")),
            tag(chip_type),
            tag(package.code()),
        ))
        .and(char('-').and(uppers(1)).and(digits(2))),
        char(' '),
        tuple((year2_week2, uppers(1), digits(4))),
    ))
    .map(
        |(rom_id, _, _, _, ((series, kind, package), _), _, (date_code, _, _))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Nec),
            chip_type: Some(format!("{series}{kind}{package}")),
            date_code: Some(date_code),
        },
    )
}

fn upd23c_old<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    package: Package,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        tag("NEC JAPAN "),
        alt((dmg_rom_code(), cgb_rom_code())),
        char(' '),
        uppers(1).and(digits(1)),
        char(' '),
        tuple((
            value("μPD23C", tag("UPD23C")),
            tag(chip_type),
            tag(package.code()),
        ))
        .and(char('-').and(uppers(1)).and(digits(2))),
        char(' '),
        tuple((year2_week2, uppers(1), digits(4))),
    ))
    .map(
        |(_, rom_id, _, _, _, ((series, kind, package), _), _, (date_code, _, _))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(Manufacturer::Nec),
            chip_type: Some(format!("{series}{kind}{package}")),
            date_code: Some(date_code),
        },
    )
}

fn upd23c_licensed<'a, E: ParseError<&'a str>>(
    chip_type: &'static str,
    package: Package,
    (manufacturer_text, manufacturer): (&'static str, Manufacturer),
    long: bool,
) -> impl Parser<&'a str, MaskRom, E> {
    tuple((
        tag(manufacturer_text),
        char(' '),
        alt((dmg_rom_code(), cgb_rom_code())),
        cond(long, tuple((char(' '), uppers(1).and(digits(1))))),
        char(' '),
        tuple((
            value("μPD23C", tag("23C")),
            tag(chip_type),
            tag(package.code()),
        ))
        .and(char('-').and(uppers(1)).and(digits(2))),
        char(' '),
        tuple((year2_week2, uppers(1), digits(4))),
    ))
    .map(
        move |(_, _, rom_id, _, _, ((series, kind, package), _), _, (date_code, _, _))| MaskRom {
            rom_id: String::from(rom_id),
            manufacturer: Some(manufacturer),
            chip_type: Some(format!("{series}{kind}{package}")),
            date_code: Some(date_code),
        },
    )
}

/// NEC μPD23C1001E (SOP-32, 5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD23C1001E.parse("NEC JAPAN DMG-SAJ-0 C1 UPD23C1001EGW-J01 9010E9702").is_ok());
/// assert!(parser::nec::NEC_UPD23C1001E.parse("DMG-HQE-0 C1 N-1001EGW-J23 9110E9001").is_ok());
/// ```
pub static NEC_UPD23C1001E: NomParser<MaskRom> = NomParser {
    name: "NEC μPD23C1001E",
    f: |input| {
        let package = Package::Sop32;
        alt((
            upd23c_old("1001E", package),
            upd23c("1001E", package),
            upd23c("1001EA", package),
            upd23c("1001EU", package),
        ))
        .parse(input)
    },
};

/// NEC μPD23C2001E (SOP-32, 5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD23C2001E.parse("DMG-AVLP-0 D1 N-2001EUGW-J38 9840E7004").is_ok());
/// ```
pub static NEC_UPD23C2001E: NomParser<MaskRom> = NomParser {
    name: "NEC μPD23C2001E",
    f: |input| {
        let package = Package::Sop32;
        alt((upd23c("2001E", package), upd23c("2001EU", package))).parse(input)
    },
};

/// NEC μPD23C4001E (SOP-32, 5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD23C4001E.parse("DMG-AYWJ-1 E1 N-4001EJGW-J82 9804E7012").is_ok());
/// assert!(parser::nec::NEC_UPD23C4001E.parse("DMG-ZLE-0 E1 N-4001EAGW-J14 9325X9700").is_ok());
/// ```
pub static NEC_UPD23C4001E: NomParser<MaskRom> = NomParser {
    name: "NEC μPD23C4001E",
    f: |input| {
        let package = Package::Sop32;
        alt((
            upd23c("4001EA", package),
            upd23c("4001EJ", package),
            upd23c("4001EU", package),
        ))
        .parse(input)
    },
};

/// NEC μPD23C8001E (SOP-32, 5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD23C8001E.parse("DMG-AGQE-0 F1 N-8001EJGW-K14 0033K7036").is_ok());
/// ```
pub static NEC_UPD23C8001E: NomParser<MaskRom> = NomParser {
    name: "NEC μPD23C8001E",
    f: |input| upd23c("8001EJ", Package::Sop32).parse(input),
};

/// NEC μPD23C16019W (TSOP-II-44, 5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::NEC_UPD23C16019W.parse("DMG-VPHP-0 G2 N-16019WG5-M51 0029K7039").is_ok());
/// ```
pub static NEC_UPD23C16019W: NomParser<MaskRom> = NomParser {
    name: "NEC μPD23C16019W",
    f: |input| upd23c("16019W", Package::TsopIi44).parse(input),
};

/// AT&T μPD23C1001E (SOP-32, 5V)
///
/// Original by NEC, manufactured under license (?)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::AT_T_UPD23C1001E.parse("Ⓜ AT&T JAPAN DMG-Q6E-0 C1 23C1001EAGW-K37 9351E9005").is_ok());
/// ```
pub static AT_T_UPD23C1001E: NomParser<MaskRom> = NomParser {
    name: "AT&T μPD23C1001E",
    f: |input| {
        upd23c_licensed(
            "1001EA",
            Package::Sop32,
            ("Ⓜ AT&T JAPAN", Manufacturer::AtT),
            true,
        )
        .parse(input)
    },
};

/// Standard Microsystems μPD23C1001E (SOP-32, 5V)
///
/// Original by NEC, manufactured under license (?)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::SMSC_UPD23C1001E.parse("STANDARD MICRO DMG-BIA-0 C1 23C1001EGW-J61 9140E9017").is_ok());
/// ```
pub static SMSC_UPD23C1001E: NomParser<MaskRom> = NomParser {
    name: "SMSC μPD23C1001E",
    f: |input| {
        let package = Package::Sop32;
        let manufacturer = ("STANDARD MICRO", Manufacturer::Smsc);
        alt((
            upd23c_licensed("1001E", package, manufacturer, true),
            upd23c_licensed("1001EA", package, manufacturer, true),
        ))
        .parse(input)
    },
};

/// MANI μPD23C4001E (SOP-32, 5V)
///
/// Original by NEC, manufactured under license (?)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::nec::MANI_UPD23C4001E.parse("MANI DMG-MQE-2 23C4001EAGW-J22 9447X9200").is_ok());
/// ```
pub static MANI_UPD23C4001E: NomParser<MaskRom> = NomParser {
    name: "MANI μPD23C4001E",
    f: |input| {
        upd23c_licensed(
            "4001EA",
            Package::Sop32,
            ("MANI", Manufacturer::Mani),
            false,
        )
        .parse(input)
    },
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Package {
    Sop32,
    TsopIi44,
}

impl Package {
    pub const fn code(&self) -> &'static str {
        match self {
            Package::Sop32 => "GW",
            Package::TsopIi44 => "G5",
        }
    }
}