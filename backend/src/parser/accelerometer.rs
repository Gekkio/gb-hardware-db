use lazy_static::lazy_static;

use super::{week2, year2, Manufacturer, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Accelerometer {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_accelerometer;
/// assert!(parse_accelerometer("2738109451 0028 ADXL202JQC").is_some());
/// ```
fn analog_adxl202jqc() -> MatcherDef<Accelerometer> {
    MatcherDef(
        r#"^[0-9]{10}\ ([0-9]{2})([0-9]{2})\ ADXL202JQC?$"#,
        move |c| {
            Ok(Accelerometer {
                chip_type: Some("ADXL202JQC".to_owned()),
                manufacturer: Some(Manufacturer::Analog),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_accelerometer(text: &str) -> Option<Accelerometer> {
    lazy_static! {
        static ref MATCHER: Matcher<Accelerometer> = analog_adxl202jqc().into();
    }
    MATCHER.apply(text)
}
