use super::Ram;
use crate::{
    macros::single_parser,
    parser::{month1_alpha, week2, year1, year2, ChipDateCode, LabelParser, Manufacturer},
};

/// BSI BS62LV256SC (SOP-28)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::bsi_bs62lv256().parse("BSI BS62LV256SC-70 S2827V52155 A0106 TAIWAN").is_ok());
/// assert!(parser::sram::sop_28::bsi_bs62lv256().parse("BSI BS62LV256SC-70 S2828W11075.1 F0231 TAIWAN").is_ok());
/// assert!(parser::sram::sop_28::bsi_bs62lv256().parse("BSI BS62LV256SCG70 S2828CA30125 A D05502 TAIWAN").is_ok());
/// assert!(parser::sram::sop_28::bsi_bs62lv256().parse("BSI BS62LV256SC-70 S2828W13088.1N F0318 TAIWAN").is_ok());
/// ```
pub fn bsi_bs62lv256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^BSI\ (?<kind>BS62LV256)(?<package>S)(?<grade>[CI])(?<material>[GP\-])(?<speed>(55|70))\ [[:alnum:]]{10,12}(.[0-9][A-Z]?)?(\ A)?\ [A-Z](?<year>[0-9]{2})(?<week>[0-9]{2})[0-9]?\ TAIWAN$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}{grade}{material}{speed}",
                    kind = &c["kind"],
                    package = &c["package"],
                    grade = &c["grade"],
                    material = &c["material"],
                    speed = &c["speed"],
                ),
                manufacturer: Some(Manufacturer::Bsi),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Hyundai GM76C256C (SOP-28)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::hyundai_gm76c256c().parse("HYUNDAI GM76C256CLLFW70 0047 KOREA").is_ok());
/// ```
pub fn hyundai_gm76c256c() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HYUNDAI\ (?<kind>GM76C256[ABC]LL)FW70\ (?<year>[0-9]{2})(?<week>[0-9]{2})\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: c["kind"].to_owned(),
                manufacturer: Some(Manufacturer::Hyundai),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Hyundai HY6264 (1994+)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::hyundai_hy6264_new().parse("HY6264A LLJ-10 9902B KOREA").is_ok());
/// ```
pub fn hyundai_hy6264_new() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>HY6264A)\ (?<power>(L|LL))(?<package>J)-(?<speed>10|70)\ (?<year>[0-9]{2})(?<week>[0-9]{2})[A-Z]\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: format!("{}{}", &c["kind"], &c["power"]),
                manufacturer: Some(Manufacturer::Hyundai),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Hyundai HY6264 (1992-1994)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::hyundai_hy6264_old().parse("HYUNDAI HY6264ALLJ-10 9327B KOREA").is_ok());
/// ```
pub fn hyundai_hy6264_old() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HYUNDAI\ (?<kind>HY6264A)(?<power>(L|LL))(?<package>)J-(?<speed>)10\ (?<year>[0-9]{2})(?<week>[0-9]{2})[A-Z]\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: format!("{}{}", &c["kind"], &c["power"]),
                manufacturer: Some(Manufacturer::Hyundai),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// LGS GM76C256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::lgs_gm76c256().parse("LGS GM76C256CLLFW70 9849 KOREA").is_ok());
/// ```
pub fn lgs_gm76c256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^LGS\ (?<kind>GM76C256[ABC]LL)FW70\ (?<year>[0-9]{2})(?<week>[0-9]{2})\ KOREA$"#,
        move |c| {
            Ok(Ram {
                kind: c["kind"].to_owned(),
                manufacturer: Some(Manufacturer::Lgs),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Sanyo LC35256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::sanyo_lc35256().parse("SANYO LC35256DM-70W JAPAN 0EUPG").is_ok());
/// assert!(parser::sram::sop_28::sanyo_lc35256().parse("SANYO LC35256FM-70U JAPAN 0LK5G").is_ok());
/// ```
pub fn sanyo_lc35256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^SANYO\ (?<kind>LC35256[A-F]?)(?<package>M)-(?<speed>[0-9]{2})[[:alnum:]]\ JAPAN\ (?<year>[0-9])(?<month>[A-M])[[:alnum:]]{3}$"#,
        move |c| {
            Ok(Ram {
                kind: c["kind"].to_owned(),
                manufacturer: Some(Manufacturer::Sanyo),
                date_code: Some(ChipDateCode::YearMonth {
                    year: year1(&c["year"])?,
                    month: month1_alpha(&c["month"])?,
                }),
            })
        },
    )
}

/// Sanyo LC3564
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::sanyo_lc3564().parse("SANYO LC3564BM-70 JAPAN 9MUBG").is_ok());
/// ```
pub fn sanyo_lc3564() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^SANYO\ (?<kind>LC3564[A-B]?)(?<package>M)-(?<speed>[0-9]{2})\ JAPAN\ (?<year>[0-9])(?<month>[A-M])[[:alnum:]]{3}$"#,
        move |c| {
            Ok(Ram {
                kind: c["kind"].to_owned(),
                manufacturer: Some(Manufacturer::Sanyo),
                date_code: Some(ChipDateCode::YearMonth {
                    year: year1(&c["year"])?,
                    month: month1_alpha(&c["month"])?,
                }),
            })
        },
    )
}

/// Rohm BR62256F (SOP-28)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::rohm_br62256f().parse("BR62256F-70LL 006 169NA").is_ok());
/// ```
pub fn rohm_br62256f() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>BR62256[AB]?F)-(?<speed>70)(?<power>LL)\ (?<year>[0-9])(?<week>[0-9]{2})\ [0-9]{3}[A-Z]{0,2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}-{speed}{power}",
                    kind = &c["kind"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Rohm BR6265 (SOP-28)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::rohm_br6265().parse("BR6265BF-10SL 111 120N").is_ok());
/// ```
pub fn rohm_br6265() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>BR6265[AB]?F)-(?<speed>10)(?<power>SL)\ (?<year>[0-9])(?<week>[0-9]{2})\ [0-9]{3}[A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}-{speed}{power}",
                    kind = &c["kind"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Rohm XLJ6265
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::rohm_xlj6265().parse("XLJ6265BF-10SL 640 173N").is_ok());
/// ```
pub fn rohm_xlj6265() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>XLJ6265[AB]?F)-(?<speed>10)(?<power>SL)\ (?<year>[0-9])(?<week>[0-9]{2})\ [0-9]{3}[A-Z]{0,2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}-{speed}{power}",
                    kind = &c["kind"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Victronix VN4464
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::victronix_vn4464().parse("Victronix VN4464S-08LL 95103B029").is_ok());
/// ```
pub fn victronix_vn4464() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Victronix\ (?<kind>VN4464)(?<package>S)-(?<speed>08)(?<power>LL)\ (?<year>[0-9]{2})(?<week>[0-9]{2})[0-9][A-Z][0-9]{3}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}-{speed}{power}",
                    kind = &c["kind"],
                    package = &c["package"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Victronix),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Winbond W24257
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::winbond_w24257().parse("Winbond W24257S-70LL 046QB202858301AC").is_ok());
/// ```
pub fn winbond_w24257() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Winbond\ (?<kind>W24257)(?<package>S)-(?<speed>70)(?<power>LL)\ (?<year>[0-9])(?<week>[0-9]{2})[A-Z]{2}[0-9]{9}[A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}-{speed}{power}",
                    kind = &c["kind"],
                    package = &c["package"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Winbond),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Winbond W24258
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::winbond_w24258().parse("Winbond W24258S-70LE 011MH200254401AA").is_ok());
/// ```
pub fn winbond_w24258() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Winbond\ (?<kind>W24258)(?<package>S)-(?<speed>70)(?<power>LE)\ (?<year>[0-9])(?<week>[0-9]{2})[A-Z]{2}[0-9]{9}[A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}-{speed}{power}",
                    kind = &c["kind"],
                    package = &c["package"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Winbond),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Winbond W2465
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::winbond_w2465().parse("Winbond W2465S-70LL 140SD21331480-II1RA").is_ok());
/// ```
pub fn winbond_w2465() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^Winbond\ (?<kind>W2465)(?<package>S)-(?<speed>70)(?<power>LL)\ (?<year>[0-9])(?<week>[0-9]{2})[A-Z]{2}[0-9]{8}-II1RA$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}-{speed}{power}",
                    kind = &c["kind"],
                    package = &c["package"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Winbond),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year1(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Sharp LH52256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_28::sharp_lh52256().parse("LH52256CN-10LL SHARP JAPAN 9832 1 SN").is_ok());
/// ```
pub fn sharp_lh52256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>LH52256C?)(?<package>N)-(?<speed>10)(?<power>LL)\ SHARP\ JAPAN\ (?<year>[0-9]{2})(?<week>[0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}-{speed}{power}",
                    kind = &c["kind"],
                    package = &c["package"],
                    speed = &c["speed"],
                    power = &c["power"],
                ),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}
