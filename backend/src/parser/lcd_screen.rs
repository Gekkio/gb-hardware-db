use lazy_static::lazy_static;

use super::{month2, year1, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LcdScreen {
    pub year: Option<Year>,
    pub month: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_lcd_screen;
/// assert!(parse_lcd_screen("S890220").is_some());
/// ```
fn lcd_screen() -> MatcherDef<LcdScreen> {
    MatcherDef(r#"^.*[0-9]([0-9])([0-9]{2})[0-9]{2}$"#, move |c| {
        Ok(LcdScreen {
            year: Some(year1(&c[1])?),
            month: Some(month2(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_lcd_screen;
/// assert!(parse_lcd_screen("T61102S T61104").is_some());
/// ```
fn lcd_screen2() -> MatcherDef<LcdScreen> {
    MatcherDef(r#"^.*([0-9])([0-9]{2})[0-9]{2}$"#, move |c| {
        Ok(LcdScreen {
            year: Some(year1(&c[1])?),
            month: Some(month2(&c[2])?),
        })
    })
}

pub fn parse_lcd_screen(text: &str) -> Option<LcdScreen> {
    lazy_static! {
        static ref MATCHER: MatcherSet<LcdScreen> =
            MatcherSet::new(&[lcd_screen(), lcd_screen2(),]);
    }
    MATCHER.apply(text)
}
