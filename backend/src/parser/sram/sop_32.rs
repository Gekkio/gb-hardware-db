use super::Ram;
use crate::{
    macros::single_parser,
    parser::{week2, year2, ChipDateCode, LabelParser, Manufacturer},
};

/// Hyundai HY628100 (SOP-32)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::sram::sop_32::hyundai_hy628100().parse("HYUNDAI KOREA HY628100B 0041A LLG-70").is_ok());
/// ```
pub fn hyundai_hy628100() -> &'static impl LabelParser<Ram> {
    single_parser!(
        Ram,
        r#"^HYUNDAI\ KOREA\ (?<kind>HY628100[AB]?)\ (?<year>[0-9]{2})(?<week>[0-9]{2})[A-Z]\ (?<power>(L|LL))(?<package>G)\-(?<speed>(50|55|70|85))$"#,
        move |c| {
            Ok(Ram {
                kind: format!(
                    "{kind}{power}{package}-{speed}",
                    kind = &c["kind"],
                    power = &c["power"],
                    package = &c["package"],
                    speed = &c["speed"]
                ),
                manufacturer: Some(Manufacturer::Hyundai),
                date_code: Some(ChipDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}
