use lazy_static::lazy_static;

use super::{week2, year2, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tama {
    pub tama_type: TamaType,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TamaType {
    Tama5,
    Tama6,
    Tama7,
}

/// TAMA5
///
/// ```
/// # use gbhwdb_backend::parser::parse_tama;
/// assert!(parse_tama("TAMA5 9726 EAD1").is_ok());
/// ```
fn tama5() -> Matcher<Tama> {
    Matcher::new(r#"^TAMA5\ ([0-9]{2})([0-9]{2})\ EA[A-Z]1$"#, move |c| {
        Ok(Tama {
            tama_type: TamaType::Tama5,
            year: Some(year2(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// TAMA6
///
/// ```
/// # use gbhwdb_backend::parser::parse_tama;
/// assert!(parse_tama("TAMA6 JAPAN 47C243M FV61 9751H").is_ok());
/// ```
fn tama6() -> Matcher<Tama> {
    Matcher::new(
        r#"^TAMA6\ JAPAN\ [[:alnum:]]{7}\ FV61\ ([0-9]{2})([0-9]{2})H$"#,
        move |c| {
            Ok(Tama {
                tama_type: TamaType::Tama6,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// TAMA7 ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_tama;
/// assert!(parse_tama("TAMA7 B9748 43913A TAIWAN").is_ok());
/// ```
fn tama7() -> Matcher<Tama> {
    Matcher::new(
        r#"^TAMA7\ [A-Z]([0-9]{2})([0-9]{2})\ [0-9]{5}[A-Z]\ TAIWAN$"#,
        move |c| {
            Ok(Tama {
                tama_type: TamaType::Tama7,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_tama(text: &str) -> Result<Tama, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Tama>; 3] = [tama5(), tama6(), tama7(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
