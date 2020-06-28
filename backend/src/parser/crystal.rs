use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, MatcherDef, MatcherSet, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub month: Option<u8>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KDS1H").is_some());
/// ```
fn kds_short() -> MatcherDef<Crystal> {
    MatcherDef(r#"^KDS([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KDS 9803 4.194").is_some());
/// assert!(parse_crystal("KDS9807 4.194").is_some());
/// assert!(parse_crystal("KDSI 0549 4.194").is_some());
/// ```
fn kds_4194() -> MatcherDef<Crystal> {
    MatcherDef(r#"^KDSI?\ ?([0-9]{2})([0-9]{2})\ 4\.194$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year2(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KDS 9841 8.388").is_some());
/// ```
fn kds_8388() -> MatcherDef<Crystal> {
    MatcherDef(r#"^KDS\ ([0-9]{2})([0-9]{2})\ 8\.388$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year2(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}
/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KDS 6F 4.194").is_some());
/// ```
fn kds_4194_short() -> MatcherDef<Crystal> {
    MatcherDef(r#"^KDS\ ([0-9])([A-Z])\ 4\.194$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("D419A2").is_some());
/// assert!(parse_crystal("D419J3I").is_some());
/// ```
fn kds_d419() -> MatcherDef<Crystal> {
    MatcherDef(r#"^D419([A-Z])([0-9])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("D838K0I").is_some());
/// ```
fn kds_d838() -> MatcherDef<Crystal> {
    MatcherDef(r#"^D838([A-Z])([0-9])[A-Z]$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("D209A8").is_some());
/// ```
fn kds_d209() -> MatcherDef<Crystal> {
    MatcherDef(r#"^D209([A-Z])([0-9])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kds),
            year: Some(year1(&c[2])?),
            month: Some(kds_month(&c[1])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KSS20V 8A").is_some());
/// ```
fn kinseki_kss20() -> MatcherDef<Crystal> {
    MatcherDef(r#"^KSS20V\ ([0-9])([A-Z])$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("33WKSS6DT").is_some());
/// ```
fn kinseki_kss30() -> MatcherDef<Crystal> {
    MatcherDef(r#"^33WKSS([0-9])([A-Z])T$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("4194 KSS 0KF").is_some());
/// assert!(parse_crystal("4194 KSS1A").is_some());
/// ```
fn kinseki_4194() -> MatcherDef<Crystal> {
    MatcherDef(r#"^4194\ KSS\ ?([0-9])([A-Z])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("8388 KSS 1CF").is_some());
/// assert!(parse_crystal("8388 KSS 9J").is_some());
/// ```
fn kinseki_8388() -> MatcherDef<Crystal> {
    MatcherDef(r#"^8388\ KSS\ ([0-9])([A-Z])[A-Z]?$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("32K09").is_some());
/// assert!(parse_crystal("32K9Y").is_some());
/// assert!(parse_crystal("32K0Z").is_some());
/// ```
fn unknown() -> MatcherDef<Crystal> {
    MatcherDef(r#"^32K([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("4.19C59").is_some());
/// ```
fn unknown2() -> MatcherDef<Crystal> {
    MatcherDef(r#"^4\.19C([0-9])[[:alnum:]]$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year1(&c[1])?),
            month: None,
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("4.1943 9752").is_some());
/// ```
fn unknown_41943() -> MatcherDef<Crystal> {
    MatcherDef(r#"^4\.1943\ ([0-9]{2})([0-9]{2})$"#, move |c| {
        Ok(Crystal {
            manufacturer: None,
            year: Some(year2(&c[1])?),
            month: None,
            week: Some(week2(&c[2])?),
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("4.1943 RVR 841").is_some());
/// ```
fn unknown_41943_2() -> MatcherDef<Crystal> {
    MatcherDef(r#"^4\.1943\ RVR\ ([0-9])([0-9]{2})$"#, move |c| {
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

pub fn parse_crystal(text: &str) -> Option<Crystal> {
    lazy_static! {
        static ref MATCHER: MatcherSet<Crystal> = MatcherSet::new(&[
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
        ]);
    }
    MATCHER.apply(text)
}
