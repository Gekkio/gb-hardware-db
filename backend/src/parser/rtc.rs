// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{GenericChip, LabelParser};
use crate::macros::multi_parser;

pub mod sop_20;
pub mod sop_8;

pub type Rtc = GenericChip;

pub fn rtc_sop_8() -> &'static impl LabelParser<Rtc> {
    multi_parser!(Rtc, sop_8::seiko_s3511a(), sop_8::seiko_s3516ae(),)
}

pub fn rtc_sop_20() -> &'static impl LabelParser<Rtc> {
    multi_parser!(Rtc, sop_20::toshiba_tc8521am(),)
}
