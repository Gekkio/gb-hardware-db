// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{IResult, Parser as _, bytes::streaming::tag, error::ParseError, sequence::terminated};

use super::{
    Manufacturer, Mapper, MapperChip, NomParser, PartDateCode,
    for_nom::{lines4, uppers, year2_week2},
};

/// Hudson HuC-1 mapper (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hudson::HUDSON_HUC1.parse("HuC-1 © HUDSON Nintendo 9752 A").is_ok());
/// ```
pub static HUDSON_HUC1: NomParser<Mapper> = NomParser {
    name: "Hudson HuC-1",
    f: |input| {
        lines4(tag("HuC-1"), tag("© HUDSON"), tag("Nintendo"), date_code)
            .map(|(_, _, _, date_code)| Mapper {
                kind: MapperChip::Huc1,
                manufacturer: Some(Manufacturer::Hudson),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Hudson HuC-1A mapper (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hudson::HUDSON_HUC1A.parse("HuC1A © HUDSON Nintendo 9845 A").is_ok());
/// ```
pub static HUDSON_HUC1A: NomParser<Mapper> = NomParser {
    name: "Hudson HuC-1A",
    f: |input| {
        lines4(tag("HuC1A"), tag("© HUDSON"), tag("Nintendo"), date_code)
            .map(|(_, _, _, date_code)| Mapper {
                kind: MapperChip::Huc1A,
                manufacturer: Some(Manufacturer::Hudson),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// Hudson HuC-3 mapper (QFP-48)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::hudson::HUDSON_HUC3.parse("HuC-3 © HUDSON Nintendo 9943 A").is_ok());
/// ```
pub static HUDSON_HUC3: NomParser<Mapper> = NomParser {
    name: "Hudson HuC-3",
    f: |input| {
        lines4(tag("HuC-3"), tag("© HUDSON"), tag("Nintendo"), date_code)
            .map(|(_, _, _, date_code)| Mapper {
                kind: MapperChip::Huc3,
                manufacturer: Some(Manufacturer::Hudson),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

fn date_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, PartDateCode, E> {
    terminated(year2_week2, tag(" ").and(uppers(1))).parse(input)
}
