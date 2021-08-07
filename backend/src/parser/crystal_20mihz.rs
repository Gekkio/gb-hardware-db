use super::{kds_month, year1, Crystal, LabelParser, Manufacturer};
use crate::macros::{multi_parser, single_parser};

const FREQUENCY: u32 = 20_971_520;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_20mihz::kds_d209().parse("D209A8").is_ok());
/// ```
pub fn kds_d209() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^D209([A-Z])([0-9])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            frequency: FREQUENCY,
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal_20mihz::kinseki_kss20().parse("KSS20V 8A").is_ok());
/// ```
pub fn kinseki_kss20() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^KSS20V\ ([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            frequency: FREQUENCY,
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

pub fn crystal_20mihz() -> &'static impl LabelParser<Crystal> {
    multi_parser!(Crystal, kds_d209(), kinseki_kss20(),)
}
