use lazy_static::lazy_static;

use super::{year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgsU4 {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_ags_u4;
/// assert!(parse_ags_u4("S6403 CU4E0 9723").is_ok());
/// ```
fn unknown() -> Matcher<AgsU4> {
    Matcher::new(r#"^S6403\ [[:alnum:]]{5}\ [0-9]{4}$"#, move |_| {
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
/// assert!(parse_ags_u4("9753 4862").is_ok());
/// ```
fn unknown2() -> Matcher<AgsU4> {
    Matcher::new(r#"^(9753)\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(AgsU4 {
            kind: Some(c[1].to_owned()),
            manufacturer: None,
            year: Some(year1(&c[2])?),
            week: None,
        })
    })
}

pub fn parse_ags_u4(text: &str) -> Result<AgsU4, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<AgsU4>; 2] = [unknown(), unknown2()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
