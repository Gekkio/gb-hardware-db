use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub month: Option<u8>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("KDS1H").is_ok());
/// ```
fn kds_short() -> Matcher<Crystal> {
    Matcher::new(r#"^KDS([0-9])([A-Z])$"#, move |c| {
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
/// assert!(parse_crystal("KDS 9803 4.194").is_ok());
/// assert!(parse_crystal("KDS9807 4.194").is_ok());
/// ```
fn kds_4194() -> Matcher<Crystal> {
    Matcher::new(r#"^KDS\ ?([0-9]{2})([0-9]{2})\ 4\.194$"#, move |c| {
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
/// assert!(parse_crystal("KDS 9841 8.388").is_ok());
/// ```
fn kds_8388() -> Matcher<Crystal> {
    Matcher::new(r#"^KDS\ ([0-9]{2})([0-9]{2})\ 8\.388$"#, move |c| {
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
/// assert!(parse_crystal("KDS 6F 4.194").is_ok());
/// ```
fn kds_4194_short() -> Matcher<Crystal> {
    Matcher::new(r#"^KDS\ ([0-9])([A-Z])\ 4\.194$"#, move |c| {
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
/// assert!(parse_crystal("D419A2").is_ok());
/// ```
fn kds_d419() -> Matcher<Crystal> {
    Matcher::new(r#"^D419([A-Z])([0-9])$"#, move |c| {
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
/// assert!(parse_crystal("D838K0I").is_ok());
/// ```
fn kds_d838() -> Matcher<Crystal> {
    Matcher::new(r#"^D838([A-Z])([0-9])[A-Z]$"#, move |c| {
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
/// assert!(parse_crystal("D209A8").is_ok());
/// ```
fn kds_d209() -> Matcher<Crystal> {
    Matcher::new(r#"^D209([A-Z])([0-9])$"#, move |c| {
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
/// assert!(parse_crystal("KSS20V 8A").is_ok());
/// ```
fn kinseki_kss20() -> Matcher<Crystal> {
    Matcher::new(r#"^KSS20V\ ([0-9])([A-Z])$"#, move |c| {
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
/// assert!(parse_crystal("4194 KSS 0KF").is_ok());
/// ```
fn kinseki_4194() -> Matcher<Crystal> {
    Matcher::new(r#"^4194\ KSS\ ([0-9])([A-Z])[A-Z]$"#, move |c| {
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
/// assert!(parse_crystal("8388 KSS 1CF").is_ok());
/// ```
fn kinseki_8388() -> Matcher<Crystal> {
    Matcher::new(r#"^8388\ KSS\ ([0-9])([A-Z])[A-Z]$"#, move |c| {
        Ok(Crystal {
            manufacturer: Some(Manufacturer::Kinseki),
            year: Some(year1(&c[1])?),
            month: Some(kds_month(&c[2])?),
            week: None,
        })
    })
}

fn unknown() -> Matcher<Crystal> {
    Matcher::new(r#"^32K9[A-Z]$"#, move |_| {
        Ok(Crystal {
            manufacturer: None,
            year: None,
            month: None,
            week: None,
        })
    })
}

/// ```
/// # use gbhwdb_backend::parser::parse_crystal;
/// assert!(parse_crystal("4.19C59").is_ok());
/// ```
fn unknown2() -> Matcher<Crystal> {
    Matcher::new(r#"^4\.19C([0-9])[[:alnum:]]$"#, move |c| {
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
/// assert!(parse_crystal("4.1943 9752").is_ok());
/// ```
fn unknown_41943() -> Matcher<Crystal> {
    Matcher::new(r#"^4\.1943\ ([0-9]{2})([0-9]{2})$"#, move |c| {
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
/// assert!(parse_crystal("4.1943 RVR 841").is_ok());
/// ```
fn unknown_41943_2() -> Matcher<Crystal> {
    Matcher::new(r#"^4\.1943\ RVR\ ([0-9])([0-9]{2})$"#, move |c| {
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

pub fn parse_crystal(text: &str) -> Result<Crystal, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Crystal>; 14] = [
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
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
