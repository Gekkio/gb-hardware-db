use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU4 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_oxy_u4;
/// assert!(parse_oxy_u4("AKV 522").is_ok());
/// ```
fn unknown() -> Matcher<OxyU4> {
    Matcher::new(r#"^AKV\ ([0-9])([0-9]{2})$"#, move |c| {
        Ok(OxyU4 {
            kind: "AKV".to_owned(),
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_oxy_u4(text: &str) -> Result<OxyU4, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<OxyU4>; 1] = [unknown()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
