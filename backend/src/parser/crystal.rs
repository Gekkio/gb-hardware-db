use lazy_static::lazy_static;

use super::{year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub month: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KDS1H").is_ok());
/// ```
fn kds_short() -> Matcher<Crystal> {
    Matcher::new(r#"^KDS([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("D419A2").is_ok());
/// ```
fn kds_d419() -> Matcher<Crystal> {
    Matcher::new(r#"^D419([A-Z])([0-9])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
        })
    })
}

fn unknown() -> Matcher<Crystal> {
    Matcher::new(r#"^32K9[A-Z]$"#, move |_| {
        Ok(Crystal {
            manufacturer: None,
            year: None,
            month: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("4.19C59").is_ok());
/// ```
fn unknown2() -> Matcher<Crystal> {
    Matcher::new(r#"^4.19C([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year1(&c[1])?),
            month: None,
        })
    })
}

fn kds_month(text: &str) -> Result<u8, String> {
    match text {
        "A" => Ok(1),
        "B" => Ok(2),
        "C" => Ok(3),
        "D" => Ok(4),
        "E" => Ok(5),
        "F" => Ok(6),
        "G" => Ok(7),
        "H" => Ok(8),
        "J" => Ok(9),
        // I is intentionally skipped
        "K" => Ok(10),
        "L" => Ok(11),
        "M" => Ok(12),
        _ => Err(format!("Invalid 1-letter month: {}", text)),
    }
}

pub fn parse_crystal(text: &str) -> Result<Crystal, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Crystal>; 4] =
            [kds_short(), unknown(), kds_d419(), unknown2()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
