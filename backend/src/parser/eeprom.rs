use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Eeprom {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_eeprom;
/// assert!(parse_eeprom("LCS5 040").is_ok());
/// ```
fn lcs5() -> Matcher<Eeprom> {
    Matcher::new(r#"^LCS5\ ([0-9])([0-9]{2})(\ [0-9]{2})?$"#, move |c| {
        Ok(Eeprom {
            chip_type: Some("LCS5".to_owned()),
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_eeprom;
/// assert!(parse_eeprom("LC56 W617 08").is_ok());
/// ```
fn lc56() -> Matcher<Eeprom> {
    Matcher::new(r#"^LC56\ [A-Z][0-9]{3}\ [0-9]{2}$"#, move |_| {
        Ok(Eeprom {
            chip_type: Some("LC56".to_owned()),
            manufacturer: None,
            year: None,
            week: None,
        })
    })
}

pub fn parse_eeprom(text: &str) -> Result<Eeprom, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Eeprom>; 2] = [lcs5(), lc56()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
