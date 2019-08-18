use lazy_static::lazy_static;

use super::{month2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LcdScreen {
    pub year: Option<u16>,
    pub month: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_lcd_screen;
/// assert!(parse_lcd_screen("S890220").is_ok());
/// ```
fn lcd_screen() -> Matcher<LcdScreen> {
    Matcher::new(
        r#"^(AE|AH|S|ST|SY)?\ ?([0-9]{2})([0-9]{2})[0-9]{2}$"#,
        move |c| {
            Ok(LcdScreen {
                year: Some(year2_u16(&c[2])?),
                month: Some(month2(&c[3])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_lcd_screen;
/// assert!(parse_lcd_screen("S890220").is_ok());
/// ```
fn lcd_screen2() -> Matcher<LcdScreen> {
    Matcher::new(
        r#"^[[:alnum:]]{1,3}\ ?(AE|AH|S|ST|SY)\ ?([0-9]{2})([0-9]{2})[0-9]{2}$"#,
        move |c| {
            Ok(LcdScreen {
                year: Some(year2_u16(&c[2])?),
                month: Some(month2(&c[3])?),
            })
        },
    )
}

pub fn parse_lcd_screen(text: &str) -> Result<LcdScreen, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<LcdScreen>; 2] = [lcd_screen(), lcd_screen2()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
