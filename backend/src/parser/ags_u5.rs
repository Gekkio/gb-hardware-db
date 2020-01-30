use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgsU5 {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_ags_u5;
/// assert!(parse_ags_u5("422 1581A").is_ok());
/// ```
fn mitsumi_mm1581a() -> Matcher<AgsU5> {
    Matcher::new(r#"^([0-9])([0-9]{2})\ 1581A$"#, move |c| {
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
/// assert!(parse_ags_u5("2253B 3129").is_ok());
/// ```
fn unknown() -> Matcher<AgsU5> {
    Matcher::new(r#"^2253B\ ([0-9])([0-9]{2})[0-9]$"#, move |c| {
        Ok(AgsU5 {
            kind: None,
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_ags_u5(text: &str) -> Result<AgsU5, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<AgsU5>; 2] = [mitsumi_mm1581a(), unknown(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
