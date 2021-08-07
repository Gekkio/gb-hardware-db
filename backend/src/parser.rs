use regex::{Captures, Regex, RegexBuilder, RegexSet, RegexSetBuilder};
use std::str::FromStr;

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
    ram::Ram,
    ram_backup::RamBackup,
    sgb_rom::SgbRom,
    tama::TamaType,
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
pub mod ram;
pub mod ram_backup;
pub mod sgb_rom;
pub mod sram_tsop1_48;
pub mod tama;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipYearWeek {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Crystal {
    pub manufacturer: Option<Manufacturer>,
    pub frequency: u32,
    pub year: Option<Year>,
    pub month: Option<u8>,
    pub week: Option<u8>,
}

fn kds_month(text: &str) -> Result<u8, String> {
    match text {
        "A" => Ok(1),
        "B" => Ok(2),
        "C" => Ok(3),
        "D" => Ok(4),
        "E" => Ok(5),
        "F" => Ok(6),
        "G" => Ok(7),
        "H" => Ok(8),
        // I is intentionally skipped
        "J" => Ok(9),
        "K" => Ok(10),
        "L" => Ok(11),
        "M" => Ok(12),
        _ => Err(format!("Invalid 1-letter month: {}", text)),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Manufacturer {
    Amic,
    Analog,
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
    Mitsubishi,
    Mitsumi,
    MoselVitelic,
    Motorola,
    Nec,
    Oki,
    Rohm,
    Samsung,
    Sanyo,
    Sharp,
    Smsc,
    StMicro,
    Tdk,
    TexasInstruments,
    Toshiba,
    Victronix,
    Winbond,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Year {
    Full(u16),
    Partial(u8),
}

pub fn year1(text: &str) -> Result<Year, String> {
    match u8::from_str(text) {
        Ok(value) => Ok(Year::Partial(value)),
        _ => Err(format!("Invalid 1-digit year: {}", text)),
    }
}

pub fn year2(text: &str) -> Result<Year, String> {
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

pub fn week2(text: &str) -> Result<u8, String> {
    match u8::from_str(text) {
        Ok(value @ 1..=53) => Ok(value),
        _ => Err(format!("Invalid 2-digit week: {}", text)),
    }
}

pub fn month2(text: &str) -> Result<u8, String> {
    match u8::from_str(text) {
        Ok(value @ 1..=12) => Ok(value),
        _ => Err(format!("Invalid 2-digit month: {}", text)),
    }
}

pub trait LabelParser<T> {
    fn parse(&self, label: &str) -> Result<T, String>;
    fn parsers(&self) -> Vec<&SingleParser<T>>;
}

#[derive(Clone)]
pub struct SingleParser<T> {
    regex: Regex,
    f: fn(Captures) -> Result<T, String>,
}

impl<T> LabelParser<T> for SingleParser<T> {
    fn parse(&self, label: &str) -> Result<T, String> {
        if let Some(captures) = self.regex.captures(label) {
            (self.f)(captures)
        } else {
            Err("no match".to_owned())
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticRam {
    pub family: Option<&'static str>,
    pub part: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
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

impl<T> LabelParser<T> for MultiParser<T> {
    fn parse(&self, label: &str) -> Result<T, String> {
        let matches = self.regex_set.matches(label);
        if matches.iter().count() > 1 {
            eprintln!("Warning: multiple matches for {}", label);
        }
        matches
            .iter()
            .find_map(|m| self.parsers[m].parse(label).ok())
            .ok_or_else(|| "no match".to_owned())
    }

    fn parsers(&self) -> Vec<&SingleParser<T>> {
        self.parsers.clone()
    }
}
