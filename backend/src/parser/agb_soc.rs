use super::{week2, year2_u16, LabelParser};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbSoc {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_soc::cpu_agb_0_a_ae().parse("CPU AGB Ⓜ © 2000 Nintendo JAPAN ARM 0104 I").is_ok());
/// ```
pub fn cpu_agb_0_a_ae() -> &'static impl LabelParser<AgbSoc> {
    single_parser!(
        AgbSoc,
        r#"^(CPU\ AGB(\ A(\ E)?)?)\ Ⓜ\ ©\ 2000\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbSoc {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_soc::cpu_agb_b_be().parse("CPU AGB B E Ⓜ © 2002 Nintendo JAPAN ARM 0602 UB").is_ok());
/// ```
pub fn cpu_agb_b_be() -> &'static impl LabelParser<AgbSoc> {
    single_parser!(
        AgbSoc,
        r#"^(CPU\ AGB\ B(\ E)?)\ Ⓜ\ ©\ 2002\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbSoc {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_soc::cpu_agb_e().parse("0529 2m CPU AGB E Ⓜ © 2004 Nintendo JAPAN ARM").is_ok());
/// ```
pub fn cpu_agb_e() -> &'static impl LabelParser<AgbSoc> {
    single_parser!(
        AgbSoc,
        r#"^([0-9]{2})([0-9]{2})\ 2m\ (CPU\ AGB\ E)\ Ⓜ\ ©\ 2004\ Nintendo\ JAPAN\ ARM$"#,
        move |c| {
            Ok(AgbSoc {
                kind: c[3].to_owned(),
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn agb_soc() -> &'static impl LabelParser<AgbSoc> {
    multi_parser!(AgbSoc, cpu_agb_0_a_ae(), cpu_agb_b_be(), cpu_agb_e())
}
