use super::{week2, year1, year2, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

pub type AgbAmp = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_amp::sharp_ir3r60n().parse("AMP AGB IR3R60N 0103 a").is_ok());
/// ```
pub fn sharp_ir3r60n() -> &'static impl LabelParser<AgbAmp> {
    single_parser!(
        AgbAmp,
        r#"^AMP\ AGB\ IR3R60N\ ([A0-9]{2})([0-9]{2})\ [A-Za-z]$"#,
        move |c| {
            Ok(AgbAmp {
                kind: "IR3R60N".to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::agb_amp::rohm_bh7835afs().parse("BH7835AFS 337 T22").is_ok());
/// ```
pub fn rohm_bh7835afs() -> &'static impl LabelParser<AgbAmp> {
    single_parser!(
        AgbAmp,
        r#"^BH7835AFS\ ([0-9])([0-9]{2})\ [[:alnum:]][0-9]{2}$"#,
        move |c| {
            Ok(AgbAmp {
                kind: "BH7835AFS".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                year: Some(year1(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn agb_amp() -> &'static impl LabelParser<AgbAmp> {
    multi_parser!(AgbAmp, sharp_ir3r60n(), rohm_bh7835afs(),)
}
