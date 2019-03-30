use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_code: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// Sharp ROM chip (1990+)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-WJA-0 S LH534M05 JAPAN E1 9606 D").is_ok());
/// ```
fn sharp() -> Matcher<MaskRom> {
    Matcher::new(r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ S\ (LH[[:alnum:]]{6})\ JAPAN\ [A-Z][0-9]?\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Sharp),
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[4])?),
            week: Some(week2(&c[5])?),
        })
    })
}

/// Old sharp ROM chip with no chip type (1989 - 1991)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-TRA-1 SHARP JAPAN A0 9019 D").is_ok());
/// ```
fn sharp2() -> Matcher<MaskRom> {
    Matcher::new(
        r#"^(DMG-[[:alnum:]]{3}-[0-9])\ SHARP\ JAPAN\ [A-Z][0-9]?\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: None,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Very old Sharp mask ROM chip (1989 and older)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-AWA-0 SHARP JAPAN 8909 D A").is_ok());
/// ```
fn sharp3() -> Matcher<MaskRom> {
    Matcher::new(
        r#"^(DMG-[[:alnum:]]{3}-[0-9])\ SHARP\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: None,
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Macronix MX23C mask ROM chip (1999+)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("M003119-M MX23C1603-12A DMG-VPHP-0 G2 2C882503").is_ok());
/// ```
fn macronix() -> Matcher<MaskRom> {
    Matcher::new(r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}-M\ (MX23C[0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ([0-9]\ )? ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ ([0-9][0-9]\ )? [A-Z][0-9]?\ [[:alnum:]]{8,10}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[5].to_owned(),
            manufacturer: Some(Manufacturer::Macronix),
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// Macronix MX23C mask ROM chip (pre-1999)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("C9745-M MX23C4002-20 DMG-APOJ-0 E1 43824C").is_ok());
/// ```
fn macronix2() -> Matcher<MaskRom> {
    Matcher::new(r#"^[A-Z]([0-9]{2})([0-9]{2})-M\ (MX23C[0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]?\ [[:alnum:]]{6}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[4].to_owned(),
            manufacturer: Some(Manufacturer::Macronix),
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// OKI Semiconductor M538011E mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-AM6J-0 F1 M538011E-36 9085401").is_ok());
/// ```
fn oki() -> Matcher<MaskRom> {
    Matcher::new(r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (M538011E-[[:alnum:]]{2})\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Oki),
            chip_type: Some(c[3].to_owned()),
            year: Some(year1(&c[4])?),
            week: Some(week2(&c[5])?),
        })
    })
}

/// NEC mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("NEC JAPAN DMG-SAJ-0 C1 UPD23C1001EGW-J01 9010E9702").is_ok());
/// ```
fn nec() -> Matcher<MaskRom> {
    Matcher::new(r#"^NEC\ JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (UPD23C[0-9]{4}[[:alnum:]]{3,4}-[A-Z][0-9]{2})\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Nec),
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[4])?),
            week: Some(week2(&c[5])?),
        })
    })
}

/// Unknown mask ROM with NEC-like labeling
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-ZLE-0 E1 N-4001EAGW-J14 9329X7007").is_ok());
/// ```
fn nec_like() -> Matcher<MaskRom> {
    Matcher::new(r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (N-[0-9]{4}[[:alnum:]]{3,4}-[A-Z][0-9]{2})\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[1].to_owned(),
            manufacturer: None,
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[4])?),
            week: Some(week2(&c[5])?),
        })
    })
}

/// AT&T mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("Ⓜ AT&T JAPAN DMG-Q6E-0 C1 23C1001EAGW-K37 9351E9005").is_ok());
/// ```
fn at_t() -> Matcher<MaskRom> {
    Matcher::new(r#"^Ⓜ\ AT&T\ JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (23C[0-9]{4}[[:alnum:]]{3,4}-[A-Z][0-9]{2})\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[1].to_owned(),
            manufacturer: Some(Manufacturer::AtT),
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[4])?),
            week: Some(week2(&c[5])?),
        })
    })
}

/// Standard Microsystems mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("STANDARD MICRO DMG-BIA-0 C1 23C1001EGW-J61 9140E9017").is_ok());
/// ```
fn smsc() -> Matcher<MaskRom> {
    Matcher::new(r#"^STANDARD\ MICRO\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (23C[0-9]{4}[[:alnum:]]{3,4}-[A-Z][0-9]{2})\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Smsc),
            chip_type: Some(c[3].to_owned()),
            year: Some(year2(&c[4])?),
            week: Some(week2(&c[5])?),
        })
    })
}

/// Glop top mask ROM.
///
/// Probably manufactured by Sharp (?)
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("LR0G150 DMG-TRA-1 97141").is_ok());
/// ```
fn glop_top() -> Matcher<MaskRom> {
    Matcher::new(
        r#"^(LR0G150)\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ ([0-9]{2})([0-9]{2})[0-9]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[2].to_owned(),
                manufacturer: None,
                chip_type: Some(c[1].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Toshiba mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("TOSHIBA 9136EAI TC531001CF DMG-NCE-0 C1 J541 JAPAN").is_ok());
/// ```
fn toshiba() -> Matcher<MaskRom> {
    Matcher::new(r#"^TOSHIBA\ ([0-9]{2})([0-9]{2})EAI\ (TC53[0-9]{4}[A-Z]{2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [A-Z][0-9]{3}\ JAPAN$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[4].to_owned(),
            manufacturer: Some(Manufacturer::Toshiba),
            chip_type: (Some(c[3].to_owned())),
            year: Some(year2(&c[1])?),
            week: Some(week2(&c[2])?),
        })
    })
}

/// Samsung mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("SEC KM23C16120DT CGB-BHMJ-0 G2 K3N5C317GD").is_ok());
/// ```
fn samsung() -> Matcher<MaskRom> {
    Matcher::new(r#"^SEC\ (KM23C[0-9]{4,5}[A-Z]{1,2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [[:alnum:]]{10}$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[2].to_owned(),
            manufacturer: Some(Manufacturer::Samsung),
            chip_type: (Some(c[1].to_owned())),
            year: None,
            week: None,
        })
    })
}

/// Old samsung mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("SEC KM23C8000DG DMG-AAUJ-1 F1 KFX331U").is_ok());
/// ```
fn samsung2() -> Matcher<MaskRom> {
    Matcher::new(r#"^SEC\ (KM23C[0-9]{4,5}[A-Z]{1,2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ KF[[:alnum:]]{4}[A-Z]$"#,
    move |c| {
        Ok(MaskRom {
            rom_code: c[2].to_owned(),
            manufacturer: Some(Manufacturer::Samsung),
            chip_type: (Some(c[1].to_owned())),
            year: None,
            week: None,
        })
    })
}

/// Fujitsu Mask ROM
///
/// ```
/// # use gbhwdb_backend::parser::parse_mask_rom;
/// assert!(parse_mask_rom("DMG-GKX-0 D1 1P0 AK 9328 R09").is_ok());
/// ```
fn fujitsu() -> Matcher<MaskRom> {
    Matcher::new(r#"^JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [0-9][A-Z][0-9]\ [A-Z]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}$"#,
                 move |c| {
                     Ok(MaskRom {
                         rom_code: c[1].to_owned(),
                         manufacturer: Some(Manufacturer::Fujitsu),
                         chip_type: None,
                         year: Some(year2(&c[3])?),
                         week: Some(week2(&c[4])?),
                     })
                 })
}

pub fn parse_mask_rom(text: &str) -> Result<MaskRom, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<MaskRom>; 15] = [
            sharp(),
            sharp2(),
            sharp3(),
            macronix(),
            macronix2(),
            oki(),
            nec(),
            nec_like(),
            at_t(),
            smsc(),
            glop_top(),
            toshiba(),
            samsung(),
            samsung2(),
            fujitsu(),
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
