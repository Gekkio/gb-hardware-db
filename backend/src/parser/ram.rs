// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, year2, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

pub type Ram = ChipYearWeek;

/// LSI Logic LH52xx 64 kbit
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::lsi_logic_lh52xx().parse("LH5264N4T LSI LOGIC JAPAN D222 24 C").is_ok());
/// assert!(parser::ram::lsi_logic_lh52xx().parse("LH5264N4T LSI LOGIC JAPAN D4 06 05 C").is_ok());
/// assert!(parser::ram::lsi_logic_lh52xx().parse("LH52A64N-TL LSI LOGIC JAPAN D4 06 05 C").is_ok());
/// ```
pub fn lsi_logic_lh52xx() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5264N4T|LH52A64N-TL|LH5264TN-TL)\ LSI\ LOGIC\ JAPAN\ [A-Z]([0-9])\ ?([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::LsiLogic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// LSI Logic LH52B256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::lsi_logic_lh52b256().parse("LH52B256NA-10TLL LSI LOGIC JAPAN D344 03 B").is_ok());
/// ```
pub fn lsi_logic_lh52b256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52B256[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ LSI\ LOGIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::LsiLogic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// LSI Logic LH5168
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::lsi_logic_lh5168().parse("LH5168NFB-10TL LSI LOGIC JAPAN D242 7 BC").is_ok());
/// ```
pub fn lsi_logic_lh5168() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5168[A-Z]{0,3}-[0-9]{2}[A-Z]{2,3})\ LSI\ LOGIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [0-9]\ [[:alnum:]]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::LsiLogic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Mosel-Vitelic LH52B256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::mosel_vitelic_lh52b256().parse("LH52B256NA-10PLL MOSEL-VITELIC JAPAN N643 0T BB").is_ok());
/// ```
pub fn mosel_vitelic_lh52b256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52B256[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Mosel-Vitelic LH5168
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::mosel_vitelic_lh5168().parse("LH5168N-10PL MOSEL-VITELIC JAPAN N745 1G BH").is_ok());
/// assert!(parser::ram::mosel_vitelic_lh5168().parse("LH5168N-10PL MOSEL-VITELIC JAPAN N7 34 22 BH").is_ok());
/// ```
pub fn mosel_vitelic_lh5168() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5168[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])\ ?([0-9]{2})\ [[:alnum:]]{2}\ [[:alnum:]]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Mosel-Vitelic LH5268A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::mosel_vitelic_lh5268a().parse("LH5268ANF-10PLL MOSEL-VITELIC JAPAN N633 0A BC").is_ok());
/// ```
pub fn mosel_vitelic_lh5268a() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5268A[A-Z]{0,2}-[0-9]{2}[A-Z]{2,3})\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [[:alnum:]]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sanyo LC35256D
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sanyo_lc35256d().parse("SANYO LC35256DM-70W JAPAN 0EUPG").is_ok());
/// ```
pub fn sanyo_lc35256d() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^SANYO\ (LC35256D[MT]-[0-9]{2}W)\ JAPAN\ ([0-9])[[:alnum:]]{4}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sanyo),
                year: Some(year1(&c[2])?),
                week: None,
            })
        },
    )
}

/// Sanyo LC35256F
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sanyo_lc35256f().parse("SANYO LC35256FM-70U JAPAN 0LK5G").is_ok());
/// ```
pub fn sanyo_lc35256f() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^SANYO\ (LC35256F[MT]-[0-9]{2}U)\ JAPAN\ ([0-9])[[:alnum:]]{4}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sanyo),
                year: Some(year1(&c[2])?),
                week: None,
            })
        },
    )
}

/// Sanyo LC3564B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sanyo_lc3564b().parse("SANYO LC3564BM-70 JAPAN 9MUBG").is_ok());
/// ```
pub fn sanyo_lc3564b() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^SANYO\ (LC3564B[A-Z]?-[0-9]{2})\ JAPAN\ ([0-9])[[:alnum:]]{4}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sanyo),
                year: Some(year1(&c[2])?),
                week: None,
            })
        },
    )
}

/// Sharp LH52256C
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh52256c().parse("LH52256CT-10LL SHARP JAPAN 9824 3 SF").is_ok());
/// ```
pub fn sharp_lh52256c() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52256C[A-Z]{1,2}-[0-9]{2}[A-Z]{0,2})\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH52256CVT
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh52256cvt().parse("LH52256CVT SHARP JAPAN 9841 3 LO").is_ok());
/// ```
pub fn sharp_lh52256cvt() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52256CVT)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH52CV256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh52cv256().parse("LH52CV256JT-10LL SHARP JAPAN 9814 7 SA").is_ok());
/// ```
pub fn sharp_lh52cv256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52CV256[A-Z]{1,2}-[0-9]{2}[A-Z]{0,2})\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH51D256T
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh51d256t().parse("LH51D256T-Z7 SHARP Y013 5 J").is_ok());
/// assert!(parser::ram::sharp_lh51d256t().parse("LH51D256T-Z7 SHARP JAPAN Y0 47 3 JA").is_ok());
/// ```
pub fn sharp_lh51d256t() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH51D256T-Z[0-9])\ SHARP(\ JAPAN)?\ A?Y([0-9])\ ?([0-9]{2})\ [0-9]\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Sharp LH5160
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5160().parse("LH5160N-10L SHARP JAPAN 9007 5 DA").is_ok());
/// ```
pub fn sharp_lh5160() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5160[A-Z]{0,3}-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})(\ [0-9])?\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5168
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5168().parse("LH5168N-10L SHARP JAPAN 9803 1 DG").is_ok());
/// assert!(parser::ram::sharp_lh5168().parse("LH5168NFA-10L SHARP JAPAN 9103 3 SA").is_ok());
/// assert!(parser::ram::sharp_lh5168().parse("LH5168NFB-10L SHARP JAPAN 9147 DC").is_ok());
/// ```
pub fn sharp_lh5168() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5168[A-Z]{0,3}-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})(\ [0-9])?\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5164AN
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5164an().parse("LH5164AN-10L SHARP JAPAN 9933 3 EB").is_ok());
/// ```
pub fn sharp_lh5164an() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164AN-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ A?([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5164AN
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5164an_2().parse("LH5164AN-10L SHARP A0005 3 CB").is_ok())
/// ```
pub fn sharp_lh5164an_2() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164AN-[0-9]{2}[A-Z]?)\ SHARP\ A([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5164LN
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5164ln().parse("LH5164LN-10 SHARP JAPAN 8848 3 D").is_ok())
/// ```
pub fn sharp_lh5164ln() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164LN-[0-9]{2})\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5264N
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5264n().parse("LH5264N4 SHARP JAPAN 9204 5 Y").is_ok());
/// assert!(parser::ram::sharp_lh5264n().parse("LH5264N SHARP JAPAN 9022 7 Y").is_ok());
/// ```
pub fn sharp_lh5264n() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5264N4?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5264TN-L
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5264tn_l().parse("LH5264TN-L SHARP JAPAN 9038 5 Y").is_ok());
/// ```
pub fn sharp_lh5264tn_l() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5264TN-L)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH5164N
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh5164n().parse("LH5164N-10L SHARP JAPAN 9043 1 DA").is_ok());
/// ```
pub fn sharp_lh5164n() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5164N-[0-9]{2}[A-Z]?)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Sharp LH52A64N-L
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::sharp_lh52a64n_l().parse("LH52A64N-L SHARP JAPAN 9817 1 Y").is_ok());
/// ```
pub fn sharp_lh52a64n_l() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52A64N-L)\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// BSI BS62LV256SC
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::bsi_bs62lv256sc().parse("BSI BS62LV256SC-70 S2827V52155 A0106 TAIWAN").is_ok());
/// assert!(parser::ram::bsi_bs62lv256sc().parse("BSI BS62LV256SC-70 S2828W11075.1 F0231 TAIWAN").is_ok());
/// assert!(parser::ram::bsi_bs62lv256sc().parse("BSI BS62LV256SCG70 S2828CA30125 A D05502 TAIWAN").is_ok());
/// assert!(parser::ram::bsi_bs62lv256sc().parse("BSI BS62LV256SC-70 S2828W13088.1N F0318 TAIWAN").is_ok());
/// ```
pub fn bsi_bs62lv256sc() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^BSI\ (BS62LV256SC[G\-][0-9]{2})\ [[:alnum:]]{10,12}(.[0-9][A-Z]?)?(\ A)?\ [A-Z]([0-9]{2})([0-9]{2})[0-9]?\ TAIWAN$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Bsi),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Winbond W2465
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::winbond_w2465().parse("Winbond W2465S-70LL 140SD21331480-II1RA").is_ok());
/// ```
pub fn winbond_w2465() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Winbond\ (W2465[A-Z]?-[0-9]{2}[A-Z]{1,2})\ ([0-9])([0-9]{2})[A-Z]{2}[0-9]{8}-II1RA$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Winbond),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Winbond W24257
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::winbond_w24257().parse("Winbond W24257S-70LL 046QB202858301AC").is_ok());
/// ```
pub fn winbond_w24257() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Winbond\ (W24257[A-Z]?(-[0-9]{2}[A-Z]{1,2})?)\ ([0-9])([0-9]{2})[A-Z]{2}[0-9]{9}[A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Winbond),
                year: Some(year1(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Winbond W24258
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::winbond_w24258().parse("Winbond W24258S-70LE 011MH200254401AA").is_ok());
/// ```
pub fn winbond_w24258() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Winbond\ (W24258[A-Z]?(-[0-9]{2}[A-Z]{1,2})?)\ ([0-9])([0-9]{2})[A-Z]{2}[0-9]{9}[A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Winbond),
                year: Some(year1(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Rohm XLJ6265
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::rohm_xlj6265().parse("XLJ6265BF-10SL 640 173N").is_ok());
/// ```
pub fn rohm_xlj6265() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(XLJ6265[AB]?F?-N?[0-9]{2}[A-Z]{2})\ ([0-9])([0-9]{2})\ [0-9]{3}[A-Z]{0,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Rohm BR6265
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::rohm_br6265().parse("BR6265BF-10SL 111 120N").is_ok());
/// ```
pub fn rohm_br6265() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(BR6265[AB]?F?-N?[0-9]{2}[A-Z]{2})\ ([0-9])([0-9]{2})\ [0-9]{3}[A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Rohm BR62256F
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::rohm_br62256f().parse("BR62256F-70LL 006 169NA").is_ok());
/// ```
pub fn rohm_br62256f() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(BR62256F-[0-9]{2}[A-Z]{2})\ ([0-9])([0-9]{2})\ [0-9]{3}[A-Z]{0,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// LGS GM76C256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::lgs_gm76c256().parse("LGS GM76C256CLLFW70 9849 KOREA").is_ok());
/// ```
pub fn lgs_gm76c256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^LGS\ (GM76C256[ABC][A-Z]{1,4}[0-9]{2}E?)\ ([0-9]{2})([0-9]{2})\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Lgs),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Hyundai GM76C256C
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::hyundai_gm76c256c().parse("HYUNDAI GM76C256CLLFW70 0047 KOREA").is_ok());
/// ```
pub fn hyundai_gm76c256c() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HYUNDAI\ (GM76C256C[A-Z]{1,4}[0-9]{2}E?)\ ([0-9]{2})([0-9]{2})\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Hyundai),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Hyundai HY628100B
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::hyundai_hy628100b().parse("HYUNDAI KOREA HY628100B 0041A LLG-70").is_ok());
/// ```
pub fn hyundai_hy628100b() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HYUNDAI\ KOREA\ HY628100B\ ([0-9]{2})([0-9]{2})[A-Z]\ ([[:alnum:]]{2,4}-[0-9]{2}[EI]?)$"#,
        move |c| {
            Ok(Ram {
                kind: format!("HY628100B{}", &c[3]),
                manufacturer: Some(Manufacturer::Hyundai),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Hyundai HY6264A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::hyundai_hy6264a().parse("HY6264A LLJ-10 9902B KOREA").is_ok());
/// ```
pub fn hyundai_hy6264a() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HY6264A\ ([A-Z]{2,3}-[0-9]{2})\ ([0-9]{2})([0-9]{2})[A-Z]\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: format!("HY6264A{}", &c[1]),
                manufacturer: Some(Manufacturer::Hyundai),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Hyundai HY6264A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::hyundai_hy6264a_2().parse("HYUNDAI HY6264ALLJ-10 9327B KOREA").is_ok());
/// ```
pub fn hyundai_hy6264a_2() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HYUNDAI\ (HY6264A[A-Z]{3}-[0-9]{2})\ ([0-9]{2})([0-9]{2})[A-Z]\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Hyundai),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Victronix VN4464S
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::victronix_vn4464s().parse("Victronix VN4464S-08LL 95103B029").is_ok());
/// ```
pub fn victronix_vn4464s() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Victronix\ (VN4464S-08LL)\ ([0-9]{2})([0-9]{2})[0-9][A-Z][0-9]{3}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Victronix),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Crosslink LH52A64N-YL
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::crosslink_lh52a64n_yl().parse("LH52A64N-YL Xlink JAPAN H432 0U C").is_ok());
/// ```
pub fn crosslink_lh52a64n_yl() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52A64N-YL)\ Xlink\ JAPAN\ H([0-9]{1})\ ?([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Crosslink),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Crosslink LH5268ANF-10YLL
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::crosslink_lh5268anf().parse("LH5268ANF-10YLL Xlink JAPAN H429 0Y BB").is_ok());
/// ```
pub fn crosslink_lh5268anf() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH5268ANF-10YLL)\ Xlink\ JAPAN\ H([0-9]{1})\ ?([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Crosslink),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Mosel-Vitelic LH52A64N-PL
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::mosel_vitelic_lh52a64n_pl().parse("LH52A64N-PL MOSEL-VITELIC JAPAN N651 0F C").is_ok());
/// ```
pub fn mosel_vitelic_lh52a64n_pl() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52A64N-PL)\ MOSEL-VITELIC\ JAPAN\ [A-Z]([0-9])([0-9]{2})\ [[:alnum:]]{2}\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::MoselVitelic),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Hynix HY62WT08081E
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::hynix_hy62wt08081e().parse("hynix 0231A HY62WT081ED70C KOREA").is_ok());
/// ```
pub fn hynix_hy62wt08081e() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^hynix\ ([0-9]{2})([0-9]{2})[A-Z]\ (HY62WT081E[LD][0-9][0-9][CEI])\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: c[3].to_owned(),
                manufacturer: Some(Manufacturer::Hynix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Fujitsu MB85R256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ram::fujitsu_mb85r256().parse("JAPAN MB85R256A 0412 M88").is_ok());
/// assert!(parser::ram::fujitsu_mb85r256().parse("JAPAN MB85R256S 0511 M22 E1").is_ok());
/// ```
pub fn fujitsu_mb85r256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^JAPAN\ (MB85R256(A|S))\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}(\ [A-Z][0-9])?$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Fujitsu),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

pub fn ram() -> &'static impl LabelParser<Ram> {
    multi_parser!(
        Ram,
        bsi_bs62lv256sc(),
        hyundai_gm76c256c(),
        hyundai_hy6264a(),
        hyundai_hy6264a_2(),
        hyundai_hy628100b(),
        lgs_gm76c256(),
        lsi_logic_lh5168(),
        lsi_logic_lh52b256(),
        mosel_vitelic_lh5168(),
        mosel_vitelic_lh5268a(),
        mosel_vitelic_lh52b256(),
        rohm_br62256f(),
        rohm_br6265(),
        rohm_xlj6265(),
        sanyo_lc35256d(),
        sanyo_lc35256f(),
        sanyo_lc3564b(),
        sharp_lh5164an(),
        sharp_lh5164an_2(),
        sharp_lh5160(),
        sharp_lh5168(),
        sharp_lh52256c(),
        sharp_lh52256cvt(),
        sharp_lh51d256t(),
        sharp_lh52cv256(),
        winbond_w24257(),
        winbond_w24258(),
        winbond_w2465(),
        victronix_vn4464s(),
        sharp_lh5164ln(),
        sharp_lh5164n(),
        sharp_lh5264n(),
        sharp_lh5264tn_l(),
        sharp_lh52a64n_l(),
        lsi_logic_lh52xx(),
        crosslink_lh52a64n_yl(),
        crosslink_lh5268anf(),
        mosel_vitelic_lh52a64n_pl(),
        hynix_hy62wt08081e(),
        fujitsu_mb85r256(),
    )
}
