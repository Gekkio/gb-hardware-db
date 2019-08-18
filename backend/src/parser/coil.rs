use lazy_static::lazy_static;

use super::{Manufacturer, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coil {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_coil;
/// assert!(parse_coil("TDK ZJY-M4A N").is_ok());
/// ```
fn tdk() -> Matcher<Coil> {
    Matcher::new(r#"^TDK\ (ZJY-M4A)\ [A-Z]$"#, move |c| {
        Ok(Coil {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Tdk),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_coil;
/// assert!(parse_coil("TDK ZJY-M4PA n").is_ok());
/// ```
fn tdk2() -> Matcher<Coil> {
    Matcher::new(r#"^TDK\ (ZJY-M4PA)\ [a-z]$"#, move |c| {
        Ok(Coil {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Tdk),
        })
    })
}

pub fn parse_coil(text: &str) -> Result<Coil, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Coil>; 2] = [tdk(), tdk2()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
