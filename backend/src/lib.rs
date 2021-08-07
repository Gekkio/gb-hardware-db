pub mod config;
pub mod input;
pub mod parser;

#[macro_use]
pub(crate) mod macros {
    macro_rules! single_parser {
        ($t:ty, $re:literal, $f:expr $(,)?) => {{
            static PARSER: once_cell::sync::OnceCell<crate::parser::SingleParser<$t>> =
                once_cell::sync::OnceCell::new();
            PARSER.get_or_init(|| crate::parser::SingleParser::compile($re, $f))
        }};
    }
    pub(crate) use single_parser;

    macro_rules! multi_parser {
        ($t:ty, $($m:expr),+ $(,)?) => {{
            static PARSER: once_cell::sync::OnceCell<crate::parser::MultiParser<$t>> =
                once_cell::sync::OnceCell::new();
            PARSER.get_or_init(|| {
                use crate::parser::LabelParser;
                let parsers: Vec<&'static dyn LabelParser<$t>> = vec![$($m),+];
                let parsers: Vec<&'static crate::parser::SingleParser<$t>> = parsers.into_iter().flat_map(|p| p.parsers()).collect();
                crate::parser::MultiParser::compile(parsers)
            })
        }};
    }
    pub(crate) use multi_parser;
}
