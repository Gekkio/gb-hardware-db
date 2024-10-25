use super::Ram;
use crate::{
    macros::single_parser,
    parser::{week2, year1, year2, LabelParser, Manufacturer, PartDateCode},
};

/// Sharp LH52256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::tsop_i_28::sharp_lh52256().parse("LH52256CT-10LL SHARP JAPAN 9824 3 SF").is_ok());
/// assert!(parser::sram::tsop_i_28::sharp_lh52256().parse("LH52256CVT SHARP JAPAN 9841 3 LO").is_ok());
/// ```
pub fn sharp_lh52256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(?<kind>LH52256C?V?)(?<package>T)(?<extra>-(?<speed>10)(?<power>LL))?\ SHARP\ JAPAN\ (?<year>[0-9]{2})(?<week>[0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{package}{extra}",
                    kind = &c["kind"],
                    package = &c["package"],
                    extra = c.name("extra").map_or("", |m| m.as_str())
                ),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Sharp LH52CV256
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::tsop_i_28::sharp_lh52cv256().parse("LH52CV256JT-10LL SHARP JAPAN 9814 7 SA").is_ok());
/// ```
pub fn sharp_lh52cv256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH52CV256[A-Z]{1,2}-[0-9]{2}[A-Z]{0,2})\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]{2}$"#,
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

/// Sharp LH51D256T
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::tsop_i_28::sharp_lh51d256().parse("LH51D256T-Z7 SHARP Y013 5 J").is_ok());
/// assert!(parser::sram::tsop_i_28::sharp_lh51d256().parse("LH51D256T-Z7 SHARP JAPAN Y0 47 3 JA").is_ok());
/// ```
pub fn sharp_lh51d256() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^(LH51D256T-Z[0-9])\ SHARP(\ JAPAN)?\ A?Y([0-9])\ ?([0-9]{2})\ [0-9]\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Ram {
                kind: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                date_code: Some(PartDateCode::YearWeek {
                    year: year1(&c[3])?,
                    week: week2(&c[4])?,
                }),
            })
        },
    )
}
