use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GbsReg {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::gbs_reg::mitsumi_mm1592f().parse("548 592F").is_ok());
/// ```
pub fn mitsumi_mm1592f() -> &'static impl LabelParser<GbsReg> {
    single_parser!(GbsReg, r#"^([0-9])([0-9]{2})\ 592F$"#, move |c| {
        Ok(GbsReg {
            kind: "MM1592F".to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn gbs_reg() -> &'static impl LabelParser<GbsReg> {
    mitsumi_mm1592f()
}
