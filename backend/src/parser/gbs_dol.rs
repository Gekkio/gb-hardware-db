use super::{week2, year2, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GbsDol {
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gbs_dol::unknown().parse("Nintendo GBS-DOL 011 0623L3001").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<GbsDol> {
    single_parser!(
        GbsDol,
        r#"^Nintendo\ GBS-DOL\ 011\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(GbsDol {
                manufacturer: None,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn gbs_dol() -> &'static impl LabelParser<GbsDol> {
    unknown()
}
