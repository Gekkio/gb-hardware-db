// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use log::warn;
use nom::bytes::streaming::tag;
use nom::sequence::separated_pair;
use nom::{IResult, Parser as _, combinator::all_consuming};
use nom_language::error::VerboseError;
use serde::{Deserialize, Serialize};
use stamp::{CgbStamp, DmgStamp};

use crate::parser::for_nom::{month2, year2};
use crate::{
    macros::multi_parser,
    time::{Month, Week},
};

pub mod amic;
pub mod analog;
pub mod atmel;
pub mod bsi;
pub mod crosslink;
pub mod fujitsu;
pub mod hudson;
pub mod hynix;
pub mod hyundai;
pub mod kds;
pub mod kinseki;
pub mod lgs;
pub mod lsi_logic;
pub mod macronix;
pub mod magnachip;
pub mod mitsubishi;
pub mod mitsumi;
pub mod mosel_vitelic;
pub mod motorola;
pub mod nec;
pub mod oki;
pub mod panasonic;
pub mod rohm;
pub mod samsung;
pub mod sanyo;
pub mod seiko;
pub mod sharp;
pub mod sst;
pub mod st_micro;
pub mod stamp;
pub mod tdk;
pub mod ti;
pub mod toshiba;
pub mod unknown;
pub mod victronix;
pub mod winbond;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PartDateCode {
    Year { year: Year },
    YearMonth { year: Year, month: Month },
    YearWeek { year: Year, week: Week },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericPart {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub date_code: Option<PartDateCode>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub frequency: u32,
    pub date_code: Option<PartDateCode>,
}

impl Crystal {
    pub const FREQ_32_KIHZ: u32 = 32_768;
    pub const FREQ_4_MIHZ: u32 = 4_194_304;
    pub const FREQ_8_MIHZ: u32 = 8_388_608;
    pub const FREQ_20_MIHZ: u32 = 20_971_520;
    pub const FREQ_32_MIHZ: u32 = 33_554_432;
    pub fn format_frequency(&self) -> String {
        if self.frequency > 1_000_000 {
            format!(
                "{}.{} MHz",
                self.frequency / 1_000_000,
                self.frequency % 1_000_000
            )
        } else if self.frequency > 1_000 {
            format!("{}.{} kHz", self.frequency / 1_000, self.frequency % 1_000)
        } else {
            format!("{} Hz", self.frequency)
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Deserialize,
    Serialize,
    strum::VariantArray,
    strum::IntoStaticStr,
    strum::EnumString,
)]
pub enum Manufacturer {
    Amic,
    Analog,
    Atmel,
    AtT,
    Bsi,
    Crosslink,
    Fujitsu,
    Hudson,
    Hynix,
    Hyundai,
    Kds,
    Lgs,
    LsiLogic,
    Kinseki,
    Macronix,
    Magnachip,
    Mani,
    Maxell,
    Mitsubishi,
    Mitsumi,
    MoselVitelic,
    Motorola,
    Nec,
    Oki,
    Panasonic,
    Rohm,
    Samsung,
    Sanyo,
    Seiko,
    Sharp,
    Smsc,
    Sony,
    Sst,
    StMicro,
    Tdk,
    TexasInstruments,
    Toshiba,
    Victronix,
    Winbond,
}

impl Manufacturer {
    pub const fn name(&self) -> &'static str {
        match self {
            Manufacturer::Amic => "AMIC Technology",
            Manufacturer::Analog => "Analog Devices",
            Manufacturer::Atmel => "Atmel",
            Manufacturer::AtT => "AT&T Technologies",
            Manufacturer::Bsi => "BSI",
            Manufacturer::Crosslink => "Crosslink Semiconductor",
            Manufacturer::Fujitsu => "Fujitsu",
            Manufacturer::Hudson => "Hudson",
            Manufacturer::Hynix => "Hynix",
            Manufacturer::Hyundai => "Hyundai",
            Manufacturer::Kds => "Daishinku",
            Manufacturer::Kinseki => "Kinseki",
            Manufacturer::Lgs => "Lucky GoldStar",
            Manufacturer::LsiLogic => "LSI Logic",
            Manufacturer::Macronix => "Macronix",
            Manufacturer::Magnachip => "Magnachip",
            Manufacturer::Mani => "Mani Ltd.",
            Manufacturer::Maxell => "Maxell",
            Manufacturer::Mitsubishi => "Mitsubishi",
            Manufacturer::Mitsumi => "Mitsumi",
            Manufacturer::MoselVitelic => "Mosel-Vitelic",
            Manufacturer::Motorola => "Motorola",
            Manufacturer::Nec => "NEC",
            Manufacturer::Oki => "OKI",
            Manufacturer::Panasonic => "Panasonic",
            Manufacturer::Rohm => "ROHM",
            Manufacturer::Samsung => "Samsung",
            Manufacturer::Sanyo => "Sanyo",
            Manufacturer::Seiko => "Seiko Instruments Inc.",
            Manufacturer::Sharp => "Sharp",
            Manufacturer::Smsc => "Standard Microsystems Corporation",
            Manufacturer::Sony => "Sony",
            Manufacturer::Sst => "SST",
            Manufacturer::StMicro => "STMicroelectronics",
            Manufacturer::Tdk => "TDK",
            Manufacturer::TexasInstruments => "Texas Instruments",
            Manufacturer::Toshiba => "Toshiba",
            Manufacturer::Victronix => "Victronix",
            Manufacturer::Winbond => "Winbond",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Year {
    Full(u16),
    Partial(u8),
}

mod for_nom {
    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::streaming::{tag, take},
        character::streaming::{anychar, char, satisfy},
        combinator::{map_opt, recognize},
        error::ParseError,
        multi::fold_many_m_n,
    };

    use super::{PartDateCode, Year};
    use crate::time::{Month, Week};

    pub fn dmg_rom_code<'a, E: ParseError<&'a str>>()
    -> impl Parser<&'a str, Output = &'a str, Error = E> {
        recognize((
            tag("DMG-"),
            satisfy_m_n(3, 4, |c| c.is_ascii_digit() || c.is_ascii_uppercase()),
            char('-'),
            digits(1),
        ))
    }

    pub fn cgb_rom_code<'a, E: ParseError<&'a str>>()
    -> impl Parser<&'a str, Output = &'a str, Error = E> {
        recognize((tag("CGB-"), alnum_uppers(4), char('-'), digits(1)))
    }

    pub fn agb_rom_code<'a, E: ParseError<&'a str>>()
    -> impl Parser<&'a str, Output = &'a str, Error = E> {
        recognize((tag("AGB-"), alnum_uppers(4), char('-'), digits(1)))
    }

    pub fn satisfy_m_n<'a, E: ParseError<&'a str>>(
        min: usize,
        max: usize,
        f: impl Fn(char) -> bool,
    ) -> impl Parser<&'a str, Output = &'a str, Error = E> {
        recognize(fold_many_m_n(min, max, satisfy(f), || (), |_, _| ()))
    }

    pub fn satisfy_m_n_complete<'a, E: ParseError<&'a str>>(
        min: usize,
        max: usize,
        f: impl Fn(char) -> bool,
    ) -> impl Parser<&'a str, Output = &'a str, Error = E> {
        recognize(fold_many_m_n(
            min,
            max,
            nom::character::complete::satisfy(f),
            || (),
            |_, _| (),
        ))
    }

    pub fn alnum_uppers<'a, E: ParseError<&'a str>>(
        count: usize,
    ) -> impl Parser<&'a str, Output = &'a str, Error = E> {
        satisfy_m_n(count, count, |c| {
            c.is_ascii_digit() || c.is_ascii_uppercase()
        })
    }

    pub fn uppers<'a, E: ParseError<&'a str>>(
        count: usize,
    ) -> impl Parser<&'a str, Output = &'a str, Error = E> {
        satisfy_m_n(count, count, |c| c.is_ascii_uppercase())
    }

    pub fn alphas<'a, E: ParseError<&'a str>>(
        count: usize,
    ) -> impl Parser<&'a str, Output = &'a str, Error = E> {
        satisfy_m_n(count, count, |c| c.is_ascii_alphabetic())
    }

    pub fn digits<'a, E: ParseError<&'a str>>(
        count: usize,
    ) -> impl Parser<&'a str, Output = &'a str, Error = E> {
        satisfy_m_n(count, count, |c| c.is_ascii_digit())
    }

    pub fn year1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Year, E> {
        map_opt(take(1_usize), |text| match u8::from_str_radix(text, 10) {
            Ok(value) => Some(Year::Partial(value)),
            _ => None,
        })
        .parse(input)
    }

    pub fn year2<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Year, E> {
        map_opt(take(2_usize), |text| match u16::from_str_radix(text, 10) {
            Ok(value @ 0..=87) => Some(Year::Full(value + 2000)),
            Ok(value @ 88..=99) => Some(Year::Full(value + 1900)),
            _ => None,
        })
        .parse(input)
    }

    pub fn year1_week2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        (year1, week2)
            .map(|(year, week)| PartDateCode::YearWeek { year, week })
            .parse(input)
    }

    pub fn year1_month2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        (year1, month2)
            .map(|(year, month)| PartDateCode::YearMonth { year, month })
            .parse(input)
    }

    pub fn year2_week2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        (year2, week2)
            .map(|(year, week)| PartDateCode::YearWeek { year, week })
            .parse(input)
    }

    pub fn year2_month2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        (year2, month2)
            .map(|(year, month)| PartDateCode::YearMonth { year, month })
            .parse(input)
    }

    pub fn year1_month1_123abc<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        (year1, month1_123abc)
            .map(|(year, month)| PartDateCode::YearMonth { year, month })
            .parse(input)
    }

    pub fn year1_month1_abc<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        (year1, month1_abc)
            .map(|(year, month)| PartDateCode::YearMonth { year, month })
            .parse(input)
    }

    pub fn month1_123abc<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Month, E> {
        map_opt(take(1_usize), |text| match text {
            "1" => Some(Month::January),
            "2" => Some(Month::February),
            "3" => Some(Month::March),
            "4" => Some(Month::April),
            "5" => Some(Month::May),
            "6" => Some(Month::June),
            "7" => Some(Month::July),
            "8" => Some(Month::August),
            "9" => Some(Month::September),
            "A" => Some(Month::October),
            "B" => Some(Month::November),
            "C" => Some(Month::December),
            _ => None,
        })
        .parse(input)
    }

    pub fn month1_123xyz<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Month, E> {
        map_opt(anychar, |ch| match ch {
            '1' => Some(Month::January),
            '2' => Some(Month::February),
            '3' => Some(Month::March),
            '4' => Some(Month::April),
            '5' => Some(Month::May),
            '6' => Some(Month::June),
            '7' => Some(Month::July),
            '8' => Some(Month::August),
            '9' => Some(Month::September),
            'X' => Some(Month::October),
            'Y' => Some(Month::November),
            'Z' => Some(Month::December),
            _ => None,
        })
        .parse(input)
    }

    pub fn month1_abc<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Month, E> {
        map_opt(anychar, |text| match text {
            'A' => Some(Month::January),
            'B' => Some(Month::February),
            'C' => Some(Month::March),
            'D' => Some(Month::April),
            'E' => Some(Month::May),
            'F' => Some(Month::June),
            'G' => Some(Month::July),
            'H' => Some(Month::August),
            // I is intentionally skipped
            'J' => Some(Month::September),
            'K' => Some(Month::October),
            'L' => Some(Month::November),
            'M' => Some(Month::December),
            _ => None,
        })
        .parse(input)
    }

    pub fn month1_123ond<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Month, E> {
        map_opt(take(1_usize), |text| match text {
            "1" => Some(Month::January),
            "2" => Some(Month::February),
            "3" => Some(Month::March),
            "4" => Some(Month::April),
            "5" => Some(Month::May),
            "6" => Some(Month::June),
            "7" => Some(Month::July),
            "8" => Some(Month::August),
            "9" => Some(Month::September),
            "O" => Some(Month::October),
            "0" => Some(Month::October), // backwards-compatibility
            "N" => Some(Month::November),
            "D" => Some(Month::December),
            _ => None,
        })
        .parse(input)
    }

    pub fn week2<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Week, E> {
        map_opt(take(2_usize), |text| {
            u8::from_str_radix(text, 10)
                .ok()
                .and_then(|v| Week::try_from(v).ok())
        })
        .parse(input)
    }

    pub fn month2<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Month, E> {
        map_opt(take(2_usize), |text| {
            u8::from_str_radix(text, 10)
                .ok()
                .and_then(|v| Month::try_from(v).ok())
        })
        .parse(input)
    }

    fn line_sep<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, char, E> {
        alt((char(' '), char('\n'))).parse(input)
    }

    pub fn lines2<'a, O1, O2, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, Output = O1, Error = E>,
        b: impl Parser<&'a str, Output = O2, Error = E>,
    ) -> impl Parser<&'a str, Output = (O1, O2), Error = E> {
        (a, line_sep, b).map(|(a, _, b)| (a, b))
    }

    pub fn lines3<'a, O1, O2, O3, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, Output = O1, Error = E>,
        b: impl Parser<&'a str, Output = O2, Error = E>,
        c: impl Parser<&'a str, Output = O3, Error = E>,
    ) -> impl Parser<&'a str, Output = (O1, O2, O3), Error = E> {
        (a, line_sep, b, line_sep, c).map(|(a, _, b, _, c)| (a, b, c))
    }

    pub fn lines4<'a, O1, O2, O3, O4, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, Output = O1, Error = E>,
        b: impl Parser<&'a str, Output = O2, Error = E>,
        c: impl Parser<&'a str, Output = O3, Error = E>,
        d: impl Parser<&'a str, Output = O4, Error = E>,
    ) -> impl Parser<&'a str, Output = (O1, O2, O3, O4), Error = E> {
        (a, line_sep, b, line_sep, c, line_sep, d).map(|(a, _, b, _, c, _, d)| (a, b, c, d))
    }

    pub fn lines5<'a, O1, O2, O3, O4, O5, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, Output = O1, Error = E>,
        b: impl Parser<&'a str, Output = O2, Error = E>,
        c: impl Parser<&'a str, Output = O3, Error = E>,
        d: impl Parser<&'a str, Output = O4, Error = E>,
        e: impl Parser<&'a str, Output = O5, Error = E>,
    ) -> impl Parser<&'a str, Output = (O1, O2, O3, O4, O5), Error = E> {
        (a, line_sep, b, line_sep, c, line_sep, d, line_sep, e)
            .map(|(a, _, b, _, c, _, d, _, e)| (a, b, c, d, e))
    }
}

pub trait LabelParser<T>: Send + Sync {
    fn parse(&self, label: &str) -> Result<T, String>;
}

pub struct NomParser<T> {
    pub name: &'static str,
    f: fn(label: &str) -> IResult<&str, T, VerboseError<&str>>,
}

impl<T> LabelParser<T> for NomParser<T> {
    fn parse(&self, label: &str) -> Result<T, String> {
        match all_consuming(self.f).parse(label) {
            Ok((_, chip)) => Ok(chip),
            Err(err) => Err(format!("{label}:{err:?}")),
        }
    }
}

#[derive(Clone)]
pub struct MultiParser<T: 'static> {
    parsers: &'static [&'static dyn LabelParser<T>],
}

impl<T> MultiParser<T> {
    pub const fn new(parsers: &'static [&'static dyn LabelParser<T>]) -> Self {
        MultiParser { parsers }
    }
}

impl<T> LabelParser<T> for MultiParser<T> {
    fn parse(&self, label: &str) -> Result<T, String> {
        let mut iter = self.parsers.iter();
        while let Some(parser) = iter.next() {
            if let Ok(m) = parser.parse(label) {
                if iter.any(|parser| parser.parse(label).is_ok()) {
                    warn!("Warning: multiple matches for {}", label);
                }
                return Ok(m);
            }
        }
        Err(format!("no match for {label}"))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UnknownChip;

pub static UNKNOWN_CHIP: NomParser<UnknownChip> = NomParser {
    name: "Unknown Chip",
    f: |_| Ok(("", UnknownChip)),
};

pub fn mgb_amp() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &sharp::SHARP_IR3R53, &sharp::SHARP_IR3R56)
}

pub fn agb_amp() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &sharp::SHARP_IR3R60, &rohm::ROHM_BH7835AFS,)
}

pub fn agb_reg() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_IR3E09
}

pub fn cgb_reg() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_IR3E06
}

pub fn dmg_amp() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_IR3R40
}

pub fn dmg_reg() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_IR3E02
}

pub fn rtc_sop_8() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &seiko::SEIKO_S3511A, &seiko::SEIKO_S3516AE)
}

pub fn rtc_sop_20() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &toshiba::TOSHIBA_TC8521AM)
}

pub fn line_decoder() -> &'static impl LabelParser<GenericPart> {
    &toshiba::TOSHIBA_TC7W139F
}

pub fn hex_inverter() -> &'static impl LabelParser<GenericPart> {
    &toshiba::TOSHIBA_TC74LVX04FT
}

pub fn flash_tsop_i_32_3v3() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &macronix::MACRONIX_MX29L010,
        &sanyo::SANYO_LE26FV10,
        &atmel::ATMEL_AT29LV512,
        &sst::SST_SST39VF512,
    )
}

pub fn flash_tsop_i_40_5v() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &macronix::MACRONIX_MX29F008)
}

pub fn supervisor_reset() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &mitsumi::MITSUMI_MM1026A,
        &mitsumi::MITSUMI_MM1134A,
        &rohm::ROHM_BA6129,
        &rohm::ROHM_BA6735,
        &mitsubishi::MITSUBISHI_M62021P,
        &ti::TI_SN74LV2416,
    )
}

pub fn gbs_reg() -> &'static impl LabelParser<GenericPart> {
    &mitsumi::MITSUMI_MM1592F
}

pub fn oxy_pmic() -> &'static impl LabelParser<GenericPart> {
    &mitsumi::MITSUMI_PM
}

pub fn oxy_u4() -> &'static impl LabelParser<GenericPart> {
    &unknown::UNKNOWN_OXY_U4
}

pub fn oxy_u5() -> &'static impl LabelParser<GenericPart> {
    &unknown::UNKNOWN_OXY_U5
}

pub fn ags_charge_ctrl() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &mitsumi::MITSUMI_MM1581A,
        &unknown::UNKNOWN_AGS_CHARGE_CONTROLLER
    )
}

pub fn ags_pmic_new() -> &'static impl LabelParser<GenericPart> {
    &mitsumi::MITSUMI_PM
}

pub fn mgl_transformer() -> &'static impl LabelParser<GenericPart> {
    &mitsumi::MITSUMI_MGL_TRANSFORMER
}

pub fn agb_pmic() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &mitsumi::MITSUMI_MM1514X,
        &seiko::SEIKO_S6960E,
        &rohm::ROHM_9750
    )
}

pub fn ags_pmic_old() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &seiko::SEIKO_S6403, &rohm::ROHM_9753)
}

pub fn fram_sop_28_3v3() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &fujitsu::FUJITSU_MB85R256,)
}

pub fn gbs_dol() -> &'static impl LabelParser<GenericPart> {
    &nec::NEC_GBS_DOL
}

pub fn cic() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &sharp::SHARP_F411, &sharp::SHARP_F413,)
}

pub fn icd2() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &rohm::ROHM_ICD2_R,
        &nec::NEC_ICD2_N,
        &nec::NEC_ICD2_R
    )
}

pub fn sgb_rom() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(
        MaskRom,
        &toshiba::TOSHIBA_SGB_ROM,
        &sharp::SHARP_SGB_ROM,
        &fujitsu::FUJITSU_SGB_ROM,
        &unknown::UNKNOWN_SGB_ROM,
        &nec::NEC_SGB_ROM,
    )
}

pub fn sgb2_coil() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &tdk::TDK_ZJY_M4A, &tdk::TDK_ZJY_M4PA,)
}

pub fn sgb2_rom() -> &'static impl LabelParser<MaskRom> {
    multi_parser!(MaskRom, &sharp::SHARP_SGB2_ROM, &oki::OKI_SGB2_ROM,)
}

pub fn dmg_stamp() -> &'static impl LabelParser<DmgStamp> {
    &stamp::DMG_STAMP
}

pub fn cgb_stamp() -> &'static impl LabelParser<CgbStamp> {
    &stamp::CGB_STAMP
}

pub fn rtc_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        &kds::KDS_32_KIHZ,
        &unknown::UNKNOWN_CRYSTAL_32_KIHZ
    )
}

pub fn dmg_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        &kds::KDS_D419_OLD,
        &unknown::UNKNOWN_DMG_CRYSTAL_4_MIHZ
    )
}

pub fn mgb_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        &kds::KDS_4_MIHZ_OLD,
        &kinseki::KINSEKI_4_MIHZ,
        &unknown::UNKNOWN_MGB_CRYSTAL_4_MIHZ
    )
}

pub fn sgb2_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(Crystal, &kds::KDS_D209, &kinseki::KINSEKI_20_MIHZ,)
}

pub fn cgb_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        &kds::KDS_D838,
        &kds::KDS_8_MIHZ,
        &kinseki::KINSEKI_8_MIHZ
    )
}

pub fn agb_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(
        Crystal,
        &kds::KDS_D419_NEW,
        &kds::KDS_4_MIHZ_NEW,
        &kinseki::KINSEKI_4_MIHZ,
    )
}

pub fn ags_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(Crystal, &kds::KDS_4_MIHZ_AGS, &kinseki::KINSEKI_4_MIHZ,)
}

pub fn gbs_crystal() -> &'static impl LabelParser<Crystal> {
    multi_parser!(Crystal, &kinseki::KINSEKI_32_MIHZ,)
}

pub fn dmg_soc_qfp_80() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &sharp::SHARP_LR35902, &sharp::SHARP_DMG_CPU)
}

pub fn dmg_soc_glop_top() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_DMG_CPU_GLOP_TOP
}

pub fn sgb_soc_qfp_80() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_SGB_CPU
}

pub fn mgb_soc_qfp_80() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_MGB
}

pub fn sgb2_soc_qfp_80() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_SGB2
}

pub fn cgb_soc_qfp_128_old() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_CGB
}

pub fn cgb_soc_qfp_128_new() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_CGB_E
}

pub fn agb_soc_qfp_128() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_AGB
}

pub fn agb_soc_qfp_156() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_AGB_B
}

pub fn agb_soc_bga() -> &'static impl LabelParser<GenericPart> {
    &sharp::SHARP_CPU_AGB_E
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameRomType {
    GlopTop, // 256 Kibit / 32 KiBit, gloptop
    A0,      // 256 Kibit / 32 KiB,  QFP
    B0,      // 512 Kibit / 64 KiB,  QFP
    B1,      // 512 Kibit / 64 KiB,  SOP
    C1,      // 1 Mibit   / 128 KiB, SOP
    D1,      // 2 Mibit   / 256 KiB, SOP
    E,       // 4 Mibit   / 512 KiB, TSOP-I
    E1,      // 4 Mibit   / 512 KiB, SOP
    F,       // 8 Mibit   / 1 MiB,   TSOP-I
    F1,      // 8 Mibit   / 1 MiB,   SOP
    F2,      // 8 Mibit   / 1 MiB,   TSOP-II
    G1,      // 16 Mibit  / 2 MiB,   SOP
    G2,      // 16 Mibit  / 2 MiB,   TSOP-II
    H2,      // 32 Mibit  / 4 MiB,   TSOP-II
    I2,      // 64 Mibit  / 8 MiB,   TSOP-II
    J2,      // 128 Mibit / 16 MiB,  TSOP-II
    K2,      // 256 Mibit / 32 MiB,  TSOP-II
}

impl GameRomType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GameRomType::GlopTop => "",
            GameRomType::A0 => "A0",
            GameRomType::B0 => "B0",
            GameRomType::B1 => "B1",
            GameRomType::C1 => "C1",
            GameRomType::D1 => "D1",
            GameRomType::E1 => "E1",
            GameRomType::E => "E",
            GameRomType::F1 => "F1",
            GameRomType::F2 => "F2",
            GameRomType::F => "F",
            GameRomType::G1 => "G1",
            GameRomType::G2 => "G2",
            GameRomType::H2 => "H2",
            GameRomType::I2 => "I2",
            GameRomType::J2 => "J2",
            GameRomType::K2 => "K2",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaskCode {
    Nec(String),
    Oki(String),
    Sharp(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameMaskRom {
    pub rom_id: String,
    pub rom_type: GameRomType,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub mask_code: Option<MaskCode>,
    pub date_code: Option<PartDateCode>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskRom {
    pub rom_id: String,
    pub manufacturer: Option<Manufacturer>,
    pub chip_type: Option<String>,
    pub mask_code: Option<MaskCode>,
    pub date_code: Option<PartDateCode>,
}

pub fn agb_mask_rom_tsop_ii_44_3v3() -> &'static impl LabelParser<GameMaskRom> {
    multi_parser!(
        GameMaskRom,
        &magnachip::MAGNACHIP_AC23V32101,
        &magnachip::MAGNACHIP_AC23V64101,
        &magnachip::MAGNACHIP_AC23V128111,
        &hynix::HYNIX_AC23V32101,
        &hynix::HYNIX_AC23V64101,
        &hynix::HYNIX_AC23V128111,
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

pub fn gb_mask_rom_glop_top_28_5v() -> &'static impl LabelParser<GameMaskRom> {
    &sharp::SHARP_MASK_ROM_GLOP_TOP_28_256_KIBIT
}

pub fn gb_mask_rom_sop_32_5v() -> &'static impl LabelParser<GameMaskRom> {
    multi_parser!(
        GameMaskRom,
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
        &fujitsu::FUJITSU_MASK_ROM_SOP_32_2_MIBIT,
        &fujitsu::FUJITSU_MASK_ROM_SOP_32_4_MIBIT,
    )
}

pub fn gb_mask_rom_sop_44_5v() -> &'static impl LabelParser<GameMaskRom> {
    multi_parser!(GameMaskRom, &macronix::MACRONIX_MX23C1605,)
}

pub fn gb_mask_rom_tsop_i_32_5v() -> &'static impl LabelParser<GameMaskRom> {
    multi_parser!(
        GameMaskRom,
        &sharp::SHARP_LH534XXXS,
        &sharp::SHARP_LH538XXXS,
        &macronix::MACRONIX_MX23C8006,
    )
}

pub fn gb_mask_rom_tsop_ii_44_5v() -> &'static impl LabelParser<GameMaskRom> {
    multi_parser!(
        GameMaskRom,
        &sharp::SHARP_LH5316XXX,
        &sharp::SHARP_LH5332XXX,
        &macronix::MACRONIX_MX23C1603,
        &macronix::MACRONIX_MX23C3203,
        &oki::OKI_MR531614,
        &nec::NEC_UPD23C16019W,
        &samsung::SAMSUNG_KM23C16120,
    )
}

pub fn gb_mask_rom_qfp_44_5v() -> &'static impl LabelParser<GameMaskRom> {
    multi_parser!(
        GameMaskRom,
        &sharp::SHARP_LH53259M,
        &sharp::SHARP_LH53515M,
        &oki::OKI_MASK_ROM_QFP_44_512_KIBIT,
    )
}

pub fn eeprom_sop_8_3v3() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(GenericPart, &rohm::ROHM_9853, &rohm::ROHM_9854)
}

pub fn eeprom_tssop_8_5v() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &unknown::UNKNOWN_LCS5_EEPROM,
        &unknown::UNKNOWN_LC56_EEPROM
    )
}

pub fn lcd_chip() -> &'static impl LabelParser<PartDateCode> {
    multi_parser!(
        PartDateCode,
        &sharp::SHARP_LCD_CHIP_OLD,
        &sharp::SHARP_LCD_CHIP_NEW
    )
}

pub fn lcd_screen() -> &'static impl LabelParser<PartDateCode> {
    multi_parser!(
        PartDateCode,
        &sharp::SHARP_LCD_SCREEN,
        &unknown::UNKNOWN_LCD_SCREEN
    )
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapperChip {
    Mbc1,
    Mbc1A,
    Mbc1B,
    Mbc1B1,
    Mbc2,
    Mbc2A,
    Mbc3,
    Mbc3A,
    Mbc3B,
    Mbc30,
    Mbc5,
    Mbc6,
    Mbc7,
    Huc1,
    Huc1A,
    Huc3,
    Mmm01,
    Tama5,
}

impl MapperChip {
    pub const fn display_name(&self) -> &'static str {
        match self {
            MapperChip::Mbc1 => "MBC1",
            MapperChip::Mbc1A => "MBC1A",
            MapperChip::Mbc1B => "MBC1B",
            MapperChip::Mbc1B1 => "MBC1B1",
            MapperChip::Mbc2 => "MBC2",
            MapperChip::Mbc2A => "MBC2A",
            MapperChip::Mbc3 => "MBC3",
            MapperChip::Mbc3A => "MBC3A",
            MapperChip::Mbc3B => "MBC3B",
            MapperChip::Mbc30 => "MBC30",
            MapperChip::Mbc5 => "MBC5",
            MapperChip::Mbc6 => "MBC6",
            MapperChip::Mbc7 => "MBC7",
            MapperChip::Mmm01 => "MMM01",
            MapperChip::Huc3 => "HuC-3",
            MapperChip::Huc1 => "HuC-1",
            MapperChip::Huc1A => "HuC-1A",
            MapperChip::Tama5 => "TAMA5",
        }
    }
    pub const fn mapper_type(&self) -> MapperType {
        match self {
            MapperChip::Mbc1 => MapperType::Mbc1,
            MapperChip::Mbc1A => MapperType::Mbc1,
            MapperChip::Mbc1B => MapperType::Mbc1,
            MapperChip::Mbc1B1 => MapperType::Mbc1,
            MapperChip::Mbc2 => MapperType::Mbc2,
            MapperChip::Mbc2A => MapperType::Mbc2,
            MapperChip::Mbc3 => MapperType::Mbc3,
            MapperChip::Mbc3A => MapperType::Mbc3,
            MapperChip::Mbc3B => MapperType::Mbc3,
            MapperChip::Mbc30 => MapperType::Mbc30,
            MapperChip::Mbc5 => MapperType::Mbc5,
            MapperChip::Mbc6 => MapperType::Mbc6,
            MapperChip::Mbc7 => MapperType::Mbc7,
            MapperChip::Huc1 => MapperType::Huc1,
            MapperChip::Huc1A => MapperType::Huc1,
            MapperChip::Huc3 => MapperType::Huc3,
            MapperChip::Mmm01 => MapperType::Mmm01,
            MapperChip::Tama5 => MapperType::Tama5,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapperType {
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc30,
    Mbc5,
    Mbc6,
    Mbc7,
    Huc1,
    Huc3,
    Mmm01,
    Tama5,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mapper {
    pub kind: MapperChip,
    pub manufacturer: Option<Manufacturer>,
    pub date_code: Option<PartDateCode>,
}

pub fn mbc1_sop24() -> &'static impl LabelParser<Mapper> {
    multi_parser!(
        Mapper,
        &sharp::SHARP_MBC1,
        &sharp::SHARP_MBC1A,
        &sharp::SHARP_MBC1B,
        &sharp::SHARP_MBC1B1,
        &nec::NEC_MBC1B,
        &panasonic::PANASONIC_MBC1B,
        &motorola::MOTOROLA_MBC1B,
        &unknown::UNKNOWN_MBC1B,
    )
}

pub fn mbc2_sop28() -> &'static impl LabelParser<Mapper> {
    multi_parser!(
        Mapper,
        &nec::NEC_MBC2A,
        &panasonic::PANASONIC_MBC2A,
        &sharp::SHARP_MBC2A,
    )
}

pub fn mbc3_qfp32() -> &'static impl LabelParser<Mapper> {
    multi_parser!(
        Mapper,
        &panasonic::PANASONIC_MBC3A,
        &panasonic::PANASONIC_MBC3B,
        &rohm::ROHM_MBC3,
        &rohm::ROHM_MBC3A,
        &rohm::ROHM_MBC3B,
        &sharp::SHARP_MBC3,
        &sharp::SHARP_MBC3A,
    )
}

pub fn mbc30_qfp32() -> &'static impl LabelParser<Mapper> {
    multi_parser!(Mapper, &panasonic::PANASONIC_MBC30, &rohm::ROHM_MBC30,)
}

pub fn mbc5_qfp32() -> &'static impl LabelParser<Mapper> {
    multi_parser!(
        Mapper,
        &panasonic::PANASONIC_MBC5,
        &rohm::ROHM_MBC5,
        &sharp::SHARP_MBC5,
        &ti::TI_MBC5,
    )
}

pub fn mbc6_qfp64() -> &'static impl LabelParser<Mapper> {
    multi_parser!(Mapper, &nec::NEC_MBC6,)
}

pub fn mbc7_qfp56() -> &'static impl LabelParser<Mapper> {
    multi_parser!(Mapper, &rohm::ROHM_MBC7,)
}

pub fn mmm01_qfp32() -> &'static impl LabelParser<Mapper> {
    multi_parser!(Mapper, &unknown::UNKNOWN_MMM01,)
}

pub fn huc1_qfp32() -> &'static impl LabelParser<Mapper> {
    multi_parser!(Mapper, &hudson::HUDSON_HUC1, &hudson::HUDSON_HUC1A,)
}

pub fn huc3_qfp48() -> &'static impl LabelParser<Mapper> {
    multi_parser!(Mapper, &hudson::HUDSON_HUC3,)
}

pub fn sram_sop_28_5v() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        // 256 Kibit / 32 KiB
        &bsi::BSI_BS62LV256,
        &hynix::HYNIX_HY62WT08081,
        &lgs::HYUNDAI_GM76C256,
        &lgs::LGS_GM76C256,
        &lsi_logic::LSI_LOGIC_LH52B256N,
        &mosel_vitelic::MOSEL_VITELIC_LH52B256N,
        &rohm::ROHM_BR62256F,
        &sanyo::SANYO_LC35256,
        &sharp::SHARP_LH52256CN,
        &sharp::SHARP_LH52256CVN,
        &winbond::WINBOND_W24257S,
        &winbond::WINBOND_W24258S,
        // 64 Kibit / 8 KiB
        &crosslink::CROSSLINK_LH5268AN,
        &crosslink::CROSSLINK_LH52A64N,
        &hyundai::HYUNDAI_HY6264,
        &lsi_logic::LSI_LOGIC_LH5168N,
        &lsi_logic::LSI_LOGIC_LH5264N4T,
        &lsi_logic::LSI_LOGIC_LH5264TN,
        &lsi_logic::LSI_LOGIC_LH52A64N,
        &mosel_vitelic::MOSEL_VITELIC_LH5168N,
        &mosel_vitelic::MOSEL_VITELIC_LH5268AN,
        &mosel_vitelic::MOSEL_VITELIC_LH52A64N,
        &rohm::ROHM_BR6265BF,
        &rohm::ROHM_XLJ6265AF,
        &rohm::ROHM_XLJ6265BF,
        &sanyo::SANYO_LC3564,
        &sharp::SHARP_LH5160N,
        &sharp::SHARP_LH5164AN,
        &sharp::SHARP_LH5164N,
        &sharp::SHARP_LH5168N,
        &sharp::SHARP_LH5168NF,
        &sharp::SHARP_LH5264N4,
        &sharp::SHARP_LH5264TN,
        &sharp::SHARP_LH52A64N,
        &victronix::VICTRONIX_VN4464,
        &winbond::WINBOND_W2465S,
    )
}

pub fn sram_sop_28_3v3() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        // 64 Kibit / 8 KiB
        &sanyo::SANYO_LC3564,
        // 256 Kibit / 32 KiB
        &bsi::BSI_BS62LV256,
        &hynix::HYNIX_HY62WT08081,
        &lgs::HYUNDAI_GM76V256,
        &sanyo::SANYO_LC35256,
        &sharp::SHARP_LH52256CVN,
        &winbond::WINBOND_W24258S,
    )
}

pub fn sram_sop_32_5v() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        // 1 Mibit / 128 KiB
        &hyundai::HYUNDAI_HY628100,
    )
}

pub fn sram_tsop_i_28_3v3() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        // 256 Kibit / 32 KiB
        &sharp::SHARP_LH51D256T,
        &sharp::SHARP_LH52CV256JT,
        &sharp::SHARP_LH52256CVT,
    )
}

pub fn sram_tsop_i_28_5v() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        // 256 Kibit / 32 KiB
        &sharp::SHARP_LH52256CT,
        &sharp::SHARP_LH52256CVT
    )
}

pub fn sram_tsop_i_48() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        // 2 Mibit / 256 KiB / 128x16
        &nec::NEC_UPD442012A_X,
        &nec::NEC_UPD442012L_X,
        &fujitsu::FUJITSU_MB82D12160,
        &hynix::HYNIX_HY62LF16206,
        &st_micro::ST_MICRO_M68AS128,
        &amic::AMIC_LP62S16128,
        &bsi::BSI_BS616LV2018,
        &bsi::BSI_BS616LV2019,
        &toshiba::TOSHIBA_TC55V200
    )
}

pub fn battery() -> &'static impl LabelParser<PartDateCode> {
    static BATTERY: NomParser<PartDateCode> = NomParser {
        name: "CRxxxx battery",
        f: |input| {
            separated_pair(year2, tag("-"), month2)
                .map(|(year, month)| PartDateCode::YearMonth { year, month })
                .parse(input)
        },
    };
    &BATTERY
}
