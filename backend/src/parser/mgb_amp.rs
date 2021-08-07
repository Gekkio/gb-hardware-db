use super::{week2, year2, LabelParser, Year};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MgbAmp {
    pub kind: String,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mgb_amp::sharp_ir3r53n().parse("AMP MGB IR3R53N 9806 a").is_ok());
/// ```
pub fn sharp_ir3r53n() -> &'static impl LabelParser<MgbAmp> {
    single_parser!(
        MgbAmp,
        r#"^AMP\ MGB\ (IR3R53N)\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]$"#,
        move |c| {
            Ok(MgbAmp {
                kind: c[1].to_owned(),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mgb_amp::sharp_ir3r56n().parse("AMP MGB IR3R56N 0040 C").is_ok());
/// ```
pub fn sharp_ir3r56n() -> &'static impl LabelParser<MgbAmp> {
    single_parser!(
        MgbAmp,
        r#"^AMP\ MGB\ (IR3R56N)\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]$"#,
        move |c| {
            Ok(MgbAmp {
                kind: c[1].to_owned(),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

pub fn mgb_amp() -> &'static impl LabelParser<MgbAmp> {
    multi_parser!(MgbAmp, sharp_ir3r53n(), sharp_ir3r56n())
}
