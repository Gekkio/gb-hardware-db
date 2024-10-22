// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use log::warn;
use nom::{combinator::all_consuming, error::VerboseError, IResult, Parser as _};
use regex::{Captures, Regex, RegexBuilder};
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

mod seiko {
    use nom::{
        character::streaming::{anychar, satisfy},
        combinator::map_opt,
        error::ParseError,
        multi::count,
        IResult, Parser,
    };

    use super::Year;

    pub fn year1<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Year, E> {
        map_opt(anychar, |ch| match ch {
            '0' => Some(Year::Partial(0)),
            '1' | 'A' => Some(Year::Partial(1)),
            '2' | 'B' => Some(Year::Partial(2)),
            '3' | 'C' => Some(Year::Partial(3)),
            '4' | 'D' => Some(Year::Partial(4)),
            '5' | 'E' => Some(Year::Partial(5)),
            '6' | 'F' => Some(Year::Partial(6)),
            '7' | 'G' => Some(Year::Partial(7)),
            '8' | 'H' => Some(Year::Partial(8)),
            '9' | 'J' => Some(Year::Partial(9)),
            _ => None,
        })
        .parse(input)
    }

    pub fn lot_code<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
        count(satisfy(|c| c.is_ascii_digit()), 4)
            .map(|_| ())
            .parse(input)
    }
}

mod macronix {
    use nom::{
        character::streaming::satisfy,
        combinator::{opt, recognize},
        error::ParseError,
        sequence::tuple,
        IResult, Parser as _,
    };

    use super::{
        for_nom::{self, digits, uppers},
        ChipDateCode,
    };

    pub fn assembly_vendor_code<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, char, E> {
        satisfy(|c| match c {
            'a' => true, // ChipMOS
            'B' => true, // OSE / Orient Semiconductor Electronics
            'E' => true, // ???
            'K' => true, // ASEKS
            'J' => true, // ASEJ
            'L' => true, // LINGSEN
            'M' => true, // ???
            'N' => true, // ???
            'S' => true, // SPIL
            'T' => true, // STS
            'X' => true, // ASECL
            _ => false,
        })
        .parse(input)
    }

    pub fn date_code<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, ChipDateCode, E> {
        for_nom::year2_week2(input)
    }

    pub fn lot_code_new<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        // [0-9][A-Z][0-9]{4} + [0-9]{2}[A-Z][0-9]
        recognize(tuple((
            tuple((digits(1), uppers(1), digits(4))),
            opt(nom::bytes::complete::take(4_usize).and_then(tuple((
                digits(2),
                uppers(1),
                digits(1),
            )))),
        )))
        .parse(input)
    }

    pub fn lot_code_old<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        digits(5).parse(input)
    }
}

mod toshiba {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Package {
        SOP,
    }

    impl Package {
        pub const fn code_char(&self) -> char {
            match self {
                Package::SOP => 'M',
            }
        }
    }
}

mod for_nom {
    use nom::{
        bytes::streaming::take,
        character::streaming::satisfy,
        combinator::{map_opt, recognize},
        error::ParseError,
        multi::fold_many_m_n,
        sequence::tuple,
        IResult, Parser,
    };

    use super::{ChipDateCode, Year};
    use crate::time::{Month, Week};

    pub fn satisfy_m_n<'a, E: ParseError<&'a str>>(
        min: usize,
        max: usize,
        f: impl Fn(char) -> bool,
    ) -> impl Parser<&'a str, &'a str, E> {
        recognize(fold_many_m_n(min, max, satisfy(f), || (), |_, _| ()))
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

    pub fn year2_week2<'a, E: ParseError<&'a str>>(
        input: &'a str,
    ) -> IResult<&'a str, ChipDateCode, E> {
        tuple((year2, week2))
            .map(|(year, week)| ChipDateCode::YearWeek { year, week })
            .parse(input)
    }

    pub fn month1_alpha<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Month, E> {
        map_opt(take(1_usize), |text| match text {
            "A" => Some(Month::January),
            "B" => Some(Month::February),
            "C" => Some(Month::March),
            "D" => Some(Month::April),
            "E" => Some(Month::May),
            "F" => Some(Month::June),
            "G" => Some(Month::July),
            "H" => Some(Month::August),
            // I is intentionally skipped
            "J" => Some(Month::September),
            "K" => Some(Month::October),
            "L" => Some(Month::November),
            "M" => Some(Month::December),
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

impl<T> LabelParser<T> for RegexParser<T>
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
            Err(err) => Err(format!("{err:?}")),
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

impl ParsedData for UnknownChip {}

pub fn unknown_chip() -> &'static impl LabelParser<UnknownChip> {
    single_parser!(UnknownChip, r#"^.*$"#, move |_| Ok(UnknownChip))
}
