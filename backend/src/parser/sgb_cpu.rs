use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SgbCpu {
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_sgb_cpu;
/// assert!(parse_sgb_cpu("SGB-CPU 01 © 1994 Nintendo Ⓜ 1989 Nintendo JAPAN 9434 7 D").is_ok());
/// ```
fn sgb_cpu() -> Matcher<SgbCpu> {
    Matcher::new(
        r#"^SGB-CPU\ 01\ ©\ 1994\ Nintendo\ Ⓜ\ 1989\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [0-9]\ [A-Z]$"#,
        move |c| {
            Ok(SgbCpu {
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_sgb_cpu(text: &str) -> Result<SgbCpu, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<SgbCpu>; 1] = [sgb_cpu(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
