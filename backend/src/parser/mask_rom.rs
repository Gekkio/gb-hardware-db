// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year2, LabelParser, Manufacturer, ParsedData, PartDateCode};
use crate::{
    macros::{multi_parser, single_parser},
    parser::{fujitsu::FUJITSU_MASK_ROM, macronix, nec, oki, toshiba},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_id: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub date_code: Option<PartDateCode>,
}

impl ParsedData for MaskRom {}

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
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: Some(map_sharp_mask_rom(&c[3]).unwrap_or(&c[3]).to_owned()),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[4])?,
                    week: week2(&c[5])?,
                }),
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
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
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
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                chip_type: None,
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[2])?,
                    week: week2(&c[3])?,
                }),
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
/// assert!(parser::mask_rom::sharp_glop_top_28().parse("LR0G150 DMG-TRA-1 97141").is_ok());
/// ```
pub fn sharp_glop_top_28() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^(LR0G150)\ (DMG-[[:alnum:]]{3}-[0-9])\ ([0-9]{2})([0-9]{2})[0-9]$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[2].to_owned(),
                manufacturer: None,
                chip_type: Some(c[1].to_owned()),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c[3])?,
                    week: week2(&c[4])?,
                }),
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
                rom_id: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Samsung),
                chip_type: (Some(c[1].to_owned())),
                date_code: None,
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
                rom_id: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Samsung),
                chip_type: (Some(c[1].to_owned())),
                date_code: None,
            })
        },
    )
}

/// Magnachip AC23V Mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::magnachip_ac23v().parse("MAGNACHIP AC23V128111 AGB-BPRE-1 J2 SP0730 PS").is_ok());
/// ```
pub fn magnachip_ac23v() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^MAGNACHIP\ (?<kind>AC23V[0-9]{6})\ (?<rom_id>AGB-[[:alnum:]]{4}-[0-9])\ [A-Z][0-9]\ SP(?<year>[0-9]{2})(?<week>[0-9]{2})\ PS$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c["rom_id"].to_owned(),
                manufacturer: Some(Manufacturer::Magnachip),
                chip_type: Some(c["kind"].to_owned()),
                date_code: Some(PartDateCode::YearWeek {
                    year: year2(&c["year"])?,
                    week: week2(&c["week"])?,
                }),
            })
        },
    )
}

/// Hynix AC23V Mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::hynix_ac23v().parse("HYNIX AC23V128111 AGB-AY7E-0 J2 NL0013").is_ok());
/// assert!(parser::mask_rom::hynix_ac23v().parse("HYNIX AC23V32101 AGB-BAUE-0 H2 ZBR4079").is_ok());
/// ```
pub fn hynix_ac23v() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^HYNIX\ (AC23V[0-9]{5,6})\ (AGB-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ [A-Z]{2,3}[0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[2].to_owned(),
                manufacturer: Some(Manufacturer::Hynix),
                chip_type: Some(c[1].to_owned()),
                date_code: None,
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
        // Unknown 32 Mb
        "LHMN5M" => None,
        _ => None,
    }
}

pub fn agb_mask_rom_tsop_ii_44() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        magnachip_ac23v(),
        hynix_ac23v(),
        &macronix::MACRONIX_MX23L3206,
        &macronix::MACRONIX_MX23L6406,
        &macronix::MACRONIX_MX23L6407,
        &macronix::MACRONIX_MX23L12806,
        &macronix::MACRONIX_MX23L12807,
        &macronix::MACRONIX_MX23L25607,
        &oki::OKI_MR26V3210,
        &oki::OKI_MR26V3211,
        &oki::OKI_MR26V6413,
        &oki::OKI_MR26V6414,
        &oki::OKI_MR26V6415,
        &oki::OKI_MR27V810,
        &oki::OKI_MR27V12813,
    )
}

pub fn mask_rom_glop_top_28() -> &'static impl LabelParser<MaskRom> {
    sharp_glop_top_28()
}

pub fn mask_rom_sop_32() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        sharp2(),
        sharp3(),
        &macronix::MACRONIX_MX23C4002,
        &macronix::MACRONIX_MX23C8003,
        &macronix::MACRONIX_MX23C8005,
        &oki::OKI_MSM534011,
        &oki::OKI_MSM538011,
        &nec::NEC_UPD23C1001E,
        &nec::NEC_UPD23C2001E,
        &nec::NEC_UPD23C4001E,
        &nec::NEC_UPD23C8001E,
        &nec::AT_T_UPD23C1001E,
        &nec::SMSC_UPD23C1001E,
        &nec::MANI_UPD23C4001E,
        &toshiba::TOSHIBA_TC531001,
        &toshiba::TOSHIBA_TC532000,
        &toshiba::TOSHIBA_TC534000,
        samsung(),
        samsung2(),
        &FUJITSU_MASK_ROM,
    )
}

pub fn mask_rom_tsop_i_32() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        &macronix::MACRONIX_MX23C8006,
        samsung(),
        samsung2(),
    )
}

pub fn mask_rom_tsop_ii_44_5v() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        sharp2(),
        sharp3(),
        &macronix::MACRONIX_MX23C1603,
        &macronix::MACRONIX_MX23C3203,
        &oki::OKI_MR531614,
        &nec::NEC_UPD23C16019W,
        samsung(),
        samsung2(),
    )
}

pub fn mask_rom_qfp_44() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        sharp2(),
        sharp3(),
        samsung(),
        samsung2(),
        &oki::OKI_OLD_MASK_ROM,
    )
}
