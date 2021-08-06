use once_cell::sync::OnceCell;

use super::{month2, year1, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgStamp {
    pub year: Option<Year>,
    pub month: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_stamp;
/// assert!(parse_dmg_stamp("010 23").is_some());
/// assert!(parse_dmg_stamp("903-22").is_some());
/// assert!(parse_dmg_stamp("709.3901").is_some());
/// assert!(parse_dmg_stamp("202-0007").is_some());
/// ```
fn dmg_stamp() -> MatcherDef<DmgStamp> {
    MatcherDef(r#"^([0-9])([0-9]{2})[-\ .][0-9-]{2,4}Y?$"#, move |c| {
        Ok(DmgStamp {
            year: Some(year1(&c[1])?),
            month: Some(month2(&c[2])?),
        })
    })
}

pub fn parse_dmg_stamp(text: &str) -> Option<DmgStamp> {
    static MATCHER: OnceCell<Matcher<DmgStamp>> = OnceCell::new();
    MATCHER.get_or_init(|| dmg_stamp().into()).apply(text)
}
