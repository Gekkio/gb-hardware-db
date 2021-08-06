use once_cell::sync::OnceCell;

use super::{week2, year2_u16, MatcherDef, MatcherSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbCpu {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_cpu;
/// assert!(parse_agb_cpu("CPU AGB Ⓜ © 2000 Nintendo JAPAN ARM 0104 I").is_some());
/// ```
fn agb_cpu() -> MatcherDef<AgbCpu> {
    MatcherDef(
        r#"^(CPU\ AGB(\ A(\ E)?)?)\ Ⓜ\ ©\ 2000\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbCpu {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_cpu;
/// assert!(parse_agb_cpu("CPU AGB B E Ⓜ © 2002 Nintendo JAPAN ARM 0602 UB").is_some());
/// ```
fn agb_cpu_b() -> MatcherDef<AgbCpu> {
    MatcherDef(
        r#"^(CPU\ AGB\ B(\ E)?)\ Ⓜ\ ©\ 2002\ Nintendo\ JAPAN\ ARM\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbCpu {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_cpu;
/// assert!(parse_agb_cpu("0529 2m CPU AGB E Ⓜ © 2004 Nintendo JAPAN ARM").is_some());
/// ```
fn agb_cpu_e() -> MatcherDef<AgbCpu> {
    MatcherDef(
        r#"^([0-9]{2})([0-9]{2})\ 2m\ (CPU\ AGB\ E)\ Ⓜ\ ©\ 2004\ Nintendo\ JAPAN\ ARM$"#,
        move |c| {
            Ok(AgbCpu {
                kind: c[3].to_owned(),
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_agb_cpu(text: &str) -> Option<AgbCpu> {
    static MATCHER: OnceCell<MatcherSet<AgbCpu>> = OnceCell::new();
    MATCHER
        .get_or_init(|| MatcherSet::new(&[agb_cpu(), agb_cpu_b(), agb_cpu_e()]))
        .apply(text)
}
