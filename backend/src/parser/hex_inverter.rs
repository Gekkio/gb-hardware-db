use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexInverter {
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::hex_inverter::toshiba_tc74lvx04ft().parse("LVX 04 8 45").is_ok());
/// ```
pub fn toshiba_tc74lvx04ft() -> &'static impl LabelParser<HexInverter> {
    single_parser!(HexInverter, r#"^LVX\ 04\ ([0-9])\ ([0-9]{2})$"#, move |c| {
        Ok(HexInverter {
            manufacturer: Some(Manufacturer::Toshiba),
            chip_type: Some("TC74LVX04FT".to_owned()),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn hex_inverter() -> &'static impl LabelParser<HexInverter> {
    toshiba_tc74lvx04ft()
}
