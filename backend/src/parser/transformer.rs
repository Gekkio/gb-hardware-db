use super::{LabelParser, Manufacturer};
use crate::macros::single_parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transformer {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// # use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::transformer::mitsumi_mgl().parse("82Y7").is_ok());
/// assert!(parser::transformer::mitsumi_mgl().parse("84Z7").is_ok());
/// ```
pub fn mitsumi_mgl() -> &'static impl LabelParser<Transformer> {
    single_parser!(Transformer, r#"^(82Y7|84Z7)$"#, move |c| {
        Ok(Transformer {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
        })
    })
}

pub fn transformer() -> &'static impl LabelParser<Transformer> {
    mitsumi_mgl()
}
