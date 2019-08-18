use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sgb2Cpu {
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_sgb2_cpu;
/// assert!(parse_sgb2_cpu("CPU SGB2 Ⓜ 1996 Nintendo © 1997 Nintendo JAPAN 9806 3 E").is_ok());
/// ```
fn sgb2_cpu() -> Matcher<Sgb2Cpu> {
    Matcher::new(
        r#"^CPU\ SGB2\ Ⓜ\ 1996\ Nintendo\ ©\ 1997\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ ?[A-Z]$"#,
        move |c| {
            Ok(Sgb2Cpu {
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_sgb2_cpu(text: &str) -> Result<Sgb2Cpu, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Sgb2Cpu>; 1] = [sgb2_cpu(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
