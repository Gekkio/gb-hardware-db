use lazy_static::lazy_static;

use super::{week2, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flash {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// Macronix 29F008 flash
///
/// ```
/// # use gbhwdb_backend::parser::parse_flash;
/// assert!(parse_flash("E991012 29F008TC-14 21534 TAIWAN").is_ok());
/// ```
fn macronix() -> Matcher<Flash> {
    Matcher::new(
        r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}\ (29F008[A-Z]{2}-[0-9]{2})\ [0-9]{5}\ TAIWAN$"#,
        move |c| {
            Ok(Flash {
                chip_type: Some(format!("MX{}", &c[3])),
                manufacturer: Some(Manufacturer::Macronix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_flash(text: &str) -> Result<Flash, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Flash>; 1] = [macronix(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
