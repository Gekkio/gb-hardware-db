use super::{week2, year2_u16, LabelParser};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Gen2SocKind {
    Mgb,
    Sgb2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gen2Soc {
    pub kind: Gen2SocKind,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gen2_soc::cpu_mgb().parse("CPU MGB Ⓜ © 1996 Nintendo JAPAN 9808 D").is_ok());
/// assert!(parser::gen2_soc::cpu_mgb().parse("CPU MGB Ⓜ © 1996 Nintendo JAPAN 0040 DA").is_ok());
/// ```
pub fn cpu_mgb() -> &'static impl LabelParser<Gen2Soc> {
    single_parser!(
        Gen2Soc,
        r#"^CPU\ MGB\ Ⓜ\ ©\ 1996\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Gen2Soc {
                kind: Gen2SocKind::Mgb,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gen2_soc::cpu_sgb2().parse("CPU SGB2 Ⓜ 1996 Nintendo © 1997 Nintendo JAPAN 9806 3 E").is_ok());
/// ```
pub fn cpu_sgb2() -> &'static impl LabelParser<Gen2Soc> {
    single_parser!(
        Gen2Soc,
        r#"^CPU\ SGB2\ Ⓜ\ 1996\ Nintendo\ ©\ 1997\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ ?[A-Z]$"#,
        move |c| {
            Ok(Gen2Soc {
                kind: Gen2SocKind::Sgb2,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn gen2_soc() -> &'static impl LabelParser<Gen2Soc> {
    multi_parser!(Gen2Soc, cpu_mgb(), cpu_sgb2())
}
