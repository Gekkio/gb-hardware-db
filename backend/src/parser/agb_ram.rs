use lazy_static::lazy_static;

use super::{week2, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbRam {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// NEC D442012AGY
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("NEC JAPAN D442012AGY-BB85X-MJH 0037K7027").is_ok());
/// ```
fn nec() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^NEC\ JAPAN\ (D442012AGY-[A-Z]{2}[0-9]{2}X-MJH)\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(c[1].to_owned()),
                manufacturer: Some(Manufacturer::Nec),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Fujitsu MB82D12160
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("JAPAN 82D12160-10FN 0238 M88N").is_ok());
/// ```
fn fujitsu() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^JAPAN\ (82D12160-10FN)\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}[A-Z]$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(c[1].to_owned()),
                manufacturer: Some(Manufacturer::Fujitsu),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Hynix HY62LF16206A-LT12C
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("Hynix KOREA HY62LF16206A 0223A LT12C").is_ok());
/// ```
fn hynix() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^Hynix\ KOREA\ HY62LF16206A\ ([0-9]{2})([0-9]{2})A\ LT12C$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some("HY62LF16206A-LT12C".to_owned()),
                manufacturer: Some(Manufacturer::Hynix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_agb_ram(text: &str) -> Result<AgbRam, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<AgbRam>; 3] = [nec(), fujitsu(), hynix()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
