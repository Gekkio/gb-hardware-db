// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{LabelParser, Manufacturer};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transformer {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// # use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mgl_transformer::mitsumi_unknown().parse("82Y7").is_ok());
/// assert!(parser::mgl_transformer::mitsumi_unknown().parse("84Z7").is_ok());
/// ```
pub fn mitsumi_unknown() -> &'static impl LabelParser<Transformer> {
    single_parser!(Transformer, r#"^(82Y7|84Z7)$"#, move |c| {
        Ok(Transformer {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
        })
    })
}

pub fn mgl_transformer() -> &'static impl LabelParser<Transformer> {
    mitsumi_unknown()
}
