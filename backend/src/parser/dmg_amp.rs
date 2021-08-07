use super::{week2, year2, LabelParser, Year};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgAmp {
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::dmg_amp::sharp_ir3r40().parse("DMG-AMP IR3R40 9222 AA").is_ok());
/// assert!(parser::dmg_amp::sharp_ir3r40().parse("DMG-AMP IR3R40 8909 A").is_ok());
/// ```
pub fn sharp_ir3r40() -> &'static impl LabelParser<DmgAmp> {
    single_parser!(
        DmgAmp,
        r#"^DMG-AMP\ IR3R40\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(DmgAmp {
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn dmg_amp() -> &'static impl LabelParser<DmgAmp> {
    sharp_ir3r40()
}
