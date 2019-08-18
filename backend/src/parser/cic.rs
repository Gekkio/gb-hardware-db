use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cic {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cic;
/// assert!(parse_cic("F411A © 1990 Nintendo 9428 a").is_ok());
/// ```
fn cic() -> Matcher<Cic> {
    Matcher::new(
        r#"^(F411A|F413A|F413B)\ ©\ (1990|1992)\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Za-z]?$"#,
        move |c| {
            Ok(Cic {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

pub fn parse_cic(text: &str) -> Result<Cic, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Cic>; 1] = [cic()];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
