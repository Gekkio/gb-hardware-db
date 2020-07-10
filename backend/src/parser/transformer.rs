use lazy_static::lazy_static;

use super::{Manufacturer, Matcher, MatcherDef};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transformer {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_transformer;
/// assert!(parse_transformer("82Y7").is_some());
/// assert!(parse_transformer("84Z7").is_some());
/// ```
fn mitsumi() -> MatcherDef<Transformer> {
    MatcherDef(r#"^(82Y7|84Z7)$"#, move |c| {
        Ok(Transformer {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
        })
    })
}

pub fn parse_transformer(text: &str) -> Option<Transformer> {
    lazy_static! {
        static ref MATCHER: Matcher<Transformer> = mitsumi().into();
    }
    MATCHER.apply(text)
}
