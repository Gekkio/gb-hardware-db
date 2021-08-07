use super::{week2, year2, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Accelerometer {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::accelerometer::accelerometer().parse("2738109451 0028 ADXL202JQC").is_ok());
/// ```
pub fn analog_adxl202jqc() -> &'static impl LabelParser<Accelerometer> {
    single_parser!(
        Accelerometer,
        r#"^[0-9]{10}\ ([0-9]{2})([0-9]{2})\ ADXL202JQC$"#,
        move |c| {
            Ok(Accelerometer {
                chip_type: Some("ADXL202JQC".to_owned()),
                manufacturer: Some(Manufacturer::Analog),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

pub fn accelerometer() -> &'static impl LabelParser<Accelerometer> {
    analog_adxl202jqc()
}
