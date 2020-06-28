use lazy_static::lazy_static;

use super::{week2, year1, year2, Manufacturer, MatcherDef, MatcherSet, StaticRam};

/// NEC μPD442012A-X
///
/// Source:
///   "NEC data sheet - MOS integrated circuit μPD442012A-X - 2M-bit CMOS static RAM 128k-word by 16-bit extended temperature operation"
///
/// ```
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("NEC JAPAN D442012AGY-BB85X-MJH 0037K7027").is_some());
/// assert!(parse_sram_tsop1_48("NEC JAPAN D442012AGY-BC85X-MJH 0330K7043").is_some());
/// ```
fn nec_upd442012a() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^NEC\ JAPAN\ D442012AGY-(BB|BC|DD)([0-9]{2})X-MJH\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("μPD442012A-X"),
                part: Some(format!(
                    "μPD442012AGY-{version}{access_time}X-MJH",
                    version = &c[1],
                    access_time = &c[2]
                )),
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
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("NEC JAPAN D442012LGY-B85X-MJH 0138K7037").is_some());
/// ```
fn nec_upd442012l() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^NEC\ JAPAN\ D442012LGY-(B|C|D)([0-9]{2})X-MJH\ ([0-9]{2})([0-9]{2})[A-Z][0-9]{4}$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("μPD442012L-X"),
                part: Some(format!(
                    "μPD442012LGY-{version}{access_time}X-MJH",
                    version = &c[1],
                    access_time = &c[2]
                )),
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
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("JAPAN 82D12160-10FN 0238 M88N").is_some());
/// ```
fn fujitsu_mb82d12160() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^JAPAN\ 82D12160-10FN\ ([0-9]{2})([0-9]{2})\ [A-Z][0-9]{2}[A-Z]$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("MB82D12160"),
                part: Some("MB82D12160-10FN".to_owned()),
                manufacturer: Some(Manufacturer::Fujitsu),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// Hynix HY62LF16206A
///
/// Source:
///   "hynix HY62LF16206A-LT12C 128kx16bit full CMOS SRAM"
///
/// ```
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("Hynix KOREA HY62LF16206A 0223A LT12C").is_some());
/// ```
fn hynix_hy62lf16206a() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^Hynix\ KOREA\ HY62LF16206A\ ([0-9]{2})([0-9]{2})[A-Z]\ LT12C$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("HY62LF16206A"),
                part: Some("HY62LF16206A-LT12C".to_owned()),
                manufacturer: Some(Manufacturer::Hynix),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// STMicro M68AS128
///
/// ```
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("M68AS128 DL70N6 AANFG F6 TWN 8B 414").is_some());
/// ```
fn st_micro_m68as128dl70n6() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^([A-Z]\ )?M68AS128\ DL70N6\ [A-Z]{5}\ F6\ TWN\ [[:alnum:]]{2}\ ([0-9])([0-9]{2})$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("M68AS128"),
                part: Some("M68AS128DL70N6".to_owned()),
                manufacturer: Some(Manufacturer::StMicro),
                year: Some(year1(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// AMIC LP62S16128BW-T
///
/// Source:
///   "AMIC LP62S16128BW-T series - 128k x 16 bit low voltage CMOS SRAM"
///
/// ```
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("AMIC LP62S16128BW-70LLTF P4060473FB 0540A").is_some());
/// ```
fn amic_lp62s16128bw() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^AMIC\ LP62S16128BW-([0-9]{2})(LLT|LLTF)\ [[:alnum:]]{10}\ ([0-9]{2})([0-9]{2})[A-Z]$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("LP62S16128BW-T"),
                part: Some(format!(
                    "LP62S16128BW-{access_time}{version}",
                    access_time = &c[1],
                    version = &c[2],
                )),
                manufacturer: Some(Manufacturer::Amic),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// BSI BS616LV2018
///
/// Source:
///   "BSI BS616LV2018 - Very Low Power/Voltage CMOS SRAM 128k x 16 bit"
///
/// ```
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("BSI BS616LV2018TC-70 S31686-2FY24092.1 L0314 TAIWAN").is_some());
/// assert!(parse_sram_tsop1_48("BSI BS616LV2018TC-70 S31686-2FY10121.1 L0230 TAIWAN").is_some());
/// ```
fn bsi_bs616lv2018() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^BSI\ BS616LV2018(TC|TI)-([0-9]{2})\ S31686-2FY[0-9]{5}.1\ [A-Z]([0-9]{2})([0-9]{2})\ TAIWAN$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("BS616LV2018"),
                part: Some(format!(
                    "BS616LV2018{version}-{access_time}",
                    version = &c[1],
                    access_time = &c[2],
                )),
                manufacturer: Some(Manufacturer::Bsi),
                year: Some(year2(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

/// BSI BS616LV2019
///
/// Source:
///   "BSI BS616LV2019 - Very Low Power CMOS SRAM 128k x 16 bit"
///
/// ```
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("BSI BS616LV2019TC-70 S31687FZ26013.1 L0335 TAIWAN").is_some());
/// assert!(parse_sram_tsop1_48("BSI BS616LV2019TC-70 S31687FZ27050.1 L0336 TAIWAN").is_some());
/// assert!(parse_sram_tsop1_48("BSI BS616LV2019TC-70 S31687FZ31012.1 L0410 TAIWAN").is_some());
/// ```
fn bsi_bs616lv2019() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^BSI\ BS616LV2019(TC|TI)-([0-9]{2})\ S31687FZ[0-9]{5}.1\ [A-Z]([0-9]{2})([0-9]{2})\ TAIWAN$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("BS616LV2019"),
                part: Some(format!(
                    "BS616LV2019{version}-{access_time}",
                    version = &c[1],
                    access_time = &c[2],
                )),
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
/// # use gbhwdb_backend::parser::parse_sram_tsop1_48;
/// assert!(parse_sram_tsop1_48("K13529 JAPAN 0106 MAD TC55V200 FT-70").is_some());
/// ```
fn toshiba_tc55v200() -> MatcherDef<StaticRam> {
    MatcherDef(
        r#"^K13529\ JAPAN\ ([0-9]{2})([0-9]{2})\ MAD\ TC55V200\ FT-([0-9]{2})$"#,
        move |c| {
            Ok(StaticRam {
                family: Some("TC55V200"),
                part: Some(format!("TC55V200FT-{access_time}", access_time = &c[3],)),
                manufacturer: Some(Manufacturer::Toshiba),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn parse_sram_tsop1_48(text: &str) -> Option<StaticRam> {
    lazy_static! {
        static ref MATCHER: MatcherSet<StaticRam> = MatcherSet::new(&[
            nec_upd442012a(),
            nec_upd442012l(),
            fujitsu_mb82d12160(),
            hynix_hy62lf16206a(),
            st_micro_m68as128dl70n6(),
            amic_lp62s16128bw(),
            bsi_bs616lv2018(),
            bsi_bs616lv2019(),
            toshiba_tc55v200()
        ]);
    }
    MATCHER.apply(text)
}
