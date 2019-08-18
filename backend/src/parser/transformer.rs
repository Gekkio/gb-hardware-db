use lazy_static::lazy_static;

use super::{Manufacturer, Matcher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transformer {
    pub kind: String,
    pub manufacturer: Option<Manufacturer>,
}

/// ```
/// # use gbhwdb_backend::parser::parse_transformer;
/// assert!(parse_transformer("84Z7").is_ok());
/// ```
fn mitsumi() -> Matcher<Transformer> {
    Matcher::new(r#"^(84Z7)$"#, move |c| {
        Ok(Transformer {
            kind: c[1].to_owned(),
            manufacturer: Some(Manufacturer::Mitsumi),
        })
    })
}

pub fn parse_transformer(text: &str) -> Result<Transformer, ()> {
    lazy_static! {
        static ref MATCHERS: [Matcher<Transformer>; 1] = [mitsumi(),];
    }
    for matcher in MATCHERS.iter() {
        if let Some(chip) = matcher.apply(text) {
            return Ok(chip);
        }
    }
    Err(())
}
