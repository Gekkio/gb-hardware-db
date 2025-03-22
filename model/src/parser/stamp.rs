// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser as _, character::complete::one_of, combinator::opt, sequence::terminated};

use super::{
    NomParser, Year,
    for_nom::{month2, satisfy_m_n_complete, week2, year1},
};
use crate::time::{Month, Week};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgStamp {
    pub year: Option<Year>,
    pub month: Option<Month>,
}

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::stamp::DMG_STAMP.parse("010 23").is_ok());
/// assert!(parser::stamp::DMG_STAMP.parse("903-22").is_ok());
/// assert!(parser::stamp::DMG_STAMP.parse("709.3901").is_ok());
/// assert!(parser::stamp::DMG_STAMP.parse("202-0007").is_ok());
/// assert!(parser::stamp::DMG_STAMP.parse("008.270-").is_ok());
/// ```
pub static DMG_STAMP: NomParser<DmgStamp> = NomParser {
    name: "DMG stamp",
    f: |input| {
        terminated(
            year1.and(month2),
            opt(one_of("- ."))
                .and(satisfy_m_n_complete(2, 4, |c| {
                    c.is_ascii_digit() || c == '-'
                }))
                .and(opt(nom::character::complete::char('Y'))),
        )
        .map(|(year, month)| DmgStamp {
            year: Some(year),
            month: Some(month),
        })
        .parse(input)
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbStamp {
    pub year: Option<Year>,
    pub week: Option<Week>,
}

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::stamp::CGB_STAMP.parse("218-2221").is_ok());
/// ```
pub static CGB_STAMP: NomParser<CgbStamp> = NomParser {
    name: "CGB stamp",
    f: |input| {
        terminated(
            week2.and(year1),
            opt(one_of("- .X"))
                .and(satisfy_m_n_complete(2, 4, |c| c.is_ascii_digit()))
                .and(opt(nom::character::complete::char('Y'))),
        )
        .map(|(week, year)| CgbStamp {
            year: Some(year),
            week: Some(week),
        })
        .parse(input)
    },
};
