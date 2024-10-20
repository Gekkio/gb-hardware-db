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
        tsop_i_32::macronix_mx29l010(),
        tsop_i_32::sanyo_le26fv10(),
        tsop_i_32::atmel_at29lv512(),
        tsop_i_32::sst_sst39vf512(),
    )
}

pub fn flash_tsop_i_40() -> &'static impl LabelParser<Flash> {
    multi_parser!(Flash, tsop_i_40::macronix_mx29f008(),)
}
