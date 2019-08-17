use regex::{Captures, Regex, RegexBuilder};
use std::str::FromStr;

pub use self::accelerometer::{parse_accelerometer, Accelerometer};
pub use self::crystal::{parse_crystal, Crystal};
pub use self::eeprom::{parse_eeprom, Eeprom};
pub use self::flash::{parse_flash, Flash};
pub use self::line_decoder::{parse_line_decoder, LineDecoder};
pub use self::mapper::{
    parse_mapper, Huc1Version, Mapper, MapperType, Mbc1Version, Mbc2Version, Mbc3Version,
};
pub use self::mask_rom::{parse_mask_rom, MaskRom};
pub use self::ram::{parse_ram, Ram};
pub use self::ram_backup::{parse_ram_backup, RamBackup};
pub use self::tama::{parse_tama, TamaType};

mod accelerometer;
mod crystal;
mod eeprom;
mod flash;
mod line_decoder;
mod mapper;
mod mask_rom;
mod ram;
mod ram_backup;
mod tama;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Manufacturer {
    Analog,
    AtT,
    Bsi,
    Fujitsu,
    Hudson,
    Hyundai,
    Kds,
    Lgs,
    LsiLogic,
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

pub struct Matcher<T> {
    regex: Regex,
    f: Box<dyn Fn(Captures) -> Result<T, String> + Sync>,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(Captures) -> Result<T, String> + Sync + 'static>(
        regex: &'static str,
        f: F,
    ) -> Matcher<T> {
        let regex = RegexBuilder::new(regex)
            .ignore_whitespace(true)
            .build()
            .unwrap();
        Matcher {
            regex,
            f: Box::new(f),
        }
    }
    pub fn apply(&self, text: &str) -> Option<T> {
        self.regex.captures(text).map(|c| (self.f)(c).unwrap())
    }
}
