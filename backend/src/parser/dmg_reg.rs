use once_cell::sync::OnceCell;

use super::{week2, year2, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgReg {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_reg;
/// assert!(parse_dmg_reg("DMG-REG IR3E02 9527 CB").is_some());
/// assert!(parse_dmg_reg("DMG-REG IR3E02 9820 n").is_some());
/// assert!(parse_dmg_reg("DMG-REG IR3E02 9024 J").is_some());
/// ```
fn dmg_reg() -> MatcherDef<DmgReg> {
    MatcherDef(
        r#"^DMG-REG\ IR3E02\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(DmgReg {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_dmg_reg(text: &str) -> Option<DmgReg> {
    static MATCHER: OnceCell<Matcher<DmgReg>> = OnceCell::new();
    MATCHER.get_or_init(|| dmg_reg().into()).apply(text)
}
