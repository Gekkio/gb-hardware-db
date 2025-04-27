// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    IResult, Parser as _,
    bytes::streaming::tag,
    error::ParseError,
    sequence::{preceded, terminated},
};

use super::{
    Manufacturer, Mapper, MapperChip, NomParser, PartDateCode,
    for_nom::{alnum_uppers, digits, lines3, lines4, uppers, year1, year1_week2},
};

/// Panasonic MBC1B (SOP-24)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::panasonic::PANASONIC_MBC1B.parse("DMG MBC1-B Nintendo P 0'D7").is_ok());
/// ```
pub static PANASONIC_MBC1B: NomParser<Mapper> = NomParser {
    name: "Panasonic MBC1B",
    f: |input| {
        lines4(
            tag("DMG"),
            tag("MBC1-B"),
            tag("Nintendo"),
            preceded(tag("P "), date_code_sop),
        )
        .map(|(_, _, _, date_code)| Mapper {
            kind: MapperChip::Mbc1B,
            manufacturer: Some(Manufacturer::Panasonic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Panasonic MBC2A (SOP-28)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::panasonic::PANASONIC_MBC2A.parse("DMG MBC2-A Nintendo P 8'73").is_ok());
/// ```
pub static PANASONIC_MBC2A: NomParser<Mapper> = NomParser {
    name: "Panasonic MBC2A",
    f: |input| {
        lines4(
            tag("DMG"),
            tag("MBC2-A"),
            tag("Nintendo"),
            preceded(tag("P "), date_code_sop),
        )
        .map(|(_, _, _, date_code)| Mapper {
            kind: MapperChip::Mbc2A,
            manufacturer: Some(Manufacturer::Panasonic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Panasonic MBC3A (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::panasonic::PANASONIC_MBC3A.parse("MBC3 A P-2 834U4E").is_ok());
/// ```
pub static PANASONIC_MBC3A: NomParser<Mapper> = NomParser {
    name: "Panasonic MBC3A",
    f: |input| {
        lines3(tag("MBC3 A"), tag("P-2"), date_code_qfp)
            .map(|(_, _, date_code)| Mapper {
                kind: MapperChip::Mbc3A,
                manufacturer: Some(Manufacturer::Panasonic),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Panasonic MBC3B (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::panasonic::PANASONIC_MBC3B.parse("MBC3 B P-2 134U2D").is_ok());
/// ```
pub static PANASONIC_MBC3B: NomParser<Mapper> = NomParser {
    name: "Panasonic MBC3B",
    f: |input| {
        lines3(tag("MBC3 B"), tag("P-2"), date_code_qfp)
            .map(|(_, _, date_code)| Mapper {
                kind: MapperChip::Mbc3B,
                manufacturer: Some(Manufacturer::Panasonic),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Panasonic MBC30 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::panasonic::PANASONIC_MBC30.parse("MBC30 P 047U2M").is_ok());
/// ```
pub static PANASONIC_MBC30: NomParser<Mapper> = NomParser {
    name: "Panasonic MBC30",
    f: |input| {
        lines3(tag("MBC30"), tag("P"), date_code_qfp)
            .map(|(_, _, date_code)| Mapper {
                kind: MapperChip::Mbc30,
                manufacturer: Some(Manufacturer::Panasonic),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Panasonic MBC5 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::panasonic::PANASONIC_MBC5.parse("MBC5 P 041U7M").is_ok());
/// assert!(parser::panasonic::PANASONIC_MBC5.parse("MBC5 P-1 850U3L").is_ok());
/// assert!(parser::panasonic::PANASONIC_MBC5.parse("MBC5 P-2 104U4M").is_ok());
/// ```
pub static PANASONIC_MBC5: NomParser<Mapper> = NomParser {
    name: "Panasonic MBC5",
    f: |input| {
        lines3(
            tag("MBC5"),
            tag("P-2").or(tag("P-1")).or(tag("P")),
            date_code_qfp,
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc5,
            manufacturer: Some(Manufacturer::Panasonic),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

fn date_code_sop<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    terminated(
        year1.map(|year| PartDateCode::Year { year }),
        (tag("'"), alnum_uppers(1), digits(1)),
    )
    .parse(input)
}

fn date_code_qfp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    terminated(year1_week2, (tag("U"), digits(1), uppers(1))).parse(input)
}
