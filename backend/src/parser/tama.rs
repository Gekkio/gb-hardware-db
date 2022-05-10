use super::{week2, year2, LabelParser, Year};
use crate::{
    macros::{multi_parser, single_parser},
    time::Week,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tama {
    pub tama_type: TamaType,
    pub year: Option<Year>,
    pub week: Option<Week>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TamaType {
    Tama5,
    Tama6,
    Tama7,
}

/// TAMA5
///
/// ```
/// # use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::tama::tama5().parse("TAMA5 9726 EAD1").is_ok());
/// ```
pub fn tama5() -> &'static impl LabelParser<Tama> {
    single_parser!(
        Tama,
        r#"^TAMA5\ ([0-9]{2})([0-9]{2})\ EA[A-Z]1$"#,
        move |c| {
            Ok(Tama {
                tama_type: TamaType::Tama5,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        }
    )
}

/// TAMA6
///
/// ```
/// # use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::tama::tama6().parse("TAMA6 JAPAN 47C243M FV61 9751H").is_ok());
/// ```
pub fn tama6() -> &'static impl LabelParser<Tama> {
    single_parser!(
        Tama,
        r#"^TAMA6\ JAPAN\ 47C243M\ FV61\ ([0-9]{2})([0-9]{2})H$"#,
        move |c| {
            Ok(Tama {
                tama_type: TamaType::Tama6,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

/// TAMA7 ROM
///
/// ```
/// # use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::tama::tama7().parse("TAMA7 B9748 43913A TAIWAN").is_ok());
/// ```
pub fn tama7() -> &'static impl LabelParser<Tama> {
    single_parser!(
        Tama,
        r#"^TAMA7\ [A-Z]([0-9]{2})([0-9]{2})\ [0-9]{5}[A-Z]\ TAIWAN$"#,
        move |c| {
            Ok(Tama {
                tama_type: TamaType::Tama7,
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn tama() -> &'static impl LabelParser<Tama> {
    multi_parser!(Tama, tama5(), tama6(), tama7())
}
