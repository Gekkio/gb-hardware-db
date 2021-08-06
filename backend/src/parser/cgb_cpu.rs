use once_cell::sync::OnceCell;

use super::{week2, year2_u16, Matcher, MatcherDef};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CgbCpu {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cgb_cpu;
/// assert!(parse_cgb_cpu("CPU CGB B Ⓜ © 1998 Nintendo JAPAN 9842 I").is_some());
/// ```
fn cgb_cpu() -> MatcherDef<CgbCpu> {
    MatcherDef(
        r#"^(CPU\ CGB(\ [A-E])?)\ Ⓜ\ ©\ (1998|2000)\ Nintendo\ JAPAN\ ([0-9]{2})([0-9]{2})\ [A-Z]{1,2}$"#,
        move |c| {
            Ok(CgbCpu {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[4])?),
                week: Some(week2(&c[5])?),
            })
        },
    )
}

pub fn parse_cgb_cpu(text: &str) -> Option<CgbCpu> {
    static MATCHER: OnceCell<Matcher<CgbCpu>> = OnceCell::new();
    MATCHER.get_or_init(|| cgb_cpu().into()).apply(text)
}
