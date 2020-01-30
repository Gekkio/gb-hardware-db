use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbCpu {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cgb_cpu;
/// assert!(parse_cgb_cpu("CPU CGB B Ⓜ © 1998 Nintendo JAPAN 9842 I").is_ok());
/// ```
fn cgb_cpu() -> Matcher<CgbCpu> {
    Matcher::new(
        r#"^(CPU\ CGB(\ [A-E])?)\ Ⓜ\ ©\ (1998|2000)\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbCpu {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

pub fn parse_cgb_cpu(text: &str) -> Result<CgbCpu, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<CgbCpu>; 1] = [cgb_cpu()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
