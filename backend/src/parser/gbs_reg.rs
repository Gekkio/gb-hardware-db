use once_cell::sync::OnceCell;

use super::{week2, year1, Manufacturer, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GbsReg {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_gbs_reg;
/// assert!(parse_gbs_reg("548 592F").is_some());
/// ```
fn mitsumi_1592f() -> MatcherDef<GbsReg> {
    MatcherDef(r#"^([0-9])([0-9]{2})\ 592F$"#, move |c| {
        Ok(GbsReg {
            kind: "MM1592F".to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_gbs_reg(text: &str) -> Option<GbsReg> {
    static MATCHER: OnceCell<Matcher<GbsReg>> = OnceCell::new();
    MATCHER.get_or_init(|| mitsumi_1592f().into()).apply(text)
}
