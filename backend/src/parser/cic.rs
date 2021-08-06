use once_cell::sync::OnceCell;

use super::{week2, year2_u16, Matcher, MatcherDef};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cic {
    pub kind: String,
    pub year: Option<u16>,
    pub week: Option<u8>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_cic;
/// assert!(parse_cic("F411A © 1990 Nintendo 9428 a").is_some());
/// ```
fn cic() -> MatcherDef<Cic> {
    MatcherDef(
        r#"^(F411A|F411B|F413A|F413B)\ ©\ (1990|1992)\ Nintendo\ ([0-9]{2})([0-9]{2})\ [A-Za-z]?$"#,
        move |c| {
            Ok(Cic {
                kind: c[1].to_owned(),
                year: Some(year2_u16(&c[3])?),
                week: Some(week2(&c[4])?),
            })
        },
    )
}

pub fn parse_cic(text: &str) -> Option<Cic> {
    static MATCHER: OnceCell<Matcher<Cic>> = OnceCell::new();
    MATCHER.get_or_init(|| cic().into()).apply(text)
}
