use super::{week2, year2, LabelParser, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_reg::sharp_ir3e09n().parse("AGB-REG IR3E09N 0104 C").is_ok());
/// ```
pub fn sharp_ir3e09n() -> &'static impl LabelParser<AgbReg> {
    single_parser!(
        AgbReg,
        r#"^AGB-REG\ IR3E09N\ ([A0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn agb_reg() -> &'static impl LabelParser<AgbReg> {
    sharp_ir3e09n()
}
