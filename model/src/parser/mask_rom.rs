// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use super::{LabelParser, Manufacturer, ParsedData, PartDateCode};
use crate::{
    macros::{multi_parser, single_parser},
    parser::{fujitsu, macronix, nec, oki, samsung, sharp, toshiba},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_id: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub date_code: Option<PartDateCode>,
}

impl ParsedData for MaskRom {}

/// Magnachip AC23V Mask ROM
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::mask_rom::magnachip_ac23v().parse("MAGNACHIP AC23V128111 AGB-BPRE-1 J2 SP0730 PS").is_ok());
/// assert!(parser::mask_rom::magnachip_ac23v().parse("MAGNACHIP AC23V32101 AGB-BCRP-0 H2 GB1191 PS").is_ok());
/// ```
pub fn magnachip_ac23v() -> &'static impl LabelParser<MaskRom> {
    single_parser!(
        MaskRom,
        r#"^MAGNACHIP\ (?<kind>AC23V[0-9]{5,6})\ (?<rom_id>AGB-[[:alnum:]]{4}-[0-9])\ [A-Z][0-9]\ (SP|GB)([0-9]{2})([0-9]{2})\ PS$"#,
        move |c| {
            Ok(MaskRom {
                rom_id: c["rom_id"].to_owned(),
                manufacturer: Some(Manufacturer::Magnachip),
                chip_type: Some(c["kind"].to_owned()),
                date_code: None,
            })
        },
    )
}

/// Hynix AC23V Mask ROM
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
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

pub fn agb_mask_rom_tsop_ii_44_3v3() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        magnachip_ac23v(),
        hynix_ac23v(),
        &macronix::MACRONIX_MX23L8006,
        &macronix::MACRONIX_MX23L3206,
        &macronix::MACRONIX_MX23L3406,
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
        &oki::OKI_MR27V6416,
        &oki::OKI_MR27V12813,
    )
}

pub fn mask_rom_glop_top_28_5v() -> &'static impl LabelParser<MaskRom> {
    &sharp::SHARP_MASK_ROM_GLOP_TOP_28_256_KIBIT
}

pub fn mask_rom_sop_32_5v() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        &sharp::SHARP_MASK_ROM_SOP_32_1_MIBIT,
        &sharp::SHARP_LH53514Z,
        &sharp::SHARP_LH53517Z,
        &sharp::SHARP_LH530800N,
        &sharp::SHARP_LH532100N,
        &sharp::SHARP_LH532XXXN,
        &sharp::SHARP_LH534XXXN,
        &sharp::SHARP_LH538XXXN,
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
        &samsung::SAMSUNG_KM23C4000,
        &samsung::SAMSUNG_KM23C8000,
        &fujitsu::FUJITSU_MASK_ROM,
    )
}

pub fn mask_rom_sop_44_5v() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(MaskRom, &macronix::MACRONIX_MX23C1605,)
}

pub fn mask_rom_tsop_i_32_5v() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        &sharp::SHARP_LH534XXXS,
        &sharp::SHARP_LH538XXXS,
        &macronix::MACRONIX_MX23C8006,
    )
}

pub fn mask_rom_tsop_ii_44_5v() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        &sharp::SHARP_LH5316XXX,
        &sharp::SHARP_LH5332XXX,
        &macronix::MACRONIX_MX23C1603,
        &macronix::MACRONIX_MX23C3203,
        &oki::OKI_MR531614,
        &nec::NEC_UPD23C16019W,
        &samsung::SAMSUNG_KM23C16120,
    )
}

pub fn mask_rom_qfp_44_5v() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        &sharp::SHARP_LH53259M,
        &sharp::SHARP_LH53515M,
        &oki::OKI_OLD_MASK_ROM,
    )
}
