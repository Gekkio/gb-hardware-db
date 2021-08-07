use super::{week2, year2, LabelParser, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::cgb_reg::sharp_ir3e06n().parse("CGB-REG IR3E06N 9839 C").is_ok());
/// ```
pub fn sharp_ir3e06n() -> &'static impl LabelParser<CgbReg> {
    single_parser!(
        CgbReg,
        r#"^CGB-REG\ IR3E06N\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

pub fn cgb_reg() -> &'static impl LabelParser<CgbReg> {
    sharp_ir3e06n()
}
