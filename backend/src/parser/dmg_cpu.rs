use lazy_static::lazy_static;

use super::{week2, year2_u16, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DmgCpuKind {
    Original,
    A,
    B,
    C,
    BlobB,
    BlobC,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DmgCpu {
    pub kind: DmgCpuKind,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_cpu;
/// assert!(parse_dmg_cpu("DMG-CPU LR35902 8907 D").is_ok());
/// ```
fn dmg_cpu_lr35902() -> Matcher<DmgCpu> {
    Matcher::new(
        r#"^DMG-CPU\ LR35902\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(DmgCpu {
                kind: DmgCpuKind::Original,
                year: Some(year2_u16(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_cpu;
/// assert!(parse_dmg_cpu("DMG-CPU © 1989 Nintendo JAPAN 8913 D").is_ok());
/// assert!(parse_dmg_cpu("DMG-CPU A © 1989 Nintendo JAPAN 8937 D").is_ok());
/// assert!(parse_dmg_cpu("DMG-CPU B © 1989 Nintendo JAPAN 9207 D").is_ok());
/// assert!(parse_dmg_cpu("DMG-CPU C © 1989 Nintendo JAPAN 9835 D").is_ok());
/// ```
fn dmg_cpu() -> Matcher<DmgCpu> {
    Matcher::new(
        r#"^DMG-CPU(\ [ABC])?\ ©\ 1989\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]$"#,
        move |c| {
            Ok(DmgCpu {
                kind: (match c.get(1).map(|m| m.as_str()) {
                    Some(" A") => Ok(DmgCpuKind::A),
                    Some(" B") => Ok(DmgCpuKind::B),
                    Some(" C") => Ok(DmgCpuKind::C),
                    Some(text) => Err(format!("Invalid DMG-CPU part name: {}", text)),
                    _ => Ok(DmgCpuKind::Original),
                })?,
                year: Some(year2_u16(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

fn dmg_cpu_deprecated() -> Matcher<DmgCpu> {
    Matcher::new(
        r#"^DMG-CPU(\ [A-B])?\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(DmgCpu {
                kind: (match c.get(1).map(|m| m.as_str()) {
                    Some(" A") => Ok(DmgCpuKind::A),
                    Some(" B") => Ok(DmgCpuKind::B),
                    Some(text) => Err(format!("Invalid DMG-CPU part name: {}", text)),
                    _ => Ok(DmgCpuKind::Original),
                })?,
                year: Some(year2_u16(&c[2])?),
                week: Some(week2(&c[3])?),
            })
        },
    )
}

/// ```
/// # use gbhwdb_backend::parser::parse_dmg_cpu;
/// assert!(parse_dmg_cpu("B").is_ok());
/// assert!(parse_dmg_cpu("C").is_ok());
/// ```
fn dmg_cpu_blob() -> Matcher<DmgCpu> {
    Matcher::new(r#"^[BC]$"#, move |c| {
        Ok(DmgCpu {
            kind: (match &c[0] {
                "B" => Ok(DmgCpuKind::BlobB),
                "C" => Ok(DmgCpuKind::BlobC),
                text => Err(format!("Invalid DMG-CPU part name: {}", text)),
            })?,
            year: None,
            week: None,
        })
    })
}

pub fn parse_dmg_cpu(text: &str) -> Result<DmgCpu, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<DmgCpu>; 4] = [
            dmg_cpu(),
            dmg_cpu_blob(),
            dmg_cpu_lr35902(),
            dmg_cpu_deprecated()
        ];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
