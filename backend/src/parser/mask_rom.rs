// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{week2, year1, year2, LabelParser, Manufacturer, ParsedData, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_id: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub year: Option<Year>,
    pub week: Option<Week>,
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
                rom_id: c[1].to_owned(),
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
                rom_id: c[1].to_owned(),
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
/// assert!(parser::mask_rom::macronix_mx23c_new().parse("M003119-M MX23C1603-12A DMG-VPHP-0 G2 2C882503").is_ok());
/// assert!(parser::mask_rom::macronix_mx23c_new().parse("E013104-M MX23C1603-12A CGB-BFPU-0 G2 1D2907A1B1").is_ok());
/// assert!(parser::mask_rom::macronix_mx23c_new().parse("T991349-M MX23C8006-12 DMG-VPHJ-0 F 1A4891A2").is_ok());
/// assert!(parser::mask_rom::macronix_mx23c_new().parse("M004523-M MX23C3203-11A2 CGB-B82J-0 02 H2 2D224301").is_ok());
/// ```
pub fn macronix_mx23c_new() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^[aA-Z]([0-9]{2})([0-9]{2})[0-9]{2}-MG?\ (MX23C[0-9]{4,5}-[0-9]{2}[A-Z]?[0-9]?)\ ([0-9]\ )? ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ ([0-9][0-9]\ )? [A-Z][0-9]?\ [0-9][[:alnum:]]{7,9}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[5].to_owned(),
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
/// assert!(parser::mask_rom::macronix_mx23c_old().parse("C9745-M MX23C4002-20 DMG-APOJ-0 E1 43824C").is_ok());
/// ```
pub fn macronix_mx23c_old() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^[A-Z]([0-9]{2})([0-9]{2})-M\ (MX23C[0-9]{4}-[0-9]{2}[A-Z]?[0-9]?)\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]?\ [[:alnum:]]{6}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[4].to_owned(),
                manufacturer: Some(Manufacturer::Macronix),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Macronix MX23L mask ROM chip
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::macronix_mx23l().parse("S013607-M MX23L6406-12B AGB-AWAP-0 I2 2E489301").is_ok());
/// assert!(parser::mask_rom::macronix_mx23l().parse("B063953-MG MX23L25607-12D2 AGB-B24P-0 K2 2T016800").is_ok());
/// assert!(parser::mask_rom::macronix_mx23l().parse("a064553-MG MX23L25607-12D2 AGB-B24E-0 K2 2T536900").is_ok());
/// ```
pub fn macronix_mx23l() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^[aA-Z]([0-9]{2})([0-9]{2})[0-9]{2}-MG?\ (MX23L[0-9]{4,5}-[0-9]{2}[A-Z]?[0-9]?)\ ([0-9]\ )? (AGB-[[:alnum:]]{3,4}-[0-9])\ ([0-9][0-9]\ )? [A-Z][0-9]?\ [[:alnum:]]{8,10}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[5].to_owned(),
                manufacturer: Some(Manufacturer::Macronix),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Very old OKI mask ROM chip
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::oki_old().parse("DMG-QXA-0 OKI JAPAN B0 03 X0 02").is_ok());
/// ```
pub fn oki_old() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^(DMG-[[:alnum:]]{3}-[0-9])\ OKI\ JAPAN\ [[:alnum:]]{2}\ [0-9]{2}\ [[:alnum:]]{2}\ [0-9]{2}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: None,
                year: None,
                week: None,
            })
        },
    )
}

/// OKI Semiconductor MSM534011E / MSM538011E mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::oki_msm53x011e().parse("DMG-AM6J-0 F1 M538011E-36 9085401").is_ok());
/// assert!(parser::mask_rom::oki_msm53x011e().parse("CGB-ADME-0 E1 M534011E-09 841232A").is_ok());
/// ```
pub fn oki_msm53x011e() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ [A-Z][0-9]\ (M53[48]011E)-[[:alnum:]]{2}\ ([0-9])([0-9]{2})[0-9]{3}[[:alnum:]]$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
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
                rom_id: c[1].to_owned(),
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
                rom_id: c[1].to_owned(),
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
                rom_id: c[1].to_owned(),
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
                rom_id: c[1].to_owned(),
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
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Smsc),
                chip_type: Some(c[3].to_owned()),
                year: Some(year2(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

/// MANI mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::mani().parse("MANI DMG-MQE-2 23C4001EAGW-J22 9447X9200").is_ok());
/// ```
pub fn mani() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^MANI\ ((DMG|CGB)-[[:alnum:]]{3,4}-[0-9])\ (23C[0-9]{4}[[:alnum:]]{3,4})-[A-Z][0-9]{2}\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Mani),
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
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
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
                rom_id: c[4].to_owned(),
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
                rom_id: c[2].to_owned(),
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
                rom_id: c[2].to_owned(),
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
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Fujitsu),
                chip_type: None,
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// OKI MR26V TSOP-II-44 Mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::oki_mr26v().parse("AGB-TCHK-1 H2 R26V3210F-087 244A239").is_ok());
/// assert!(parser::mask_rom::oki_mr26v().parse("AGB-AXVJ-0 I2 R26V6414G-0A7 243A262").is_ok());
/// assert!(parser::mask_rom::oki_mr26v().parse("AGB-BR4J-0 I2 R26V6415G-02L 427ABA3").is_ok());
/// assert!(parser::mask_rom::oki_mr26v().parse("AGB-BR3P-0 H2 R26V3211F-0T6 442ABAJJ").is_ok());
/// ```
pub fn oki_mr26v() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^(AGB-[[:alnum:]]{4}-[0-9])\ [A-Z][0-9]\ (R26V[0-9]{4}[A-Z])-[0-9][[:alnum:]][[:alnum:]]\ ([0-9])([0-9]{2})[A-Z][[:alnum:]]{3}[A-Z]?$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("M{}", &c[2])),
                year: Some(year1(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// OKI MR27V TSOP-II-44 Mask ROM
///
/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::mask_rom::oki_mr27v().parse("AGB-AXPS-1 J2 R27V12813M-0C7 6145BARJ").is_ok());
/// assert!(parser::mask_rom::oki_mr27v().parse("AGB-FADP-0 F2 R27V810F-059 4475BB4J").is_ok());
/// assert!(parser::mask_rom::oki_mr27v().parse("AGB-U32P-0 J2 R27V12813M-0D2 5175204J").is_ok());
/// ```
pub fn oki_mr27v() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^(AGB-[[:alnum:]]{4}-[0-9])\ [A-Z][0-9]\ (R27V[0-9]{3,5}[A-Z])-[0-9][[:alnum:]][0-9]\ ([0-9])[0-9]{3}[[:alnum:]]{3}[A-Z]$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c[1].to_owned(),
                manufacturer: Some(Manufacturer::Oki),
                chip_type: Some(format!("M{}", &c[2])),
                year: Some(year1(&c[3])?),
                week: None,
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
                year: Some(year2(&c["year"])?),
                week: Some(week2(&c["week"])?),
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
                year: None,
                week: None,
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

pub fn agb_mask_rom_tsop_ii_44() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        magnachip_ac23v(),
        hynix_ac23v(),
        macronix_mx23l(),
        oki_mr26v(),
        oki_mr27v(),
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
        macronix_mx23c_new(),
        macronix_mx23c_old(),
        oki_msm53x011e(),
        oki_mr531614g(),
        nec(),
        nec_like(),
        at_t(),
        smsc(),
        mani(),
        toshiba(),
        samsung(),
        samsung2(),
        fujitsu(),
        oki_old(),
    )
}

pub fn mask_rom_tsop_i_32() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        macronix_mx23c_new(),
        oki_msm53x011e(),
        oki_mr531614g(),
        nec(),
        nec_like(),
        at_t(),
        smsc(),
        mani(),
        toshiba(),
        samsung(),
        samsung2(),
        fujitsu(),
        oki_old(),
    )
}

pub fn mask_rom_tsop_ii_44() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        sharp2(),
        sharp3(),
        macronix_mx23c_new(),
        oki_msm53x011e(),
        oki_mr531614g(),
        nec(),
        nec_like(),
        at_t(),
        smsc(),
        mani(),
        toshiba(),
        samsung(),
        samsung2(),
        fujitsu(),
        oki_old(),
    )
}

pub fn mask_rom_qfp_44() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        sharp(),
        sharp2(),
        sharp3(),
        oki_msm53x011e(),
        oki_mr531614g(),
        nec(),
        nec_like(),
        at_t(),
        smsc(),
        mani(),
        toshiba(),
        samsung(),
        samsung2(),
        fujitsu(),
        oki_old(),
    )
}
