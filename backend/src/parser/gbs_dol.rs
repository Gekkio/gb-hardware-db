use lazy_static::lazy_static;

use super::{week2, year2, Manufacturer, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GbsDol {
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_gbs_dol;
/// assert!(parse_gbs_dol("Nintendo GBS-DOL 011 0623L3001").is_some());
/// ```
fn unknown() -> MatcherDef<GbsDol> {
    MatcherDef(
        r#"^Nintendo\ GBS-DOL\ 011\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(GbsDol {
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_gbs_dol(text: &str) -> Option<GbsDol> {
    lazy_static! {
        static ref MATCHER: Matcher<GbsDol> = unknown().into();
    }
    MATCHER.apply(text)
}
