use once_cell::sync::OnceCell;

use super::{year1, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgsU4 {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_ags_u4;
/// assert!(parse_ags_u4("S6403 CU4E0 9723").is_some());
/// ```
fn unknown() -> MatcherDef<AgsU4> {
    MatcherDef(r#"^S6403\ [[:alnum:]]{5}\ [0-9]{4}$"#, move |_| {
        Ok(AgsU4 {
            kind: Some("S6403".to_owned()),
            manufacturer: None,
            year: None,
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_ags_u4;
/// assert!(parse_ags_u4("9753 4862").is_some());
/// ```
fn unknown2() -> MatcherDef<AgsU4> {
    MatcherDef(r#"^(9753)\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(AgsU4 {
            kind: Some(c[1].to_owned()),
            manufacturer: None,
            year: Some(year1(&c[2])?),
            week: None,
        })
    })
}

pub fn parse_ags_u4(text: &str) -> Option<AgsU4> {
    static MATCHER: OnceCell<MatcherSet<AgsU4>> = OnceCell::new();
    MATCHER
        .get_or_init(|| MatcherSet::new(&[unknown(), unknown2()]))
        .apply(text)
}
