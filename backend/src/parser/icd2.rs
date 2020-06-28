use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Icd2 {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_icd2;
/// assert!(parse_icd2("Nintendo ICD2-R 435 129").is_some());
/// ```
fn unknown() -> MatcherDef<Icd2> {
    MatcherDef(
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
/// assert!(parse_icd2("Nintendo ICD2-N 9415KX226 D93115").is_some());
/// ```
fn unknown2() -> MatcherDef<Icd2> {
    MatcherDef(
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

pub fn parse_icd2(text: &str) -> Option<Icd2> {
    lazy_static! {
        static ref MATCHER: MatcherSet<Icd2> = MatcherSet::new(&[unknown(), unknown2(),]);
    }
    MATCHER.apply(text)
}
