// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, year2, LabelParser, Manufacturer, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_code: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

/// Sharp ROM chip (1990+)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::sharp().parse("DMG-WJA-0 S LH534M05 JAPAN E1 9606 D").is_ok());
/// assert!(parser::mask_rom::sharp().parse("DMG-AP2J-0 S LH534MVD JAPAN E1 9639 D").is_ok());
/// assert!(parser::mask_rom::sharp().parse("DMG-HFAJ-0 S LHMN4MTI JAPAN E 9838 E").is_ok());
/// ```
pub fn sharp() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ S\ (LH[[:alnum:]]{4})[[:alnum:]]{2} \ JAPAN\ [A-Z][0-9]?\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(map_sharp_mask_rom(&c[3]).unwrap_or(&c[3]).to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Old sharp ROM chip with no chip type (1989 - 1991)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::sharp2().parse("DMG-TRA-1 SHARP JAPAN A0 9019 D").is_ok());
/// ```
pub fn sharp2() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
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
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::sharp3().parse("DMG-AWA-0 SHARP JAPAN 8909 D A").is_ok());
/// ```
pub fn sharp3() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
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
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::macronix().parse("M003119-M MX23C1603-12A DMG-VPHP-0 G2 2C882503").is_ok());
/// assert!(parser::mask_rom::macronix().parse("E013104-M MX23C1603-12A CGB-BFPU-0 G2 1D2907A1B1").is_ok());
/// assert!(parser::mask_rom::macronix().parse("T991349-M MX23C8006-12 DMG-VPHJ-0 F 1A4891A2").is_ok());
/// assert!(parser::mask_rom::macronix().parse("M004523-M MX23C3203-11A2 CGB-B82J-0 02 H2 2D224301").is_ok());
/// assert!(parser::mask_rom::macronix().parse("S013607-M MX23L6406-12B AGB-AWAP-0 I2 2E489301").is_ok())
/// ```
pub fn macronix() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^[A-Z]([0-9]{2})([0-9]{2})[0-9]{2}-M\ (MX23[CL][0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ([0-9]\ )? ((AGB|DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ ([0-9][0-9]\ )? [A-Z][0-9]?\ [[:alnum:]]{8,10}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[5].to_owned(),
                manufacturer: Some(Manufacturer::Macronix),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Macronix MX23C mask ROM chip (pre-1999)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::macronix2().parse("C9745-M MX23C4002-20 DMG-APOJ-0 E1 43824C").is_ok());
/// ```
pub fn macronix2() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^[A-Z]([0-9]{2})([0-9]{2})-M\ (MX23C[0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]?\ [[:alnum:]]{6}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[4].to_owned(),
                manufacturer: Some(Manufacturer::Macronix),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// OKI Semiconductor MSM538011E mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::oki_msm538011e().parse("DMG-AM6J-0 F1 M538011E-36 9085401").is_ok());
/// ```
pub fn oki_msm538011e() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (M538011E)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("MS{}", &c[3])),
                year: Some(year1(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// OKI Semiconductor MR531614G mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::oki_mr531614g().parse("CGB-BPTE-0 G2 R531614G-44 044232E").is_ok());
/// ```
pub fn oki_mr531614g() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (R531614G)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("M{}", &c[3])),
                year: Some(year1(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// NEC mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::nec().parse("NEC JAPAN DMG-SAJ-0 C1 UPD23C1001EGW-J01 9010E9702").is_ok());
/// ```
pub fn nec() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^NEC\ JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (UPD23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Nec),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Unknown mask ROM with NEC-like labeling
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::nec_like().parse("DMG-ZLE-0 E1 N-4001EAGW-J14 9329X7007").is_ok());
/// ```
pub fn nec_like() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (N-[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: None,
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// AT&T mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::at_t().parse("Ⓜ AT&T JAPAN DMG-Q6E-0 C1 23C1001EAGW-K37 9351E9005").is_ok());
/// ```
pub fn at_t() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^Ⓜ\ AT&T\ JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::AtT),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Standard Microsystems mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::smsc().parse("STANDARD MICRO DMG-BIA-0 C1 23C1001EGW-J61 9140E9017").is_ok());
/// ```
pub fn smsc() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^STANDARD\ MICRO\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Smsc),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// Glop top mask ROM.
///
/// Probably manufactured by Sharp (?)
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::glop_top().parse("LR0G150 DMG-TRA-1 97141").is_ok());
/// ```
pub fn glop_top() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
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
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::toshiba().parse("TOSHIBA 9136EAI TC531001CF DMG-NCE-0 C1 J541 JAPAN").is_ok());
/// ```
pub fn toshiba() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^TOSHIBA\ ([0-9]{2})([0-9]{2})EAI\ (TC53[0-9]{4}[A-Z]{2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [A-Z][0-9]{3}\ JAPAN$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[4].to_owned(),
                manufacturer: Some(Manufacturer::Toshiba),
                chip_type: (Some(c[3].to_owned())),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Samsung mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::samsung().parse("SEC KM23C16120DT CGB-BHMJ-0 G2 K3N5C317GD").is_ok());
/// ```
pub fn samsung() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^SEC\ (KM23C[0-9]{4,5}[A-Z]{1,2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [[:alnum:]]{10}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Samsung),
                chip_type: (Some(c[1].to_owned())),
                year: None,
                week: None,
            })
        },
    )
}

/// Old samsung mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::samsung2().parse("SEC KM23C8000DG DMG-AAUJ-1 F1 KFX331U").is_ok());
/// ```
pub fn samsung2() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^SEC\ (KM23C[0-9]{4,5}[A-Z]{1,2})\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ KF[[:alnum:]]{4}[A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Samsung),
                chip_type: (Some(c[1].to_owned())),
                year: None,
                week: None,
            })
        },
    )
}

/// Fujitsu Mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::fujitsu().parse("JAPAN DMG-GKX-0 D1 1P0 AK 9328 R09").is_ok());
/// assert!(parser::mask_rom::fujitsu().parse("JAPAN DMG-WJA-0 E1 3NH AK 9401 R17").is_ok());
/// ```
pub fn fujitsu() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^JAPAN\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [0-9][A-Z][[:alnum:]]\ [A-Z]{2}\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}$"#,
        move |c| {
            Ok(MaskRom {
                rom_code: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Fujitsu),
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

fn map_sharp_mask_rom(code: &str) -> Option<&'static str> {
    match code {
        "LH5359" => Some("LH53259"),   // Sharp Memory Data Book 1992
        "LH5317" => Some("LH53517"),   // Unknown mask ROM listing scan
        "LH531H" => Some("LH530800A"), // Sharp Memory Data Book 1992
        // reasonable guesses
        "LH5308" => Some("LH530800"), // unknown 1Mb JEDEC, compatible with LH530800A
        "LH5314" => Some("LH53514"),  // unknown 512Kb JEDEC, compatible with LH53517
        "LH5321" => Some("LH532100"), // unknown 2Mb JEDEC
        // unknown 2Mb JEDEC
        // maybe: LH532100 series / LH532300 / LH532700 series
        "LH532D" => None,
        "LH532M" => None,
        "LH532W" => None,
        "LHMN2E" => None,
        // Unknown 4Mb JEDEC
        // maybe: LH534100 series / LH534300 series / LH534R00
        "LH534M" => None,
        "LH5S4M" => None,
        "LHMN4M" => None,
        // Unknown 8Mb JEDEC
        // maybe: LH538300 series / LH538400 series / LH538700 / LH538R00 series
        "LH538M" => None,
        "LH538W" => None,
        "LH5S8M" => None,
        "LHMN8J" => None,
        "LHMN8M" => None,
        // Unknown 16 Mb
        // maybe: LH5316400 / LH5316500 series / LH5316P00 series
        "LH537M" => None,
        _ => None,
    }
}

pub fn mask_rom() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        sharp2(),
        sharp3(),
        macronix(),
        macronix2(),
        oki_msm538011e(),
        oki_mr531614g(),
        nec(),
        nec_like(),
        at_t(),
        smsc(),
        glop_top(),
        toshiba(),
        samsung(),
        samsung2(),
        fujitsu(),
    )
}
