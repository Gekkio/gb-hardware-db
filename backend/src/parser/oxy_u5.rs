use super::{week2, year2, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU5 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::oxy_u5::unknown().parse("CP6465 B 02 KOR0531 635963").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<OxyU5> {
    single_parser!(
        OxyU5,
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

pub fn oxy_u5() -> &'static impl LabelParser<OxyU5> {
    unknown()
}
