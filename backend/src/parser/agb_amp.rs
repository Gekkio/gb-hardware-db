use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbAmp {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_amp;
/// assert!(parse_agb_amp("AMP AGB IR3R60N 0103 a").is_ok());
/// ```
fn sharp_ir3r60n() -> Matcher<AgbAmp> {
    Matcher::new(
        r#"^AMP\ AGB\ IR3R60N\ ([A0-9]{2})([0-9]{2})\ [A-Za-z]$"#,
        move |c| {
            Ok(AgbAmp {
                kind: "IR3R60N".to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_amp;
/// assert!(parse_agb_amp("BH7835AFS 337 T22").is_ok());
/// ```
fn rohm_bh7835afs() -> Matcher<AgbAmp> {
    Matcher::new(
        r#"^BH7835AFS\ ([0-9])([0-9]{2})\ [[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(AgbAmp {
                kind: "BH7835AFS".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_agb_amp;
/// assert!(parse_agb_amp("MITSUMI JAPAN 602E PM B3").is_ok());
/// ```
fn mitsumi_pm() -> Matcher<AgbAmp> {
    // FIXME: Not really an amplifier
    Matcher::new(
        r#"^MITSUMI\ JAPAN\ ([0-9])([0-9]{2})[A-Z]\ (PM\ B[0-9])$"#,
        move |c| {
            Ok(AgbAmp {
                kind: c[3].to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_agb_amp(text: &str) -> Result<AgbAmp, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<AgbAmp>; 3] =
            [sharp_ir3r60n(), rohm_bh7835afs(), mitsumi_pm()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
