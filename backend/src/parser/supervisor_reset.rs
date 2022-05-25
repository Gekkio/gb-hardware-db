use super::{week2, year1, LabelParser, Manufacturer, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisorReset {
    pub chip_type: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

/// Mitsubishi M62021P
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::supervisor_reset::mitsubishi_m62021p().parse("2021 7Z2").is_ok());
/// ```
pub fn mitsubishi_m62021p() -> &'static impl LabelParser<SupervisorReset> {
    single_parser!(
        SupervisorReset,
        r#"^2021\ ([0-9])[[:alnum:]][0-9]$"#,
        move |c| {
            Ok(SupervisorReset {
                chip_type: "M62021P".to_owned(),
                manufacturer: Some(Manufacturer::Mitsubishi),
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}

/// Mitsumi MM1026A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::supervisor_reset::mitsumi_mm1026a().parse("843 26A").is_ok());
/// assert!(parser::supervisor_reset::mitsumi_mm1026a().parse("1L51 26A").is_ok());
/// ```
pub fn mitsumi_mm1026a() -> &'static impl LabelParser<SupervisorReset> {
    single_parser!(
        SupervisorReset,
        r#"^([0-9])([[:alnum:]][0-9]{1,2})\ 26A$"#,
        move |c| {
            Ok(SupervisorReset {
                chip_type: "MM1026A".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}

/// Mitsumi MM1134A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::supervisor_reset::mitsumi_mm1134a().parse("939 134A").is_ok());
/// ```
pub fn mitsumi_mm1134a() -> &'static impl LabelParser<SupervisorReset> {
    single_parser!(SupervisorReset, r#"^([0-9])([0-9]{2})\ 134A$"#, move |c| {
        Ok(SupervisorReset {
            chip_type: "MM1134A".to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// ROHM BA6129
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::supervisor_reset::rohm_ba6129().parse("6129 4803").is_ok());
/// ```
pub fn rohm_ba6129() -> &'static impl LabelParser<SupervisorReset> {
    single_parser!(
        SupervisorReset,
        r#"^6129\ ([0-9])[[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(SupervisorReset {
                chip_type: "BA6129".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}

/// ROHM BA6129A
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::supervisor_reset::rohm_ba6129a().parse("6129A 6194").is_ok());
/// ```
pub fn rohm_ba6129a() -> &'static impl LabelParser<SupervisorReset> {
    single_parser!(
        SupervisorReset,
        r#"^6129A\ ([0-9])[[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(SupervisorReset {
                chip_type: "BA6129A".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}

/// ROHM BA6735
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::supervisor_reset::rohm_ba6735().parse("6735 8C19").is_ok());
/// ```
pub fn rohm_ba6735() -> &'static impl LabelParser<SupervisorReset> {
    single_parser!(
        SupervisorReset,
        r#"^6735\ ([0-9])[[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(SupervisorReset {
                chip_type: "BA6735".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: None,
            })
        }
    )
}

pub fn supervisor_reset() -> &'static impl LabelParser<SupervisorReset> {
    multi_parser!(
        SupervisorReset,
        mitsumi_mm1026a(),
        mitsumi_mm1134a(),
        rohm_ba6129(),
        rohm_ba6129a(),
        rohm_ba6735(),
        mitsubishi_m62021p(),
    )
}
