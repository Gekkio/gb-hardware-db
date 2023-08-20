// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coil {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::coil::tdk().parse("TDK ZJY-M4A N").is_ok());
/// ```
pub fn tdk() -> &'static impl LabelParser<Coil> {
    single_parser!(Coil, r#"^TDK\ (ZJY-M4A)\ [A-Z]$"#, move |c| {
        Ok(Coil {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Tdk),
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::coil::tdk2().parse("TDK ZJY-M4PA n").is_ok());
/// ```
pub fn tdk2() -> &'static impl LabelParser<Coil> {
    single_parser!(Coil, r#"^TDK\ (ZJY-M4PA)\ [a-z]$"#, move |c| {
        Ok(Coil {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Tdk),
        })
    })
}

pub fn coil() -> &'static impl LabelParser<Coil> {
    multi_parser!(Coil, tdk(), tdk2())
}
