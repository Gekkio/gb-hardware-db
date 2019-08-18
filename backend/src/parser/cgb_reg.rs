use lazy_static::lazy_static;

use super::{week2, year2, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cgb_reg;
/// assert!(parse_cgb_reg("CGB-REG IR3E06N 9839 C").is_ok());
/// ```
fn cgb_reg() -> Matcher<CgbReg> {
    Matcher::new(
        r#"^CGB-REG\ IR3E06N\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_cgb_reg(text: &str) -> Result<CgbReg, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<CgbReg>; 1] = [cgb_reg()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
