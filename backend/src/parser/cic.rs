use super::{week2, year2_u16, LabelParser};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cic {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::cic::sharp_cic().parse("F411A © 1990 Nintendo 9428 a").is_ok());
/// ```
pub fn sharp_cic() -> &'static impl LabelParser<Cic> {
    single_parser!(
        Cic,
        r#"^(F411A|F411B|F413A|F413B)\ ©\ (1990|1992)\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Za-z]?$"#,
        move |c| {
            Ok(Cic {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        }
    )
}

pub fn cic() -> &'static impl LabelParser<Cic> {
    sharp_cic()
}
