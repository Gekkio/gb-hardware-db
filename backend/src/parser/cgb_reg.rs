use lazy_static::lazy_static;

use super::{week2, year2, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cgb_reg;
/// assert!(parse_cgb_reg("CGB-REG IR3E06N 9839 C").is_some());
/// ```
fn cgb_reg() -> MatcherDef<CgbReg> {
    MatcherDef(
        r#"^CGB-REG\ IR3E06N\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_cgb_reg(text: &str) -> Option<CgbReg> {
    lazy_static! {
        static ref MATCHER: Matcher<CgbReg> = cgb_reg().into();
    }
    MATCHER.apply(text)
}
