use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Icd2 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_icd2;
/// assert!(parse_icd2("Nintendo ICD2-R 435 129").is_ok());
/// ```
fn unknown() -> Matcher<Icd2> {
    Matcher::new(
        r#"^Nintendo\ (ICD2-[NR])\ ([0-9])([0-9]{2})\ [0-9]{3}$"#,
        move |c| {
            Ok(Icd2 {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_icd2;
/// assert!(parse_icd2("Nintendo ICD2-N 9415KX226 D93115").is_ok());
/// ```
fn unknown2() -> Matcher<Icd2> {
    Matcher::new(
        r#"^Nintendo\ (ICD2-[NR])\ ([0-9]{2})\ ?([0-9]{2})[A-Z]{2}[0-9]{3}\ (D93115|D93128)$"#,
        move |c| {
            Ok(Icd2 {
                kind: c[1].to_owned(),
                manufacturer: None,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

pub fn parse_icd2(text: &str) -> Result<Icd2, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Icd2>; 2] = [unknown(), unknown2(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
