use lazy_static::lazy_static;

use super::{week2, year2, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgAmp {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_amp;
/// assert!(parse_dmg_amp("DMG-AMP IR3R40 9222 AA").is_ok());
/// assert!(parse_dmg_amp("DMG-AMP IR3R40 8909 A").is_ok());
/// ```
fn dmg_amp() -> Matcher<DmgAmp> {
    Matcher::new(
        r#"^DMG-AMP\ IR3R40\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(DmgAmp {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_dmg_amp(text: &str) -> Result<DmgAmp, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<DmgAmp>; 1] = [dmg_amp()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
