use lazy_static::lazy_static;

use super::{week2, year2, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MgbAmp {
    pub kind: String,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_mgb_amp;
/// assert!(parse_mgb_amp("AMP MGB IR3R53N 9806 a").is_some());
/// assert!(parse_mgb_amp("AMP MGB IR3R56N 0040 C").is_some());
/// ```
fn mgb_amp() -> MatcherDef<MgbAmp> {
    MatcherDef(
        r#"^AMP\ MGB\ (IR3R53N|IR3R56N)\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]$"#,
        move |c| {
            Ok(MgbAmp {
                kind: c[1].to_owned(),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

pub fn parse_mgb_amp(text: &str) -> Option<MgbAmp> {
    lazy_static! {
        static ref MATCHER: Matcher<MgbAmp> = mgb_amp().into();
    }
    MATCHER.apply(text)
}
