use super::{week2, year1, year2, LabelParser, Manufacturer, Year};
use crate::macros::{multi_parser, single_parser};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub month: Option<u8>,
    pub week: Option<u8>,
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_short().parse("KDS1H").is_ok());
/// ```
pub fn kds_short() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^KDS([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_4194().parse("KDS 9803 4.194").is_ok());
/// assert!(parser::crystal::kds_4194().parse("KDS9807 4.194").is_ok());
/// assert!(parser::crystal::kds_4194().parse("KDSI 0549 4.194").is_ok());
/// ```
pub fn kds_4194() -> &'static impl LabelParser<Crystal> {
    single_parser!(
        Crystal,
        r#"^KDSI?\ ?([0-9]{2})([0-9]{2})\ 4\.194$"#,
        move |c| {
            Ok(Crystal {
                manufacturer: Some(Manufacturer::Kds),
                year: Some(year2(&c[1])?),
                month: None,
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_8388().parse("KDS 9841 8.388").is_ok());
/// ```
pub fn kds_8388() -> &'static impl LabelParser<Crystal> {
    single_parser!(
        Crystal,
        r#"^KDS\ ([0-9]{2})([0-9]{2})\ 8\.388$"#,
        move |c| {
            Ok(Crystal {
                manufacturer: Some(Manufacturer::Kds),
                year: Some(year2(&c[1])?),
                month: None,
                week: Some(week2(&c[2])?),
            })
        }
    )
}
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_4194_short().parse("KDS 6F 4.194").is_ok());
/// ```
pub fn kds_4194_short() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^KDS\ ([0-9])([A-Z])\ 4\.194$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_d419().parse("D419A2").is_ok());
/// assert!(parser::crystal::kds_d419().parse("D419J3I").is_ok());
/// ```
pub fn kds_d419() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^D419([A-Z])([0-9])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_d838().parse("D838K0I").is_ok());
/// ```
pub fn kds_d838() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^D838([A-Z])([0-9])[A-Z]$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kds_d209().parse("D209A8").is_ok());
/// ```
pub fn kds_d209() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^D209([A-Z])([0-9])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kinseki_kss20().parse("KSS20V 8A").is_ok());
/// ```
pub fn kinseki_kss20() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^KSS20V\ ([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kinseki_kss30().parse("33WKSS6DT").is_ok());
/// ```
pub fn kinseki_kss30() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^33WKSS([0-9])([A-Z])T$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kinseki_4194().parse("4194 KSS 0KF").is_ok());
/// assert!(parser::crystal::kinseki_4194().parse("4194 KSS1A").is_ok());
/// ```
pub fn kinseki_4194() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4194\ KSS\ ?([0-9])([A-Z])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::kinseki_8388().parse("8388 KSS 1CF").is_ok());
/// assert!(parser::crystal::kinseki_8388().parse("8388 KSS 9J").is_ok());
/// ```
pub fn kinseki_8388() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^8388\ KSS\ ([0-9])([A-Z])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::unknown().parse("32K09").is_ok());
/// assert!(parser::crystal::unknown().parse("32K9Y").is_ok());
/// assert!(parser::crystal::unknown().parse("32K0Z").is_ok());
/// ```
pub fn unknown() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^32K([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::unknown2().parse("4.19C59").is_ok());
/// ```
pub fn unknown2() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.19C([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::unknown_41943().parse("4.1943 9752").is_ok());
/// ```
pub fn unknown_41943() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.1943\ ([0-9]{2})([0-9]{2})$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year2(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::crystal::unknown_41943_2().parse("4.1943 RVR 841").is_ok());
/// ```
pub fn unknown_41943_2() -> &'static impl LabelParser<Crystal> {
    single_parser!(Crystal, r#"^4\.1943\ RVR\ ([0-9])([0-9]{2})$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year1(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

fn kds_month(text: &str) -> Result<u8, String> {
    match text {
        "A" => Ok(1),
        "B" => Ok(2),
        "C" => Ok(3),
        "D" => Ok(4),
        "E" => Ok(5),
        "F" => Ok(6),
        "G" => Ok(7),
        "H" => Ok(8),
        // I is intentionally skipped
        "J" => Ok(9),
        "K" => Ok(10),
        "L" => Ok(11),
        "M" => Ok(12),
        _ => Err(format!("Invalid 1-letter month: {}", text)),
    }
}

pub fn crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        kds_short(),
        unknown(),
        kds_d419(),
        unknown2(),
        kds_d209(),
        kinseki_kss20(),
        kinseki_4194(),
        kds_4194(),
        kds_4194_short(),
        unknown_41943(),
        unknown_41943_2(),
        kds_8388(),
        kinseki_8388(),
        kds_d838(),
        kinseki_kss30(),
    )
}
