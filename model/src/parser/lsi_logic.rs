// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser, bytes::streaming::tag, combinator::opt, error::ParseError, sequence::delimited};

use super::{
    GenericPart, Manufacturer, NomParser, PartDateCode,
    for_nom::{alnum_uppers, digits, lines4, uppers, week2, year1},
};

fn lh51_sop28<'a, E: ParseError<&'a str>>(
    kind: &'static str,
) -> impl Parser<&'a str, Output = GenericPart, Error = E> {
    lines4(
        tag(kind),
        tag("LSI LOGIC"),
        tag("JAPAN"),
        delimited(
            tag("D"),
            (year1, opt(tag(" ")), week2)
                .map(|(year, _, week)| PartDateCode::YearWeek { year, week }),
            (tag(" "), digits(1), tag(" "), uppers(2)),
        ),
    )
    .map(|(kind, _, _, date_code)| GenericPart {
        kind: String::from(kind),
        manufacturer: Some(Manufacturer::LsiLogic),
        date_code: Some(date_code),
    })
}

fn lh52_sop28<'a, E: ParseError<&'a str>>(
    kind: &'static str,
) -> impl Parser<&'a str, Output = GenericPart, Error = E> {
    lines4(
        tag(kind),
        tag("LSI LOGIC"),
        tag("JAPAN"),
        delimited(
            tag("D"),
            (year1, opt(tag(" ")), week2)
                .map(|(year, _, week)| PartDateCode::YearWeek { year, week }),
            (tag(" "), digits(1), alnum_uppers(1), tag(" "), uppers(1)),
        ),
    )
    .map(|(kind, _, _, date_code)| GenericPart {
        kind: String::from(kind),
        manufacturer: Some(Manufacturer::LsiLogic),
        date_code: Some(date_code),
    })
}

/// LSI Logic LH5264N4T SRAM (SOP-28, 5V)
///
/// Probably Sharp LH5264N4 manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lsi_logic::LSI_LOGIC_LH5264N4T.parse("LH5264N4T LSI LOGIC JAPAN D222 24 C").is_ok());
/// ```
pub static LSI_LOGIC_LH5264N4T: NomParser<GenericPart> = NomParser {
    name: "LSI Logic LH5264N4T",
    f: |input| lh52_sop28("LH5264N4T").parse(input),
};

/// LSI Logic LH5264TN SRAM (SOP-28, 5V)
///
/// Probably Sharp LH5264N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lsi_logic::LSI_LOGIC_LH5264TN.parse("LH5264TN-TL LSI LOGIC JAPAN D220 53 C").is_ok());
/// ```
pub static LSI_LOGIC_LH5264TN: NomParser<GenericPart> = NomParser {
    name: "LSI Logic LH5264TN",
    f: |input| lh52_sop28("LH5264TN-TL").parse(input),
};

/// LSI Logic LH52A64N SRAM (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// Probably Sharp LH52A64N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lsi_logic::LSI_LOGIC_LH52A64N.parse("LH52A64N-TL LSI LOGIC JAPAN D404 0U C").is_ok());
/// assert!(parser::lsi_logic::LSI_LOGIC_LH52A64N.parse("LH52A64N-TL LSI LOGIC JAPAN D4 06 05 C").is_ok());
/// ```
pub static LSI_LOGIC_LH52A64N: NomParser<GenericPart> = NomParser {
    name: "LSI Logic LH52A64N",
    f: |input| lh52_sop28("LH52A64N-TL").parse(input),
};

/// LSI Logic LH52B256N SRAM (SOP-28, 5V, 256 Kibit / 32 KiB)
///
/// Probably Sharp LH52B256N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lsi_logic::LSI_LOGIC_LH52B256N.parse("LH52B256NA-10TLL LSI LOGIC JAPAN D344 03 B").is_ok());
/// ```
pub static LSI_LOGIC_LH52B256N: NomParser<GenericPart> = NomParser {
    name: "LSI Logic LH52B256N",
    f: |input| lh52_sop28("LH52B256NA-10TLL").parse(input),
};

/// LSI Logic LH5168N SRAM (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// Probably Sharp LH5168N manufactured under license.
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::lsi_logic::LSI_LOGIC_LH5168N.parse("LH5168NFB-10TL LSI LOGIC JAPAN D242 7 BC").is_ok());
/// ```
pub static LSI_LOGIC_LH5168N: NomParser<GenericPart> = NomParser {
    name: "LSI Logic LH5168N",
    f: |input| lh51_sop28("LH5168NFB-10TL").parse(input),
};
