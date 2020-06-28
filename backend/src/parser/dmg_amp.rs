use lazy_static::lazy_static;

use super::{week2, year2, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgAmp {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_amp;
/// assert!(parse_dmg_amp("DMG-AMP IR3R40 9222 AA").is_some());
/// assert!(parse_dmg_amp("DMG-AMP IR3R40 8909 A").is_some());
/// ```
fn dmg_amp() -> MatcherDef<DmgAmp> {
    MatcherDef(
        r#"^DMG-AMP\ IR3R40\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(DmgAmp {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_dmg_amp(text: &str) -> Option<DmgAmp> {
    lazy_static! {
        static ref MATCHER: Matcher<DmgAmp> = dmg_amp().into();
    }
    MATCHER.apply(text)
}
