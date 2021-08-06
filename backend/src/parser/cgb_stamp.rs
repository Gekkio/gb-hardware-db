use once_cell::sync::OnceCell;

use super::{week2, year1, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbStamp {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cgb_stamp;
/// assert!(parse_cgb_stamp("218-2221").is_some());
/// ```
fn cgb_stamp() -> MatcherDef<CgbStamp> {
    MatcherDef(r#"^([0-9]{2})([0-9])[-\ .X]?[0-9]{2,4}Y?$"#, move |c| {
        Ok(CgbStamp {
            year: Some(year1(&c[2])?),
            week: Some(week2(&c[1])?),
        })
    })
}

pub fn parse_cgb_stamp(text: &str) -> Option<CgbStamp> {
    static MATCHER: OnceCell<Matcher<CgbStamp>> = OnceCell::new();
    MATCHER.get_or_init(|| cgb_stamp().into()).apply(text)
}
