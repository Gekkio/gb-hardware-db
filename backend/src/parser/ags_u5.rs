use once_cell::sync::OnceCell;

use super::{week2, year1, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgsU5 {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_ags_u5;
/// assert!(parse_ags_u5("422 1581A").is_some());
/// ```
fn mitsumi_mm1581a() -> MatcherDef<AgsU5> {
    MatcherDef(r#"^([0-9])([0-9]{2})\ 1581A$"#, move |c| {
        Ok(AgsU5 {
            kind: Some("MM1581A".to_owned()),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_ags_u5;
/// assert!(parse_ags_u5("2253B 3129").is_some());
/// ```
fn unknown() -> MatcherDef<AgsU5> {
    MatcherDef(r#"^2253B\ ([0-9])([0-9]{2})[0-9]$"#, move |c| {
        Ok(AgsU5 {
            kind: None,
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_ags_u5(text: &str) -> Option<AgsU5> {
    static MATCHER: OnceCell<MatcherSet<AgsU5>> = OnceCell::new();
    MATCHER
        .get_or_init(|| MatcherSet::new(&[mitsumi_mm1581a(), unknown()]))
        .apply(text)
}
