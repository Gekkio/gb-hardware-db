use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU2 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_oxy_u2;
/// assert!(parse_oxy_u2("MITSUMI JAPAN 528A PM C").is_some());
/// ```
fn mitsumi_pm() -> MatcherDef<OxyU2> {
    MatcherDef(
        r#"^MITSUMI\ JAPAN\ ([0-9])([0-9]{2})\ ?[A-Z]\ PM\ C$"#,
        move |c| {
            Ok(OxyU2 {
                kind: "PM C".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_oxy_u2(text: &str) -> Option<OxyU2> {
    lazy_static! {
        static ref MATCHER: Matcher<OxyU2> = mitsumi_pm().into();
    }
    MATCHER.apply(text)
}
