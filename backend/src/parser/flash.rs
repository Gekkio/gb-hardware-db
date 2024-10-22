// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{GenericChip, LabelParser};
use crate::macros::multi_parser;

pub mod tsop_i_32;
pub mod tsop_i_40;

pub type Flash = GenericChip;

pub fn flash_tsop_i_32() -> &'static impl LabelParser<Flash> {
    multi_parser!(
        Flash,
        &tsop_i_32::MACRONIX_MX29L010,
        &tsop_i_32::SANYO_LE26FV10,
        &tsop_i_32::ATMEL_AT29LV512,
        &tsop_i_32::SST_SST39VF512,
    )
}

pub fn flash_tsop_i_40() -> &'static impl LabelParser<Flash> {
    multi_parser!(Flash, &tsop_i_40::MACRONIX_MX29F008)
}
