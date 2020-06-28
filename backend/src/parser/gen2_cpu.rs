use lazy_static::lazy_static;

use super::{week2, year2_u16, MatcherDef, MatcherSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Gen2CpuKind {
    Mgb,
    Sgb2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gen2Cpu {
    pub kind: Gen2CpuKind,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_gen2_cpu;
/// assert!(parse_gen2_cpu("CPU MGB Ⓜ © 1996 Nintendo JAPAN 9808 D").is_some());
/// assert!(parse_gen2_cpu("CPU MGB Ⓜ © 1996 Nintendo JAPAN 0040 DA").is_some());
/// ```
fn mgb_cpu() -> MatcherDef<Gen2Cpu> {
    MatcherDef(
        r#"^CPU\ MGB\ Ⓜ\ ©\ 1996\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(Gen2Cpu {
                kind: Gen2CpuKind::Mgb,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_gen2_cpu;
/// assert!(parse_gen2_cpu("CPU SGB2 Ⓜ 1996 Nintendo © 1997 Nintendo JAPAN 9806 3 E").is_some());
/// ```
fn sgb2_cpu() -> MatcherDef<Gen2Cpu> {
    MatcherDef(
        r#"^CPU\ SGB2\ Ⓜ\ 1996\ Nintendo\ ©\ 1997\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ ?[A-Z]$"#,
        move |c| {
            Ok(Gen2Cpu {
                kind: Gen2CpuKind::Sgb2,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_gen2_cpu(text: &str) -> Option<Gen2Cpu> {
    lazy_static! {
        static ref MATCHER: MatcherSet<Gen2Cpu> = MatcherSet::new(&[mgb_cpu(), sgb2_cpu()]);
    }
    MATCHER.apply(text)
}
