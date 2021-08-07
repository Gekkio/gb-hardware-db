use super::{year1, ChipYearWeek, LabelParser, Manufacturer};
use crate::macros::single_parser;

pub type LineDecoder = ChipYearWeek;

/// Toshiba TC7W139F
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::line_decoder::toshiba_tc7w139f().parse("7W139 0J").is_ok());
/// ```
pub fn toshiba_tc7w139f() -> &'static impl LabelParser<LineDecoder> {
    single_parser!(LineDecoder, r#"^(7W139F?)\ ([0-9])[A-Z]$"#, move |c| {
        Ok(LineDecoder {
            kind: (match &c[1] {
                "7W139" => Ok("TC7W139FU".to_owned()),
                "7W139F" => Ok("TC7W139F".to_owned()),
                text => Err(format!("Invalid Toshiba TC7W139F part name: {}", text)),
            })?,
            manufacturer: Some(Manufacturer::Toshiba),
            year: Some(year1(&c[2])?),
            week: None,
        })
    })
}

pub fn line_decoder() -> &'static impl LabelParser<LineDecoder> {
    toshiba_tc7w139f()
}
