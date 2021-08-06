use once_cell::sync::OnceCell;

use super::{week2, year1, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbU4 {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_u4;
/// assert!(parse_agb_u4("105 514X").is_some());
/// ```
fn mitsumi_mm1514x() -> MatcherDef<AgbU4> {
    MatcherDef(r#"^([0-9])([0-5][0-9])\ 514X$"#, move |c| {
        Ok(AgbU4 {
            kind: Some("MM1514X".to_owned()),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_u4;
/// assert!(parse_agb_u4("081 514X").is_some());
/// ```
fn mitsumi_mm1514x_2() -> MatcherDef<AgbU4> {
    MatcherDef(r#"^([0-9])[0-9]{2}\ 514X$"#, move |c| {
        Ok(AgbU4 {
            kind: Some("MM1514X".to_owned()),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_u4;
/// assert!(parse_agb_u4("S6960 E-U2Z C700").is_some());
/// assert!(parse_agb_u4("S6960 E-U2X C410").is_some());
/// ```
fn unknown() -> MatcherDef<AgbU4> {
    MatcherDef(r#"^S6960\ E-U([0-9])[A-Z]\ C[0-9]{3}$"#, move |c| {
        Ok(AgbU4 {
            kind: Some("S6960".to_owned()),
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_u4;
/// assert!(parse_agb_u4("9750A 1581").is_some());
/// assert!(parse_agb_u4("9750B 2A69").is_some());
/// ```
fn unknown2() -> MatcherDef<AgbU4> {
    MatcherDef(r#"^(9750[AB])\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(AgbU4 {
            kind: Some(c[1].to_owned()),
            manufacturer: None,
            year: Some(year1(&c[2])?),
            week: None,
        })
    })
}

pub fn parse_agb_u4(text: &str) -> Option<AgbU4> {
    static MATCHER: OnceCell<MatcherSet<AgbU4>> = OnceCell::new();
    MATCHER
        .get_or_init(|| {
            MatcherSet::new(&[
                mitsumi_mm1514x(),
                mitsumi_mm1514x_2(),
                unknown(),
                unknown2(),
            ])
        })
        .apply(text)
}
