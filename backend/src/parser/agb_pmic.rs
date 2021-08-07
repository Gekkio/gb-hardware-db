use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbPmic {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_pmic::mitsumi_mm1514x().parse("105 514X").is_ok());
/// ```
pub fn mitsumi_mm1514x() -> &'static impl LabelParser<AgbPmic> {
    single_parser!(AgbPmic, r#"^([0-9])([0-5][0-9])\ 514X$"#, move |c| {
        Ok(AgbPmic {
            kind: Some("MM1514X".to_owned()),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_pmic::mitsumi_mm1514x_2().parse("081 514X").is_ok());
/// ```
pub fn mitsumi_mm1514x_2() -> &'static impl LabelParser<AgbPmic> {
    single_parser!(AgbPmic, r#"^([0-9])[0-9]{2}\ 514X$"#, move |c| {
        Ok(AgbPmic {
            kind: Some("MM1514X".to_owned()),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_pmic::unknown().parse("S6960 E-U2Z C700").is_ok());
/// assert!(parser::agb_pmic::unknown().parse("S6960 E-U2X C410").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<AgbPmic> {
    single_parser!(
        AgbPmic,
        r#"^S6960\ E-U([0-9])[A-Z]\ C[0-9]{3}$"#,
        move |c| {
            Ok(AgbPmic {
                kind: Some("S6960".to_owned()),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_pmic::unknown2().parse("9750A 1581").is_ok());
/// assert!(parser::agb_pmic::unknown2().parse("9750B 2A69").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<AgbPmic> {
    single_parser!(
        AgbPmic,
        r#"^(9750[AB])\ ([0-9])[[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(AgbPmic {
                kind: Some(c[1].to_owned()),
                manufacturer: None,
                year: Some(year1(&c[2])?),
                week: None,
            })
        }
    )
}

pub fn agb_pmic() -> &'static impl LabelParser<AgbPmic> {
    multi_parser!(
        AgbPmic,
        mitsumi_mm1514x(),
        mitsumi_mm1514x_2(),
        unknown(),
        unknown2()
    )
}
