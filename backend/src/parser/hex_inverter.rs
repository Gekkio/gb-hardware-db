use once_cell::sync::OnceCell;

use super::{week2, year1, Manufacturer, Matcher, MatcherDef, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexInverter {
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_hex_inverter;
/// assert!(parse_hex_inverter("LVX 04 8 45").is_some());
/// ```
fn tc74lvx04ft() -> MatcherDef<HexInverter> {
    MatcherDef(r#"^LVX\ 04\ ([0-9])\ ([0-9]{2})$"#, move |c| {
        Ok(HexInverter {
            manufacturer: Some(Manufacturer::Toshiba),
            chip_type: Some("TC74LVX04FT".to_owned()),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

pub fn parse_hex_inverter(text: &str) -> Option<HexInverter> {
    static MATCHER: OnceCell<Matcher<HexInverter>> = OnceCell::new();
    MATCHER.get_or_init(|| tc74lvx04ft().into()).apply(text)
}
