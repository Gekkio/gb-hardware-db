// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{bytes::streaming::tag, character::streaming::char, sequence::tuple, Parser as _};

use super::Flash;
use crate::parser::{macronix, Manufacturer, NomParser};

/// Macronix MX29F008 (TSOP-I-40)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::flash::tsop_i_40::MACRONIX_MX29F008.parse("E991012 29F008TC-14 21534 TAIWAN").is_ok());
/// ```
pub static MACRONIX_MX29F008: NomParser<Flash> = NomParser {
    name: "Macronix MX29F008",
    f: |input| {
        tuple((
            macronix::assembly_vendor_code,
            macronix::date_code,
            tag("12"), // digits 3 and 4 of "product body" (?)
            tag(" 29F008TC-14"),
            char(' '),
            macronix::lot_code_old,
            tag(" TAIWAN"),
        ))
        .map(|(_, date_code, _, _, _, _, _)| Flash {
            kind: String::from("MX29F008TC-14"),
            manufacturer: Some(Manufacturer::Macronix),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
