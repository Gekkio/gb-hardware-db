use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SgbRom {
    pub rom_code: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("SYS-SGB-2 © 1994 Nintendo 9429 R77").is_ok());
/// ```
fn unknown() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("SYS-SGB-2 © 1994 Nintendo 9423 E").is_ok());
/// ```
fn unknown2() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("SYS-SGB-2 JAPAN © 1994 Nintendo 427A2 A04 NND").is_ok());
/// ```
fn unknown3() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^(SYS-SGB-(NT|2))\ JAPAN\ ©\ 1994\ Nintendo\ [[:alnum:]]{5}\ [[:alnum:]]{3}\ [A-Z]{3}$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: None,
                year: None,
                week: None,
            })
        },
    )
}

/// Toshiba SGB ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("SYS-SGB-2 © 1994 Nintendo TC532000BF-N807 JAPAN 9431EAI").is_ok());
/// ```
fn toshiba() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^(SYS-SGB-(NT|2))\ ©\ 1994\ Nintendo\ (TC53[0-9]{4}[A-Z]{2})-[A-Z][0-9]{3}\ JAPAN\ ([0-9]{2})([0-9]{2})EAI$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Toshiba),
                chip_type: (Some(c[3].to_owned())),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Sharp SGB ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("SYS-SGB-2 © 1994 Nintendo LH532M0M 9432 E").is_ok());
/// ```
fn sharp_sgb() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^(SYS-SGB-NT|SYS-SGB-2)\ ©\ 1994\ Nintendo\ (LH[[:alnum:]]{4})[[:alnum:]]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(c[2].to_owned()),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Sharp SGB2 ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("© 1998 Nintendo SYS-SGB2-10 LH5S4RY4 0003 D").is_ok());
/// ```
fn sharp_sgb2() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^©\ 1998\ Nintendo\ (SYS-SGB2-10)\ (LH[[:alnum:]]{4})[[:alnum:]]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(c[2].to_owned()),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// OKI Semiconductor SGB/SGB2 ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_sgb_rom;
/// assert!(parse_sgb_rom("SYS-SGB2-10 © 1998 Nintendo M534011E-05 8012354").is_ok());
/// ```
fn oki() -> Matcher<SgbRom> {
    Matcher::new(
        r#"^(SYS-SGB-NT|SYS-SGB-2|SYS-SGB2-10)\ ©\ 1998\ Nintendo\ (M534011E)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(SgbRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(c[2].to_owned()),
                year: Some(year1(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

pub fn parse_sgb_rom(text: &str) -> Result<SgbRom, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<SgbRom>; 7] = [
            toshiba(),
            sharp_sgb(),
            sharp_sgb2(),
            oki(),
            unknown(),
            unknown2(),
            unknown3(),
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
