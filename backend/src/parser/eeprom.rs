use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Eeprom {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_eeprom;
/// assert!(parse_eeprom("LCS5 040").is_some());
/// ```
fn lcs5() -> MatcherDef<Eeprom> {
    MatcherDef(r#"^LCS5\ ([0-9])([0-9]{2})(\ [0-9]{2})?$"#, move |c| {
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
/// assert!(parse_eeprom("LC56 W617 08").is_some());
/// ```
fn lc56() -> MatcherDef<Eeprom> {
    MatcherDef(r#"^LC56\ [A-Z][0-9]{3}\ [0-9]{2}$"#, move |_| {
        Ok(Eeprom {
            chip_type: Some("LC56".to_owned()),
            manufacturer: None,
            year: None,
            week: None,
        })
    })
}

pub fn parse_eeprom(text: &str) -> Option<Eeprom> {
    lazy_static! {
        static ref MATCHER: MatcherSet<Eeprom> = MatcherSet::new(&[lcs5(), lc56()]);
    }
    MATCHER.apply(text)
}
