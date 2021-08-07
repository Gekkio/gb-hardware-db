use super::{kds_month, year1, Crystal, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

const FREQUENCY: u32 = 32_768;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_32kihz::kds_short().parse("KDS1H").is_ok());
/// ```
pub fn kds_short() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^KDS([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_32kihz::unknown().parse("32K09").is_ok());
/// assert!(parser::crystal_32kihz::unknown().parse("32K9Y").is_ok());
/// assert!(parser::crystal_32kihz::unknown().parse("32K0Z").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^32K([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

pub fn crystal_32kihz() -> &'static impl LabelParser<Crystal> {
    multi_parser!(Crystal, kds_short(), unknown(),)
}
