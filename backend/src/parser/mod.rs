use regex::{Captures, Regex, RegexBuilder, RegexSet, RegexSetBuilder};
use std::str::FromStr;

pub use self::accelerometer::{parse_accelerometer, Accelerometer};
pub use self::agb_amp::{parse_agb_amp, AgbAmp};
pub use self::agb_cpu::{parse_agb_cpu, AgbCpu};
pub use self::agb_reg::{parse_agb_reg, AgbReg};
pub use self::agb_u4::{parse_agb_u4, AgbU4};
pub use self::ags_u4::{parse_ags_u4, AgsU4};
pub use self::ags_u5::{parse_ags_u5, AgsU5};
pub use self::cgb_cpu::{parse_cgb_cpu, CgbCpu};
pub use self::cgb_reg::{parse_cgb_reg, CgbReg};
pub use self::cgb_stamp::{parse_cgb_stamp, CgbStamp};
pub use self::cic::{parse_cic, Cic};
pub use self::coil::{parse_coil, Coil};
pub use self::crystal::{parse_crystal, Crystal};
pub use self::dmg_amp::{parse_dmg_amp, DmgAmp};
pub use self::dmg_reg::{parse_dmg_reg, DmgReg};
pub use self::dmg_stamp::{parse_dmg_stamp, DmgStamp};
pub use self::eeprom::{parse_eeprom, Eeprom};
pub use self::flash::{parse_flash, Flash};
pub use self::gbs_dol::{parse_gbs_dol, GbsDol};
pub use self::gbs_reg::{parse_gbs_reg, GbsReg};
pub use self::gen1_cpu::{parse_gen1_cpu, Gen1Cpu, Gen1CpuKind};
pub use self::gen2_cpu::{parse_gen2_cpu, Gen2Cpu, Gen2CpuKind};
pub use self::hex_inverter::{parse_hex_inverter, HexInverter};
pub use self::icd2::{parse_icd2, Icd2};
pub use self::lcd_chip::{parse_lcd_chip, LcdChip};
pub use self::lcd_screen::{parse_lcd_screen, LcdScreen};
pub use self::line_decoder::{parse_line_decoder, LineDecoder};
pub use self::mapper::{
    parse_mapper, Huc1Version, Mapper, MapperType, Mbc1Version, Mbc2Version, Mbc3Version,
};
pub use self::mask_rom::{parse_mask_rom, MaskRom};
pub use self::mgb_amp::{parse_mgb_amp, MgbAmp};
pub use self::oxy_u2::{parse_oxy_u2, OxyU2};
pub use self::oxy_u4::{parse_oxy_u4, OxyU4};
pub use self::oxy_u5::{parse_oxy_u5, OxyU5};
pub use self::ram::{parse_ram, Ram};
pub use self::ram_backup::{parse_ram_backup, RamBackup};
pub use self::sgb_rom::{parse_sgb_rom, SgbRom};
pub use self::sram_tsop1_48::parse_sram_tsop1_48;
pub use self::tama::{parse_tama, TamaType};
pub use self::transformer::{parse_transformer, Transformer};

mod accelerometer;
mod agb_amp;
mod agb_cpu;
mod agb_reg;
mod agb_u4;
mod ags_u4;
mod ags_u5;
mod cgb_cpu;
mod cgb_reg;
mod cgb_stamp;
mod cic;
mod coil;
mod crystal;
mod dmg_amp;
mod dmg_reg;
mod dmg_stamp;
mod eeprom;
mod flash;
mod gbs_dol;
mod gbs_reg;
mod gen1_cpu;
mod gen2_cpu;
mod hex_inverter;
mod icd2;
mod lcd_chip;
mod lcd_screen;
mod line_decoder;
mod mapper;
mod mask_rom;
mod mgb_amp;
mod oxy_u2;
mod oxy_u4;
mod oxy_u5;
mod ram;
mod ram_backup;
mod sgb_rom;
mod sram_tsop1_48;
mod tama;
mod transformer;

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

pub fn year2_u16(text: &str) -> Result<u16, String> {
    match u16::from_str(text) {
        Ok(value @ 0..=87) => Ok(value + 2000),
        Ok(value @ 88..=99) => Ok(value + 1900),
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

pub struct MatcherDef<T>(&'static str, fn(Captures) -> Result<T, String>);

impl<T> Copy for MatcherDef<T> {}
impl<T> Clone for MatcherDef<T> {
    fn clone(&self) -> MatcherDef<T> {
        *self
    }
}

#[derive(Clone)]
pub struct Matcher<T> {
    regex: Regex,
    f: fn(Captures) -> Result<T, String>,
}

impl<T> From<MatcherDef<T>> for Matcher<T> {
    fn from(def: MatcherDef<T>) -> Matcher<T> {
        let regex = RegexBuilder::new(def.0)
            .ignore_whitespace(true)
            .build()
            .unwrap();
        Matcher { regex, f: def.1 }
    }
}

impl<T> Matcher<T> {
    pub fn apply(&self, text: &str) -> Option<T> {
        self.regex.captures(text).map(|c| (self.f)(c).unwrap())
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
pub struct MatcherSet<T> {
    matchers: Vec<Matcher<T>>,
    regex_set: RegexSet,
}

impl<T> MatcherSet<T> {
    pub fn new(defs: &[MatcherDef<T>]) -> MatcherSet<T> {
        MatcherSet {
            matchers: defs.iter().copied().map(|def| def.into()).collect(),
            regex_set: RegexSetBuilder::new(defs.iter().map(|m| m.0))
                .ignore_whitespace(true)
                .build()
                .unwrap(),
        }
    }
    pub fn apply(&self, text: &str) -> Option<T> {
        let matches = self.regex_set.matches(text);
        if matches.iter().count() > 1 {
            eprintln!("Warning: multiple matches for {}", text);
        }
        matches.iter().find_map(|m| self.matchers[m].apply(text))
    }
}
