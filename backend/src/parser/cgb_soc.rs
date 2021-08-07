use super::{week2, year2_u16, LabelParser};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbSoc {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::cgb_soc::cpu_cgb().parse("CPU CGB B Ⓜ © 1998 Nintendo JAPAN 9842 I").is_ok());
/// ```
pub fn cpu_cgb() -> &'static impl LabelParser<CgbSoc> {
    single_parser!(
        CgbSoc,
        r#"^(CPU\ CGB(\ [A-E])?)\ Ⓜ\ ©\ (1998|2000)\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbSoc {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

pub fn cgb_soc() -> &'static impl LabelParser<CgbSoc> {
    cpu_cgb()
}
