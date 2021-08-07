use super::{week2, year2, ChipYearWeek, LabelParser};
use crate::{macros::single_parser, parser::Manufacturer};

pub type DmgReg = ChipYearWeek;

/// ```
/// use gbhwdb_backend::parser::{self, LabelParser};
/// assert!(parser::dmg_reg::sharp_ir3e02().parse("DMG-REG IR3E02 9527 CB").is_ok());
/// assert!(parser::dmg_reg::sharp_ir3e02().parse("DMG-REG IR3E02 9820 n").is_ok());
/// assert!(parser::dmg_reg::sharp_ir3e02().parse("DMG-REG IR3E02 9024 J").is_ok());
/// ```
pub fn sharp_ir3e02() -> &'static impl LabelParser<DmgReg> {
    single_parser!(
        DmgReg,
        r#"^DMG-REG\ IR3E02\ ([0-9]{2})([0-9]{2})\ [a-zA-Z]{1,2}$"#,
        move |c| {
            Ok(DmgReg {
                kind: "IR3E02".to_owned(),
                manufacturer: Some(Manufacturer::Sharp),
                year: Some(year2(&c[1])?),
                week: Some(week2(&c[2])?),
            })
        },
    )
}

pub fn dmg_reg() -> &'static impl LabelParser<DmgReg> {
    sharp_ir3e02()
}
