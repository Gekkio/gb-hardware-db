use once_cell::sync::OnceCell;

use super::{year1, Manufacturer, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineDecoder {
    pub chip_type: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
}

/// Toshiba TC7W139F
///
/// ```
/// # use gbhwdb_backend::parser::parse_line_decoder;
/// assert!(parse_line_decoder("7W139 0J").is_some());
/// ```
fn toshiba_tc7w139f() -> MatcherDef<LineDecoder> {
    MatcherDef(r#"^(7W139F?)\ ([0-9])[A-Z]$"#, move |c| {
        Ok(LineDecoder {
            chip_type: Some(
                (match &c[1] {
                    "7W139" => Ok("TC7W139FU".to_owned()),
                    "7W139F" => Ok("TC7W139F".to_owned()),
                    text => Err(format!("Invalid Toshiba TC7W139F part name: {}", text)),
                })?,
            ),
            manufacturer: Some(Manufacturer::Toshiba),
            year: Some(year1(&c[2])?),
        })
    })
}

pub fn parse_line_decoder(text: &str) -> Option<LineDecoder> {
    static MATCHER: OnceCell<Matcher<LineDecoder>> = OnceCell::new();
    MATCHER
        .get_or_init(|| toshiba_tc7w139f().into())
        .apply(text)
}
