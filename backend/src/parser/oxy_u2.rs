use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OxyU2 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_oxy_u2;
/// assert!(parse_oxy_u2("MITSUMI JAPAN 528A PM C").is_ok());
/// ```
fn mitsumi_pm() -> Matcher<OxyU2> {
    Matcher::new(
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

pub fn parse_oxy_u2(text: &str) -> Result<OxyU2, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<OxyU2>; 1] = [mitsumi_pm()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
