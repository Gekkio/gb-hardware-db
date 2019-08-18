use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

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
/// assert!(parse_gen2_cpu("CPU MGB Ⓜ © 1996 Nintendo JAPAN 9808 D").is_ok());
/// assert!(parse_gen2_cpu("CPU MGB Ⓜ © 1996 Nintendo JAPAN 0040 DA").is_ok());
/// ```
fn mgb_cpu() -> Matcher<Gen2Cpu> {
    Matcher::new(
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
/// assert!(parse_gen2_cpu("CPU SGB2 Ⓜ 1996 Nintendo © 1997 Nintendo JAPAN 9806 3 E").is_ok());
/// ```
fn sgb2_cpu() -> Matcher<Gen2Cpu> {
    Matcher::new(
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

pub fn parse_gen2_cpu(text: &str) -> Result<Gen2Cpu, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Gen2Cpu>; 2] = [mgb_cpu(), sgb2_cpu(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
