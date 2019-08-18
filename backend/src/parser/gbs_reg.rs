use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GbsReg {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_gbs_reg;
/// assert!(parse_gbs_reg("548 592F").is_ok());
/// ```
fn mitsumi_1592f() -> Matcher<GbsReg> {
    Matcher::new(r#"^([0-9])([0-9]{2})\ 592F$"#, move |c| {
        Ok(GbsReg {
            kind: "MM1592F".to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_gbs_reg(text: &str) -> Result<GbsReg, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<GbsReg>; 1] = [mitsumi_1592f()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
