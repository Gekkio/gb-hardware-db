use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU2 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::oxy_u2::mitsumi_pm().parse("MITSUMI JAPAN 528A PM C").is_ok());
/// ```
pub fn mitsumi_pm() -> &'static impl LabelParser<OxyU2> {
    single_parser!(
        OxyU2,
        r#"^MITSUMI\ JAPAN\ ([0-9])([0-9]{2})\ ?[A-Z]\ PM\ C$"#,
        move |c| {
            Ok(OxyU2 {
                kind: "PM C".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn oxy_u2() -> &'static impl LabelParser<OxyU2> {
    mitsumi_pm()
}
