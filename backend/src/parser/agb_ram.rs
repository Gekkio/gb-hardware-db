use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, Matcher, Year};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgbRam {
    pub kind: Option<String>,
    pub manufacturer: Option<Manufacturer>,
    pub year: Option<Year>,
    pub week: Option<u8>,
}

/// NEC μPD442012A-X
///
/// Source:
///   "NEC data sheet - MOS integrated circuit μPD442012A-X - 2M-bit CMOS static RAM 128k-word by 16-bit extended temperature operation"
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("NEC JAPAN D442012AGY-BB85X-MJH 0037K7027").is_ok());
/// assert!(parse_agb_ram("NEC JAPAN D442012AGY-BC85X-MJH 0330K7043").is_ok());
/// ```
fn nec_upd442012a() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^NEC\ JAPAN\ D442012AGY-(BB|BC|DD)([0-9]{2})X-MJH\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(format!("μPD442012AGY-{version}{access_time}X-MJH", version=&c[1], access_time=&c[2])),
                manufacturer: Some(Manufacturer::Nec),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// NEC μPD442012L-X
///
/// Source:
///   "NEC data sheet - MOS integrated circuit μPD442012L-X - 2M-bit CMOS static RAM 128k-word by 16-bit extended temperature operation"
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("NEC JAPAN D442012LGY-B85X-MJH 0138K7037").is_ok());
/// ```
fn nec_upd442012l() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^NEC\ JAPAN\ D442012LGY-(B|C|D)([0-9]{2})X-MJH\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(format!("μPD442012LGY-{version}{access_time}X-MJH", version=&c[1], access_time=&c[2])),
                manufacturer: Some(Manufacturer::Nec),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Fujitsu MB82D12160
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("JAPAN 82D12160-10FN 0238 M88N").is_ok());
/// ```
fn fujitsu() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^JAPAN\ (82D12160-10FN)\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}[A-Z]$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(c[1].to_owned()),
                manufacturer: Some(Manufacturer::Fujitsu),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// Hynix HY62LF16206A-LT12C
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("Hynix KOREA HY62LF16206A 0223A LT12C").is_ok());
/// ```
fn hynix() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^Hynix\ KOREA\ HY62LF16206A\ ([0-9]{2})([0-9]{2})[A-Z]\ LT12C$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some("HY62LF16206A-LT12C".to_owned()),
                manufacturer: Some(Manufacturer::Hynix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// STMicro M68AS128DL70N6
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("M68AS128 DL70N6 AANFG F6 TWN 8B 414").is_ok());
/// ```
fn st_micro() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^([A-Z]\ )?M68AS128\ DL70N6\ [A-Z]{5}\ F6\ TWN\ [[:alnum:]]{2}\ ([0-9])([0-9]{2})$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some("M68AS128DL70N6".to_owned()),
                manufacturer: Some(Manufacturer::StMicro),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// AMIC LP62S16128BW
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("AMIC LP62S16128BW-70LLTF P4060473FB 0540A").is_ok());
/// ```
fn amic() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^AMIC\ (LP62S16128BW-[0-9]{2}[A-Z]{3,4})\ [[:alnum:]]{10}\ ([0-9]{2})([0-9]{2})[A-Z]$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(c[1].to_owned()),
                manufacturer: Some(Manufacturer::Amic),
                year: Some(year2(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// BSI BS616LV2018/BS616LV2019
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("BSI BS616LV2019TC-70 S31687FZ226013.1 L0335 TAIWAN").is_ok());
/// assert!(parse_agb_ram("BSI BS616LV2018TC-70 S31686-2FY24092.1 L0314 TAIWAN").is_ok());
/// assert!(parse_agb_ram("BSI BS616LV2019TC-70 S31687FZ27050.1 L0336 TAIWAN").is_ok());
/// ```
fn bsi() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^BSI\ (BS616LV201[89]TC-[0-9]{2})\ [[:alnum:]]{5,6}-?[[:alnum:]]{8}(.[0-9])?\ [A-Z]([0-9]{2})([0-9]{2})\ TAIWAN$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(c[1].to_owned()),
                manufacturer: Some(Manufacturer::Bsi),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// Toshiba TC55V200
///
/// ```
/// # use gbhwdb_backend::parser::parse_agb_ram;
/// assert!(parse_agb_ram("K13529 JAPAN 0106 MAD TC55V200 FT-70").is_ok());
/// ```
fn toshiba() -> Matcher<AgbRam> {
    Matcher::new(
        r#"^K13529\ JAPAN\ ([0-9]{2})([0-9]{2})\ MAD\ TC55V200\ (FT-70)$"#,
        move |c| {
            Ok(AgbRam {
                kind: Some(format!("TC55V200{}", &c[3])),
                manufacturer: Some(Manufacturer::Toshiba),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_agb_ram(text: &str) -> Result<AgbRam, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<AgbRam>; 8] = [
            nec_upd442012a(),
            nec_upd442012l(),
            fujitsu(),
            hynix(),
            st_micro(),
            amic(),
            bsi(),
            toshiba()
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
