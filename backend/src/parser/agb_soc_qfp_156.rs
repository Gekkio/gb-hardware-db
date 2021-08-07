use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::macros::single_parser;

pub type AgbSoc = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_soc_qfp_156::agb_soc_qfp_156().parse("CPU AGB B E Ⓜ © 2002 Nintendo JAPAN ARM 0602 UB").is_ok());
/// ```
pub fn agb_soc_qfp_156() -> &'static impl LabelParser<AgbSoc> {
    single_parser!(
        AgbSoc,
        r#"^(CPU\ AGB\ B(\ E)?)\ Ⓜ\ ©\ 2002\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbSoc {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}
