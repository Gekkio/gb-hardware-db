use lazy_static::lazy_static;

use super::{week2, year2, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MgbAmp {
    pub kind: String,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_mgb_amp;
/// assert!(parse_mgb_amp("AMP MGB IR3R53N 9806 a").is_ok());
/// assert!(parse_mgb_amp("AMP MGB IR3R56N 0040 C").is_ok());
/// ```
fn mgb_amp() -> Matcher<MgbAmp> {
    Matcher::new(
        r#"^AMP\ MGB\ (IR3R53N|IR3R56N)\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]$"#,
        move |c| {
            Ok(MgbAmp {
                kind: c[1].to_owned(),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

pub fn parse_mgb_amp(text: &str) -> Result<MgbAmp, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<MgbAmp>; 1] = [mgb_amp()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
