use super::{week2, year1, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

pub type AgsChargeController = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ags_charge_ctrl::mitsumi_mm1581a().parse("422 1581A").is_ok());
/// ```
pub fn mitsumi_mm1581a() -> &'static impl LabelParser<AgsChargeController> {
    single_parser!(
        AgsChargeController,
        r#"^([0-9])([0-9]{2})\ 1581A$"#,
        move |c| {
            Ok(AgsChargeController {
                kind: "MM1581A".to_owned(),
                manufacturer: Some(Manufacturer::Mitsumi),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::ags_charge_ctrl::unknown().parse("2253B 3129").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<AgsChargeController> {
    single_parser!(
        AgsChargeController,
        r#"^2253B\ ([0-9])([0-9]{2})[0-9]$"#,
        move |c| {
            Ok(AgsChargeController {
                kind: "2253B".to_owned(),
                manufacturer: None,
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

pub fn ags_charge_ctrl() -> &'static impl LabelParser<AgsChargeController> {
    multi_parser!(AgsChargeController, mitsumi_mm1581a(), unknown())
}
