use lazy_static::lazy_static;

use super::{week2, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU5 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_oxy_u5;
/// assert!(parse_oxy_u5("CP6465 B 02 KOR0531 635963").is_ok());
/// ```
fn unknown() -> Matcher<OxyU5> {
    Matcher::new(
        r#"^CP6465\ B\ 02\ KOR([0-9]{2})([0-9]{2})\ [0-9]{6}$"#,
        move |c| {
            Ok(OxyU5 {
                kind: "CP6465".to_owned(),
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_oxy_u5(text: &str) -> Result<OxyU5, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<OxyU5>; 1] = [unknown()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
