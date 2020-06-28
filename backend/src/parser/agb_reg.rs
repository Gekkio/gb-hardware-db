use lazy_static::lazy_static;

use super::{week2, year2, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_reg;
/// assert!(parse_agb_reg("AGB-REG IR3E09N 0104 C").is_some());
/// ```
fn agb_reg() -> MatcherDef<AgbReg> {
    MatcherDef(
        r#"^AGB-REG\ IR3E09N\ ([A0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(AgbReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_agb_reg(text: &str) -> Option<AgbReg> {
    lazy_static! {
        static ref MATCHER: Matcher<AgbReg> = agb_reg().into();
    }
    MATCHER.apply(text)
}
