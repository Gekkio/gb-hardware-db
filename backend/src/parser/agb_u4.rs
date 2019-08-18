use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbU4 {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_u4;
/// assert!(parse_agb_u4("105 514X").is_ok());
/// ```
fn mitsumi_mm1514x() -> Matcher<AgbU4> {
    Matcher::new(r#"^([0-9])([0-5][0-9])\ 514X$"#, move |c| {
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
/// assert!(parse_agb_u4("081 514X").is_ok());
/// ```
fn mitsumi_mm1514x_2() -> Matcher<AgbU4> {
    Matcher::new(r#"^([0-9])[0-9]{2}\ 514X$"#, move |c| {
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
/// assert!(parse_agb_u4("S6960 E-U2Z C700").is_ok());
/// ```
fn unknown() -> Matcher<AgbU4> {
    Matcher::new(r#"^S6960\ E-U2Z\ C700$"#, move |_| {
        Ok(AgbU4 {
            kind: Some("S6960".to_owned()),
            manufacturer: None,
            year: None,
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_u4;
/// assert!(parse_agb_u4("9750A 1581").is_ok());
/// assert!(parse_agb_u4("9750B 2A69").is_ok());
/// ```
fn unknown2() -> Matcher<AgbU4> {
    Matcher::new(r#"^(9750[AB])\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(AgbU4 {
            kind: Some(c[1].to_owned()),
            manufacturer: None,
            year: Some(year1(&c[2])?),
            week: None,
        })
    })
}

pub fn parse_agb_u4(text: &str) -> Result<AgbU4, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<AgbU4>; 4] = [
            mitsumi_mm1514x(),
            mitsumi_mm1514x_2(),
            unknown(),
            unknown2()
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
