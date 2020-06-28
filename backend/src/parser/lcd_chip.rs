use lazy_static::lazy_static;

use super::{month2, week2, year1, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LcdChip {
    pub year: Option<Year>,
    pub month: Option<u8>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_lcd_chip;
/// assert!(parse_lcd_chip("110").is_some());
/// ```
fn lcd_chip_old() -> MatcherDef<LcdChip> {
    MatcherDef(r#"^([0-9])([0-9]{2})$"#, move |c| {
        Ok(LcdChip {
            year: Some(year1(&c[1])?),
            month: Some(month2(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_lcd_chip;
/// assert!(parse_lcd_chip("5341").is_some());
/// ```
fn lcd_chip_new() -> MatcherDef<LcdChip> {
    MatcherDef(r#"^([0-9])([0-9]{2})[0-9]$"#, move |c| {
        Ok(LcdChip {
            year: Some(year1(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_lcd_chip(text: &str) -> Option<LcdChip> {
    lazy_static! {
        static ref MATCHER: MatcherSet<LcdChip> =
            MatcherSet::new(&[lcd_chip_old(), lcd_chip_new(),]);
    }
    MATCHER.apply(text)
}
