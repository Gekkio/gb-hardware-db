use lazy_static::lazy_static;

use super::{week2, year2, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_reg;
/// assert!(parse_dmg_reg("DMG-REG IR3E02 9527 CB").is_ok());
/// assert!(parse_dmg_reg("DMG-REG IR3E02 9820 n").is_ok());
/// assert!(parse_dmg_reg("DMG-REG IR3E02 9024 J").is_ok());
/// ```
fn dmg_reg() -> Matcher<DmgReg> {
    Matcher::new(
        r#"^DMG-REG\ IR3E02\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(DmgReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_dmg_reg(text: &str) -> Result<DmgReg, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<DmgReg>; 1] = [dmg_reg()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
