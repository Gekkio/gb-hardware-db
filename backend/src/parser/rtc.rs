// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{GenericChip, LabelParser};
use crate::{macros::multi_parser, parser::MultiNomFnParser};

pub mod sop_20;
pub mod sop_8;

pub type Rtc = GenericChip;

pub fn rtc_sop_8() -> &'static impl LabelParser<Rtc> {
    static PARSER: MultiNomFnParser<Rtc> =
        MultiNomFnParser::new(&[&sop_8::SEIKO_S3511A, &sop_8::SEIKO_S3516AE]);
    &PARSER
}

pub fn rtc_sop_20() -> &'static impl LabelParser<Rtc> {
    multi_parser!(Rtc, sop_20::toshiba_tc8521am(),)
}
