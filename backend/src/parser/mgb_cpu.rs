use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MgbCpu {
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_mgb_cpu;
/// assert!(parse_mgb_cpu("CPU MGB Ⓜ © 1996 Nintendo JAPAN 9808 D").is_ok());
/// assert!(parse_mgb_cpu("CPU MGB Ⓜ © 1996 Nintendo JAPAN 0040 DA").is_ok());
/// ```
fn mgb_cpu() -> Matcher<MgbCpu> {
    Matcher::new(
        r#"^CPU\ MGB\ Ⓜ\ ©\ 1996\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(MgbCpu {
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_mgb_cpu(text: &str) -> Result<MgbCpu, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<MgbCpu>; 1] = [mgb_cpu()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
