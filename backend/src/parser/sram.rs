// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{LabelParser, Manufacturer};
use crate::{
    macros::{multi_parser, single_parser},
    parser::{
        amic, bsi, fujitsu, hynix, hyundai, lgs, nec, sanyo, st_micro, toshiba, victronix, week2,
        winbond, year1, year2, GenericPart, PartDateCode,
    },
};

pub mod sop_28;
pub mod tsop_i_28;

pub type Ram = GenericPart;

/// LSI Logic LH52xx 64 kbit
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::lsi_logic_lh52xx().parse("LH5264N4T LSI LOGIC JAPAN D222 24 C").is_ok());
/// assert!(parser::sram::lsi_logic_lh52xx().parse("LH5264N4T LSI LOGIC JAPAN D4 06 05 C").is_ok());
/// assert!(parser::sram::lsi_logic_lh52xx().parse("LH52A64N-TL LSI LOGIC JAPAN D4 06 05 C").is_ok());
/// ```
pub fn lsi_logic_lh52xx() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5264N4T|LH52A64N-TL|LH5264TN-TL)\ LSI\ LOGIC\ JAPAN\ [A-Z]([0-9])\ ?([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::LsiLogic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// LSI Logic LH52B256 (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::lsi_logic_lh52b256().parse("LH52B256NA-10TLL LSI LOGIC JAPAN D344 03 B").is_ok());
/// ```
pub fn lsi_logic_lh52b256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52B256[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ LSI\ LOGIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::LsiLogic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// LSI Logic LH5168 (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::lsi_logic_lh5168().parse("LH5168NFB-10TL LSI LOGIC JAPAN D242 7 BC").is_ok());
/// ```
pub fn lsi_logic_lh5168() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5168[A-Z]{0,3}-[0-9]{2}[A-Z]{2,3})\ LSI\ LOGIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [0-9]\ [[:alnum:]]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::LsiLogic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Mosel-Vitelic LH52B256 (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::mosel_vitelic_lh52b256().parse("LH52B256NA-10PLL MOSEL-VITELIC JAPAN N643 0T BB").is_ok());
/// ```
pub fn mosel_vitelic_lh52b256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52B256[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Mosel-Vitelic LH5168 (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::mosel_vitelic_lh5168().parse("LH5168N-10PL MOSEL-VITELIC JAPAN N745 1G BH").is_ok());
/// assert!(parser::sram::mosel_vitelic_lh5168().parse("LH5168N-10PL MOSEL-VITELIC JAPAN N7 34 22 BH").is_ok());
/// ```
pub fn mosel_vitelic_lh5168() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5168[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])\ ?([0-9]{2})\ [[:alnum:]]{2}\ [[:alnum:]]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Mosel-Vitelic LH5268A (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::mosel_vitelic_lh5268a().parse("LH5268ANF-10PLL MOSEL-VITELIC JAPAN N633 0A BC").is_ok());
/// ```
pub fn mosel_vitelic_lh5268a() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5268A[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [[:alnum:]]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5160 (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5160().parse("LH5160N-10L SHARP JAPAN 9007 5 DA").is_ok());
/// ```
pub fn sharp_lh5160() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5160[A-Z]{0,3}-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})(\ [0-9])?\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5168 (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5168().parse("LH5168N-10L SHARP JAPAN 9803 1 DG").is_ok());
/// assert!(parser::sram::sharp_lh5168().parse("LH5168NFA-10L SHARP JAPAN 9103 3 SA").is_ok());
/// assert!(parser::sram::sharp_lh5168().parse("LH5168NFB-10L SHARP JAPAN 9147 DC").is_ok());
/// ```
pub fn sharp_lh5168() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5168[A-Z]{0,3}-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})(\ [0-9])?\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5164AN
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5164an().parse("LH5164AN-10L SHARP JAPAN 9933 3 EB").is_ok());
/// ```
pub fn sharp_lh5164an() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164AN-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ A?([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5164AN
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5164an_2().parse("LH5164AN-10L SHARP A0005 3 CB").is_ok())
/// ```
pub fn sharp_lh5164an_2() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164AN-[0-9]{2}[A-Z]?)\ SHARP\ A([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5164LN
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5164ln().parse("LH5164LN-10 SHARP JAPAN 8848 3 D").is_ok())
/// ```
pub fn sharp_lh5164ln() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164LN-[0-9]{2})\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5264N
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5264n().parse("LH5264N4 SHARP JAPAN 9204 5 Y").is_ok());
/// assert!(parser::sram::sharp_lh5264n().parse("LH5264N SHARP JAPAN 9022 7 Y").is_ok());
/// ```
pub fn sharp_lh5264n() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5264N4?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5264TN-L
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5264tn_l().parse("LH5264TN-L SHARP JAPAN 9038 5 Y").is_ok());
/// ```
pub fn sharp_lh5264tn_l() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5264TN-L)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH5164N
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh5164n().parse("LH5164N-10L SHARP JAPAN 9043 1 DA").is_ok());
/// ```
pub fn sharp_lh5164n() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164N-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Sharp LH52A64N-L
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sharp_lh52a64n_l().parse("LH52A64N-L SHARP JAPAN 9817 1 Y").is_ok());
/// ```
pub fn sharp_lh52a64n_l() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52A64N-L)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Crosslink LH52A64N-YL
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::crosslink_lh52a64n_yl().parse("LH52A64N-YL Xlink JAPAN H432 0U C").is_ok());
/// ```
pub fn crosslink_lh52a64n_yl() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52A64N-YL)\ Xlink\ JAPAN\ H([0-9]{1})\ ?([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Crosslink),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Crosslink LH5268ANF-10YLL (4.5-5.5V)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::crosslink_lh5268anf().parse("LH5268ANF-10YLL Xlink JAPAN H429 0Y BB").is_ok());
/// ```
pub fn crosslink_lh5268anf() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5268ANF-10YLL)\ Xlink\ JAPAN\ H([0-9]{1})\ ?([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Crosslink),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

/// Mosel-Vitelic LH52A64N-PL
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::mosel_vitelic_lh52a64n_pl().parse("LH52A64N-PL MOSEL-VITELIC JAPAN N651 0F C").is_ok());
/// ```
pub fn mosel_vitelic_lh52a64n_pl() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52A64N-PL)\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[2])?,
                    week: week2(&c[3])?,
                }),
            })
        },
    )
}

pub fn sram_sop_28_5v() -> &'static impl LabelParser<Ram> {
    multi_parser!(
        Ram,
        &bsi::BSI_BS62LV256,
        &lgs::LGS_GM76C256,
        &lgs::HYUNDAI_GM76C256,
        &hyundai::HYUNDAI_HY6264,
        &sanyo::SANYO_LC35256,
        &sanyo::SANYO_LC3564,
        sop_28::rohm_br62256f(),
        sop_28::rohm_br6265(),
        sop_28::rohm_xlj6265(),
        &victronix::VICTRONIX_VN4464,
        &winbond::WINBOND_W24257,
        &winbond::WINBOND_W24258,
        &winbond::WINBOND_W2465,
        sop_28::sharp_lh52256(),
        lsi_logic_lh5168(),
        lsi_logic_lh52b256(),
        mosel_vitelic_lh5168(),
        mosel_vitelic_lh5268a(),
        mosel_vitelic_lh52b256(),
        sharp_lh5164an(),
        sharp_lh5164an_2(),
        sharp_lh5160(),
        sharp_lh5168(),
        sharp_lh5164ln(),
        sharp_lh5164n(),
        sharp_lh5264n(),
        sharp_lh5264tn_l(),
        sharp_lh52a64n_l(),
        lsi_logic_lh52xx(),
        crosslink_lh52a64n_yl(),
        crosslink_lh5268anf(),
        mosel_vitelic_lh52a64n_pl(),
        &hynix::HYNIX_HY62WT08081,
    )
}

pub fn sram_sop_28_3v3() -> &'static impl LabelParser<Ram> {
    multi_parser!(
        Ram,
        &bsi::BSI_BS62LV256,
        &lgs::HYUNDAI_GM76V256,
        &sanyo::SANYO_LC35256,
        &sanyo::SANYO_LC3564,
        sop_28::sharp_lh52256(),
        &hynix::HYNIX_HY62WT08081,
    )
}

pub fn sram_sop_32_5v() -> &'static impl LabelParser<Ram> {
    multi_parser!(GenericPart, &hyundai::HYUNDAI_HY628100,)
}

pub fn sram_tsop_i_28() -> &'static impl LabelParser<Ram> {
    multi_parser!(
        GenericPart,
        tsop_i_28::sharp_lh52256(),
        tsop_i_28::sharp_lh51d256(),
        tsop_i_28::sharp_lh52cv256(),
    )
}

pub fn sram_tsop_i_48() -> &'static impl LabelParser<Ram> {
    multi_parser!(
        Ram,
        &nec::NEC_UPD442012A_X,
        &nec::NEC_UPD442012L_X,
        &fujitsu::FUJITSU_MB82D12160,
        &hynix::HYNIX_HY62LF16206,
        &st_micro::ST_MICRO_M68AS128,
        &amic::AMIC_LP62S16128,
        &bsi::BSI_BS616LV2018,
        &bsi::BSI_BS616LV2019,
        &toshiba::TOSHIBA_TC55V200
    )
}
