// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use log::warn;
use nom::{
    character::streaming::{one_of, satisfy},
    combinator::all_consuming,
    error::{ParseError, VerboseError},
    IResult, Parser as _,
};
use regex::{Captures, Regex, RegexBuilder, RegexSet, RegexSetBuilder};
use std::{any::Any, fmt, str::FromStr};

use crate::{
    macros::single_parser,
    time::{Month, Week},
};

pub use self::{
    accelerometer::Accelerometer,
    agb_amp::AgbAmp,
    agb_pmic::AgbPmic,
    agb_reg::AgbReg,
    ags_charge_ctrl::AgsChargeController,
    ags_pmic_old::AgsPmicOld,
    cgb_reg::CgbReg,
    cgb_soc::CgbSoc,
    cgb_stamp::CgbStamp,
    cic::Cic,
    coil::Coil,
    dmg_amp::DmgAmp,
    dmg_reg::DmgReg,
    dmg_stamp::DmgStamp,
    eeprom::Eeprom,
    flash::Flash,
    fram::Fram,
    gbs_dol::GbsDol,
    gbs_reg::GbsReg,
    gen1_soc::{Gen1Soc, Gen1SocKind},
    gen2_soc::{Gen2Soc, Gen2SocKind},
    hex_inverter::HexInverter,
    icd2::Icd2,
    lcd_chip::LcdChip,
    lcd_screen::LcdScreen,
    line_decoder::LineDecoder,
    mapper::{Huc1Version, Mapper, MapperType, Mbc1Version, Mbc2Version, Mbc3Version},
    mask_rom::MaskRom,
    mgb_amp::MgbAmp,
    mgl_transformer::Transformer,
    oxy_pmic::OxyPmic,
    oxy_u4::OxyU4,
    oxy_u5::OxyU5,
    rtc::Rtc,
    sgb_rom::SgbRom,
    sram::StaticRam,
    supervisor_reset::SupervisorReset,
    tama::{Tama, TamaType},
};

pub mod accelerometer;
pub mod agb_amp;
pub mod agb_pmic;
pub mod agb_reg;
pub mod agb_soc_bga;
pub mod agb_soc_qfp_128;
pub mod agb_soc_qfp_156;
pub mod ags_charge_ctrl;
pub mod ags_pmic_new;
pub mod ags_pmic_old;
pub mod cgb_reg;
pub mod cgb_soc;
pub mod cgb_stamp;
pub mod cic;
pub mod coil;
pub mod crystal_20mihz;
pub mod crystal_32kihz;
pub mod crystal_32mihz;
pub mod crystal_4mihz;
pub mod crystal_8mihz;
pub mod dmg_amp;
pub mod dmg_reg;
pub mod dmg_stamp;
pub mod eeprom;
pub mod flash;
pub mod fram;
pub mod gbs_dol;
pub mod gbs_reg;
pub mod gen1_soc;
pub mod gen2_soc;
pub mod hex_inverter;
pub mod icd2;
pub mod lcd_chip;
pub mod lcd_screen;
pub mod line_decoder;
pub mod mapper;
pub mod mask_rom;
pub mod mgb_amp;
pub mod mgl_transformer;
pub mod oxy_pmic;
pub mod oxy_u4;
pub mod oxy_u5;
pub mod rtc;
pub mod sgb_rom;
pub mod sram;
pub mod supervisor_reset;
pub mod tama;

pub trait ParsedData: fmt::Debug + Any {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChipDateCode {
    Year { year: Year },
    YearMonth { year: Year, month: Month },
    YearWeek { year: Year, week: Week },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericChip {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub date_code: Option<ChipDateCode>,
}

impl ParsedData for GenericChip {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipYearWeek {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

impl ParsedData for ChipYearWeek {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipYearMonthWeek {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub week: Option<Week>,
}

impl ParsedData for ChipYearMonthWeek {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub frequency: u32,
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub week: Option<Week>,
}

impl ParsedData for Crystal {}

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

fn seiko_year1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Year, E> {
    one_of("0123456789ABCDEFGHJ")
        .map(|ch| match ch {
            '0' => Year::Partial(0),
            '1' | 'A' => Year::Partial(1),
            '2' | 'B' => Year::Partial(2),
            '3' | 'C' => Year::Partial(3),
            '4' | 'D' => Year::Partial(4),
            '5' | 'E' => Year::Partial(5),
            '6' | 'F' => Year::Partial(6),
            '7' | 'G' => Year::Partial(7),
            '8' | 'H' => Year::Partial(8),
            '9' | 'J' => Year::Partial(9),
            _ => unreachable!(),
        })
        .parse(input)
}

fn alnum_upper<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, char, E> {
    satisfy(|c| match c {
        'A'..='Z' => true,
        '0'..='9' => true,
        _ => false,
    })
    .parse(input)
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
    fn parsers(&self) -> Vec<&SingleParser<T>>;
}

#[derive(Clone)]
pub struct SingleParser<T> {
    regex: Regex,
    f: fn(Captures) -> Result<T, String>,
}

impl<T> LabelParser<T> for SingleParser<T>
where
    T: ParsedData,
{
    fn parse(&self, label: &str) -> Result<T, String> {
        if let Some(captures) = self.regex.captures(label) {
            (self.f)(captures)
        } else {
            Err(format!("no match for {label}"))
        }
    }
    fn parsers(&self) -> Vec<&SingleParser<T>> {
        vec![self]
    }
}

impl<T> SingleParser<T> {
    pub fn compile(regex: &str, f: fn(Captures) -> Result<T, String>) -> SingleParser<T> {
        let regex = RegexBuilder::new(regex)
            .ignore_whitespace(true)
            .build()
            .expect("Failed to compile regex");
        SingleParser { regex, f }
    }
}

#[derive(Clone)]
pub struct MultiParser<T: 'static> {
    parsers: Vec<&'static SingleParser<T>>,
    regex_set: RegexSet,
}

impl<T> MultiParser<T> {
    pub fn compile(parsers: Vec<&'static SingleParser<T>>) -> MultiParser<T> {
        let regex_set = RegexSetBuilder::new(parsers.iter().map(|m| m.regex.as_str()))
            .ignore_whitespace(true)
            .build()
            .expect("Failed to compile regex set");
        MultiParser { parsers, regex_set }
    }
}

impl<T> LabelParser<T> for MultiParser<T>
where
    T: ParsedData,
{
    fn parse(&self, label: &str) -> Result<T, String> {
        let matches = self.regex_set.matches(label);
        if matches.iter().count() > 1 {
            warn!("Warning: multiple matches for {}", label);
        }
        matches
            .iter()
            .find_map(|m| self.parsers[m].parse(label).ok())
            .ok_or_else(|| format!("no match for {label}"))
    }

    fn parsers(&self) -> Vec<&SingleParser<T>> {
        self.parsers.clone()
    }
}

pub struct NomFnParser<T> {
    pub name: &'static str,
    f: fn(label: &str) -> IResult<&str, T, VerboseError<&str>>,
}

impl<T> LabelParser<T> for NomFnParser<T> {
    fn parse(&self, label: &str) -> Result<T, String> {
        match all_consuming(self.f).parse(label) {
            Ok((_, chip)) => Ok(chip),
            Err(err) => Err(format!("{err:?}")),
        }
    }

    fn parsers(&self) -> Vec<&crate::parser::SingleParser<T>> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct MultiNomFnParser<T: 'static> {
    parsers: &'static [&'static NomFnParser<T>],
}

impl<T> MultiNomFnParser<T> {
    pub const fn new(parsers: &'static [&'static NomFnParser<T>]) -> Self {
        MultiNomFnParser { parsers }
    }
}

impl<T> LabelParser<T> for MultiNomFnParser<T> {
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

    fn parsers(&self) -> Vec<&SingleParser<T>> {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UnknownChip;

impl ParsedData for UnknownChip {}

pub fn unknown_chip() -> &'static impl LabelParser<UnknownChip> {
    single_parser!(UnknownChip, r#"^.*$"#, move |_| Ok(UnknownChip))
}
