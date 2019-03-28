use lazy_static::lazy_static;

use super::{week2, year1, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RamBackup {
    pub chip_type: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// Mitsubishi M62021P
///
/// ```
/// # use gbhwdb_backend::parser::parse_ram_backup;
/// assert!(parse_ram_backup("2021 7Z2").is_ok());
/// ```
pub fn mitsubishi_m62021p() -> Matcher<RamBackup> {
    Matcher::new(r#"^2021\ ([0-9])[[:alnum:]][0-9]$"#, move |c| {
        Ok(RamBackup {
            chip_type: "M62021P".to_owned(),
            manufacturer: Some(Manufacturer::Mitsubishi),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// Mitsumi MM1026A
///
/// ```
/// # use gbhwdb_backend::parser::parse_ram_backup;
/// assert!(parse_ram_backup("843 26A").is_ok());
/// ```
pub fn mitsumi_mm1026a() -> Matcher<RamBackup> {
    Matcher::new(r#"^([0-9])([[:alnum:]][0-9])\ 26A$"#, move |c| {
        Ok(RamBackup {
            chip_type: "MM1026A".to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// Mitsumi MM1134A
///
/// ```
/// # use gbhwdb_backend::parser::parse_ram_backup;
/// assert!(parse_ram_backup("939 134A").is_ok());
/// ```
pub fn mitsumi_mm1134a() -> Matcher<RamBackup> {
    Matcher::new(r#"^([0-9])([0-9]{2})\ 134A$"#, move |c| {
        Ok(RamBackup {
            chip_type: "MM1134A".to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
            year: Some(year1(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// ROHM BA6129
///
/// ```
/// # use gbhwdb_backend::parser::parse_ram_backup;
/// assert!(parse_ram_backup("6129 4803").is_ok());
/// ```
pub fn rohm_ba6129() -> Matcher<RamBackup> {
    Matcher::new(r#"^6129\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(RamBackup {
            chip_type: "BA6129".to_owned(),
            manufacturer: Some(Manufacturer::Rohm),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// ROHM BA6129A
///
/// ```
/// # use gbhwdb_backend::parser::parse_ram_backup;
/// assert!(parse_ram_backup("6129A 6194").is_ok());
/// ```
pub fn rohm_ba6129a() -> Matcher<RamBackup> {
    Matcher::new(r#"^6129A\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(RamBackup {
            chip_type: "BA6129A".to_owned(),
            manufacturer: Some(Manufacturer::Rohm),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

/// ROHM BA6735
///
/// ```
/// # use gbhwdb_backend::parser::parse_ram_backup;
/// assert!(parse_ram_backup("6735 8C19").is_ok());
/// ```
pub fn rohm_ba6735() -> Matcher<RamBackup> {
    Matcher::new(r#"^6735\ ([0-9])[[:alnum:]][0-9]{2}$"#, move |c| {
        Ok(RamBackup {
            chip_type: "BA6735".to_owned(),
            manufacturer: Some(Manufacturer::Rohm),
            year: Some(year1(&c[1])?),
            week: None,
        })
    })
}

pub fn parse_ram_backup(text: &str) -> Result<RamBackup, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<RamBackup>; 6] = [
            mitsumi_mm1026a(),
            mitsumi_mm1134a(),
            rohm_ba6129(),
            rohm_ba6129a(),
            rohm_ba6735(),
            mitsubishi_m62021p(),
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
