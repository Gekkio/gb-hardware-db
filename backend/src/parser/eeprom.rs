use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Eeprom {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::eeprom::lcs5().parse("LCS5 040").is_ok());
/// ```
pub fn lcs5() -> &'static impl LabelParser<Eeprom> {
    single_parser!(
        Eeprom,
        r#"^LCS5\ ([0-9])([0-9]{2})(\ [0-9]{2})?$"#,
        move |c| {
            Ok(Eeprom {
                chip_type: Some("LCS5".to_owned()),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::eeprom::lc56().parse("LC56 W617 08").is_ok());
/// ```
pub fn lc56() -> &'static impl LabelParser<Eeprom> {
    single_parser!(Eeprom, r#"^LC56\ [A-Z][0-9]{3}\ [0-9]{2}$"#, move |_| {
        Ok(Eeprom {
            chip_type: Some("LC56".to_owned()),
            manufacturer: None,
            year: None,
            week: None,
        })
    })
}

pub fn eeprom() -> &'static impl LabelParser<Eeprom> {
    multi_parser!(Eeprom, lcs5(), lc56())
}
