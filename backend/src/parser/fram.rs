// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::{
    macros::multi_parser,
    parser::{GenericChip, LabelParser},
};

pub mod sop_28 {
    use nom::{
        bytes::streaming::tag,
        character::{complete::one_of, streaming::char},
        combinator::opt,
        sequence::tuple,
        Parser as _,
    };

    use super::Fram;
    use crate::parser::{
        for_nom::{digits, uppers, year2_week2},
        Manufacturer, NomParser,
    };

    /// Fujitsu MB85R256 (SOP-28)
    ///
    /// ```
    /// use gbhwdb_backend::parser::{self, LabelParser};
    /// assert!(parser::fram::sop_28::FUJITSU_MB85R256.parse("JAPAN MB85R256A 0412 M88").is_ok());
    /// assert!(parser::fram::sop_28::FUJITSU_MB85R256.parse("JAPAN MB85R256S 0511 M22 E1").is_ok());
    /// ```
    pub static FUJITSU_MB85R256: NomParser<Fram> = NomParser {
        name: "Fujitsu MB85R256",
        f: |input| {
            tuple((
                tag("JAPAN "),
                tag("MB85R256"),
                one_of("AS"),
                char(' '),
                year2_week2,
                char(' '),
                uppers(1),
                digits(2),
                opt(nom::bytes::complete::tag(" E1")),
            ))
            .map(|(_, kind, rev, _, date_code, _, _, _, _)| Fram {
                kind: format!("{kind}{rev}"),
                manufacturer: Some(Manufacturer::Fujitsu),
                date_code: Some(date_code),
            })
            .parse(input)
        },
    };
}

pub type Fram = GenericChip;

pub fn fram_sop_28() -> &'static impl LabelParser<Fram> {
    multi_parser!(Fram, &sop_28::FUJITSU_MB85R256,)
}
