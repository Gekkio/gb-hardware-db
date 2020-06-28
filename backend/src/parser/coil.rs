use lazy_static::lazy_static;

use super::{Manufacturer, MatcherDef, MatcherSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coil {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_coil;
/// assert!(parse_coil("TDK ZJY-M4A N").is_some());
/// ```
fn tdk() -> MatcherDef<Coil> {
    MatcherDef(r#"^TDK\ (ZJY-M4A)\ [A-Z]$"#, move |c| {
        Ok(Coil {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Tdk),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_coil;
/// assert!(parse_coil("TDK ZJY-M4PA n").is_some());
/// ```
fn tdk2() -> MatcherDef<Coil> {
    MatcherDef(r#"^TDK\ (ZJY-M4PA)\ [a-z]$"#, move |c| {
        Ok(Coil {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Tdk),
        })
    })
}

pub fn parse_coil(text: &str) -> Option<Coil> {
    lazy_static! {
        static ref MATCHER: MatcherSet<Coil> = MatcherSet::new(&[tdk(), tdk2()]);
    }
    MATCHER.apply(text)
}
