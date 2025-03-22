// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    IResult, Parser as _, branch::alt, bytes::streaming::tag, combinator::opt, error::ParseError,
    sequence::terminated,
};

use super::{
    Crystal, Manufacturer, NomParser, PartDateCode,
    for_nom::{lines2, month1_abc, uppers, year1, year1_month1_abc, year2_week2},
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_32_KIHZ.parse("KDS1H").is_ok());
/// ```
pub static KDS_32_KIHZ: NomParser<Crystal> = NomParser {
    name: "KDS 32 KiHz",
    f: |input| {
        (tag("KDS"), year1_month1_abc)
            .map(|(_, date_code)| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_32_KIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_4_MIHZ_OLD.parse("KDS9807 4.194").is_ok());
/// assert!(parser::kds::KDS_4_MIHZ_OLD.parse("KDS 9803 4.194").is_ok());
/// assert!(parser::kds::KDS_4_MIHZ_OLD.parse("KDS 6F 4.194").is_ok());
/// ```
pub static KDS_4_MIHZ_OLD: NomParser<Crystal> = NomParser {
    name: "KDS 4 MiHz",
    f: |input| {
        lines2(
            terminated(tag("KDS"), opt(nom::character::complete::char(' ')))
                .and(alt((year1_month1_abc, year2_week2))),
            tag("4.194"),
        )
        .map(|((_, date_code), _)| Crystal {
            manufacturer: Some(Manufacturer::Kds),
            frequency: Crystal::FREQ_4_MIHZ,
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_4_MIHZ_NEW.parse("KDS 0102 4.194").is_ok());
/// ```
pub static KDS_4_MIHZ_NEW: NomParser<Crystal> = NomParser {
    name: "KDS 4 MiHz",
    f: |input| {
        lines2(tag("KDS ").and(year2_week2), tag("4.194"))
            .map(|((_, date_code), _)| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_4_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_4_MIHZ_AGS.parse("KDSI 0549 4.194").is_ok());
/// ```
pub static KDS_4_MIHZ_AGS: NomParser<Crystal> = NomParser {
    name: "KDS 4 MiHz",
    f: |input| {
        lines2(tag("KDSI ").and(year2_week2), tag("4.194"))
            .map(|((_, date_code), _)| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_4_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_8_MIHZ.parse("KDS 9841 8.388").is_ok());
/// ```
pub static KDS_8_MIHZ: NomParser<Crystal> = NomParser {
    name: "KDS 8 MiHz",
    f: |input| {
        lines2(tag("KDS ").and(year2_week2), tag("8.388"))
            .map(|((_, date_code), _)| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_8_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_D419_OLD.parse("D419A2").is_ok());
/// ```
pub static KDS_D419_OLD: NomParser<Crystal> = NomParser {
    name: "KDS D419",
    f: |input| {
        tag("D419")
            .and(month1_abc_year1)
            .map(|(_, date_code)| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_4_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_D419_NEW.parse("D419J3I").is_ok());
/// ```
pub static KDS_D419_NEW: NomParser<Crystal> = NomParser {
    name: "KDS D419",
    f: |input| {
        tag("D419")
            .and(month1_abc_year1.and(uppers(1)))
            .map(|(_, (date_code, _))| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_4_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_D838.parse("D838K0I").is_ok());
/// ```
pub static KDS_D838: NomParser<Crystal> = NomParser {
    name: "KDS D838",
    f: |input| {
        tag("D838")
            .and(month1_abc_year1.and(uppers(1)))
            .map(|(_, (date_code, _))| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_8_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::kds::KDS_D209.parse("D209A8").is_ok());
/// ```
pub static KDS_D209: NomParser<Crystal> = NomParser {
    name: "KDS D209",
    f: |input| {
        tag("D209")
            .and(month1_abc_year1)
            .map(|(_, date_code)| Crystal {
                manufacturer: Some(Manufacturer::Kds),
                frequency: Crystal::FREQ_20_MIHZ,
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

fn month1_abc_year1<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, PartDateCode, E> {
    (month1_abc, year1)
        .map(|(month, year)| PartDateCode::YearMonth { year, month })
        .parse(input)
}
