// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use log::warn;
use nom::{combinator::all_consuming, error::VerboseError, IResult, Parser as _};
use regex::{Captures, Regex, RegexBuilder};
use std::str::FromStr;

use crate::{
    macros::{multi_parser, single_parser},
    time::{Month, Week},
};

pub use self::{
    accelerometer::Accelerometer,
    ags_charge_ctrl::AgsChargeController,
    cgb_stamp::CgbStamp,
    coil::Coil,
    dmg_stamp::DmgStamp,
    eeprom::Eeprom,
    gen1_soc::{Gen1Soc, Gen1SocKind},
    gen2_soc::{Gen2Soc, Gen2SocKind},
    lcd_chip::LcdChip,
    lcd_screen::LcdScreen,
    mapper::{Huc1Version, Mapper, MapperType, Mbc1Version, Mbc2Version, Mbc3Version},
};

pub mod accelerometer;
pub mod agb_soc_bga;
pub mod agb_soc_qfp_128;
pub mod agb_soc_qfp_156;
pub mod ags_charge_ctrl;
pub mod amic;
pub mod atmel;
pub mod bsi;
pub mod cgb_soc;
pub mod cgb_stamp;
pub mod cic;
pub mod coil;
pub mod crystal_20mihz;
pub mod crystal_32kihz;
pub mod crystal_32mihz;
pub mod crystal_4mihz;
pub mod crystal_8mihz;
pub mod dmg_stamp;
pub mod eeprom;
pub mod fujitsu;
pub mod gen1_soc;
pub mod gen2_soc;
pub mod hynix;
pub mod hyundai;
pub mod lcd_chip;
pub mod lcd_screen;
pub mod lgs;
pub mod macronix;
pub mod magnachip;
pub mod mapper;
pub mod mitsubishi;
pub mod mitsumi;
pub mod nec;
pub mod oki;
pub mod oxy_u4;
pub mod oxy_u5;
pub mod rohm;
pub mod samsung;
pub mod sanyo;
pub mod seiko;
pub mod sgb_rom;
pub mod sharp;
pub mod sram;
pub mod sst;
pub mod st_micro;
pub mod tama;
pub mod ti;
pub mod toshiba;
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
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub week: Option<Week>,
}

impl Crystal {
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

fn kds_month1(text: &str) -> Result<Month, String> {
    month1_alpha(text)
}

fn month1_alpha(text: &str) -> Result<Month, String> {
    match text {
        "A" => Ok(Month::January),
        "B" => Ok(Month::February),
        "C" => Ok(Month::March),
        "D" => Ok(Month::April),
        "E" => Ok(Month::May),
        "F" => Ok(Month::June),
        "G" => Ok(Month::July),
        "H" => Ok(Month::August),
        // I is intentionally skipped
        "J" => Ok(Month::September),
        "K" => Ok(Month::October),
        "L" => Ok(Month::November),
        "M" => Ok(Month::December),
        _ => Err(format!("Invalid 1-letter month: {}", text)),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

pub fn year1(text: &str) -> Result<Year, String> {
    if text.len() != 1 {
        return Err(format!("Invalid 1-digit year: {}", text));
    }
    match u8::from_str(text) {
        Ok(value) => Ok(Year::Partial(value)),
        _ => Err(format!("Invalid 1-digit year: {}", text)),
    }
}

mod for_nom {
    use nom::{
        branch::alt,
        bytes::streaming::{tag, take},
        character::streaming::{anychar, char, satisfy},
        combinator::{map_opt, recognize},
        error::ParseError,
        multi::fold_many_m_n,
        sequence::tuple,
        IResult, Parser,
    };

    use super::{PartDateCode, Year};
    use crate::time::{Month, Week};

    pub fn dmg_rom_code<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, &'a str, E> {
        recognize(tuple((
            tag("DMG-"),
            satisfy_m_n(3, 4, |c| c.is_ascii_digit() || c.is_ascii_uppercase()),
            char('-'),
            digits(1),
        )))
    }

    pub fn cgb_rom_code<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, &'a str, E> {
        recognize(tuple((tag("CGB-"), alnum_uppers(4), char('-'), digits(1))))
    }

    pub fn agb_rom_code<'a, E: ParseError<&'a str>>() -> impl Parser<&'a str, &'a str, E> {
        recognize(tuple((tag("AGB-"), alnum_uppers(4), char('-'), digits(1))))
    }

    pub fn satisfy_m_n<'a, E: ParseError<&'a str>>(
        min: usize,
        max: usize,
        f: impl Fn(char) -> bool,
    ) -> impl Parser<&'a str, &'a str, E> {
        recognize(fold_many_m_n(min, max, satisfy(f), || (), |_, _| ()))
    }

    pub fn satisfy_m_n_complete<'a, E: ParseError<&'a str>>(
        min: usize,
        max: usize,
        f: impl Fn(char) -> bool,
    ) -> impl Parser<&'a str, &'a str, E> {
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
    ) -> impl Parser<&'a str, &'a str, E> {
        satisfy_m_n(count, count, |c| {
            c.is_ascii_digit() || c.is_ascii_uppercase()
        })
    }

    pub fn uppers<'a, E: ParseError<&'a str>>(count: usize) -> impl Parser<&'a str, &'a str, E> {
        satisfy_m_n(count, count, |c| c.is_ascii_uppercase())
    }

    pub fn alphas<'a, E: ParseError<&'a str>>(count: usize) -> impl Parser<&'a str, &'a str, E> {
        satisfy_m_n(count, count, |c| c.is_ascii_alphabetic())
    }

    pub fn digits<'a, E: ParseError<&'a str>>(count: usize) -> impl Parser<&'a str, &'a str, E> {
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
        map_opt(take(2_usize), |text| match text {
            "AA" => Some(Year::Full(2000)),
            "AL" => Some(Year::Full(2001)),
            _ => match u16::from_str_radix(text, 10) {
                Ok(value @ 0..=87) => Some(Year::Full(value + 2000)),
                Ok(value @ 88..=99) => Some(Year::Full(value + 1900)),
                _ => None,
            },
        })
        .parse(input)
    }

    pub fn year1_week2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        tuple((year1, week2))
            .map(|(year, week)| PartDateCode::YearWeek { year, week })
            .parse(input)
    }

    pub fn year2_week2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, PartDateCode, E> {
        tuple((year2, week2))
            .map(|(year, week)| PartDateCode::YearWeek { year, week })
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

    pub fn week2<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Week, E> {
        map_opt(take(2_usize), |text| {
            u8::from_str_radix(text, 10)
                .ok()
                .and_then(|v| Week::try_from(v).ok())
        })
        .parse(input)
    }

    fn line_sep<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, char, E> {
        alt((char(' '), char('\n'))).parse(input)
    }

    pub fn lines2<'a, O1, O2, O3, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, O1, E>,
        b: impl Parser<&'a str, O2, E>,
    ) -> impl Parser<&'a str, (O1, O2), E> {
        tuple((a, line_sep, b)).map(|(a, _, b)| (a, b))
    }

    pub fn lines3<'a, O1, O2, O3, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, O1, E>,
        b: impl Parser<&'a str, O2, E>,
        c: impl Parser<&'a str, O3, E>,
    ) -> impl Parser<&'a str, (O1, O2, O3), E> {
        tuple((a, line_sep, b, line_sep, c)).map(|(a, _, b, _, c)| (a, b, c))
    }

    pub fn lines4<'a, O1, O2, O3, O4, E: ParseError<&'a str>>(
        a: impl Parser<&'a str, O1, E>,
        b: impl Parser<&'a str, O2, E>,
        c: impl Parser<&'a str, O3, E>,
        d: impl Parser<&'a str, O4, E>,
    ) -> impl Parser<&'a str, (O1, O2, O3, O4), E> {
        tuple((a, line_sep, b, line_sep, c, line_sep, d)).map(|(a, _, b, _, c, _, d)| (a, b, c, d))
    }
}

pub fn year2(text: &str) -> Result<Year, String> {
    if text.len() != 2 {
        return Err(format!("Invalid 2-digit year: {}", text));
    }
    if text == "AL" {
        return Ok(Year::Full(2000));
    }
    if text == "AA" {
        return Ok(Year::Full(2001));
    }
    match u16::from_str(text) {
        Ok(value @ 0..=87) => Ok(Year::Full(value + 2000)),
        Ok(value @ 88..=99) => Ok(Year::Full(value + 1900)),
        _ => Err(format!("Invalid 2-digit year: {}", text)),
    }
}

pub fn week2(text: &str) -> Result<Week, String> {
    if text.len() != 2 {
        return Err(format!("Invalid 2-digit week: {}", text));
    }
    u8::from_str(text)
        .ok()
        .and_then(|v| Week::try_from(v).ok())
        .ok_or_else(|| format!("Invalid 2-digit week: {}", text))
}

pub fn month2(text: &str) -> Result<Month, String> {
    if text.len() != 2 {
        return Err(format!("Invalid 2-digit month: {}", text));
    }
    u8::from_str(text)
        .ok()
        .and_then(|v| Month::try_from(v).ok())
        .ok_or_else(|| format!("Invalid 2-digit month: {}", text))
}

pub trait LabelParser<T>: Send + Sync {
    fn parse(&self, label: &str) -> Result<T, String>;
}

#[derive(Clone)]
pub struct RegexParser<T> {
    regex: Regex,
    f: fn(Captures) -> Result<T, String>,
}

impl<T> LabelParser<T> for RegexParser<T> {
    fn parse(&self, label: &str) -> Result<T, String> {
        if let Some(captures) = self.regex.captures(label) {
            (self.f)(captures)
        } else {
            Err(format!("no match for {label}"))
        }
    }
}

impl<T> RegexParser<T> {
    pub fn compile(regex: &str, f: fn(Captures) -> Result<T, String>) -> RegexParser<T> {
        let regex = RegexBuilder::new(regex)
            .ignore_whitespace(true)
            .build()
            .expect("Failed to compile regex");
        RegexParser { regex, f }
    }
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

pub fn unknown_chip() -> &'static impl LabelParser<UnknownChip> {
    single_parser!(UnknownChip, r#"^.*$"#, move |_| Ok(UnknownChip))
}

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

pub fn icd2() -> &'static impl LabelParser<GenericPart> {
    multi_parser!(
        GenericPart,
        &rohm::ROHM_ICD2_R,
        &nec::NEC_ICD2_N,
        &nec::NEC_ICD2_R
    )
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
