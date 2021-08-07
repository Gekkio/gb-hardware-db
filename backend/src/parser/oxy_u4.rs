use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU4 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::oxy_u4::unknown().parse("AKV 522").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<OxyU4> {
    single_parser!(OxyU4, r#"^AKV\ ([0-9])([0-9]{2})$"#, move |c| {
        Ok(OxyU4 {
            kind: "AKV".to_owned(),
            manufacturer: None,
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn oxy_u4() -> &'static impl LabelParser<OxyU4> {
    unknown()
}
