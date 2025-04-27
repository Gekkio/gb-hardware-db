// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{
    Parser as _,
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{char, one_of},
    combinator::{opt, recognize},
    sequence::{preceded, terminated},
};

use super::{
    GenericPart, Manufacturer, Mapper, MapperChip, NomParser, PartDateCode,
    for_nom::{alnum_uppers, digits, lines2, lines3, month1_123abc, year1, year1_week2},
};

/// ROHM ??9853 EEPROM (SOP-8)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9853.parse("9853 2A46").is_ok());
/// assert!(parser::rohm::ROHM_9853.parse("9853 6912").is_ok());
/// ```
pub static ROHM_9853: NomParser<GenericPart> = NomParser {
    name: "ROHM 9853",
    f: |input| {
        lines2(tag("9853"), (year1, month1_123abc, digits(2)))
            .map(|(kind, (year, month, _))| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM ??9854 EEPROM (SOP-8)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9854.parse("9854 5S95W").is_ok());
/// ```
pub static ROHM_9854: NomParser<GenericPart> = NomParser {
    name: "ROHM 9854",
    f: |input| {
        lines2(tag("9854"), (year1, alnum_uppers(1), digits(2), char('W')))
            .map(|(kind, (year, _, _, _))| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};

/// ROHM BA6129 supervisor
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BA6129.parse("6129 4803").is_ok());
/// assert!(parser::rohm::ROHM_BA6129.parse("6129A 6194").is_ok());
/// ```
pub static ROHM_BA6129: NomParser<GenericPart> = NomParser {
    name: "ROHM BA6129",
    f: |input| {
        (
            alt((tag("6129A"), tag("6129"))),
            char(' '),
            year1,
            alnum_uppers(1),
            digits(2),
        )
            .map(|(kind, _, year, _, _)| GenericPart {
                kind: format!("BA{kind}"),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::Year { year }),
            })
            .parse(input)
    },
};

/// ROHM BA6735 supervisor
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BA6735.parse("6735 8C19").is_ok());
/// ```
pub static ROHM_BA6735: NomParser<GenericPart> = NomParser {
    name: "ROHM BA6735",
    f: |input| {
        (tag("6735"), char(' '), year1, month1_123abc, digits(2))
            .map(|(_, _, year, month, _)| GenericPart {
                kind: "BA6735".to_owned(),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM ??9750
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9750.parse("9750A 1581").is_ok());
/// assert!(parser::rohm::ROHM_9750.parse("9750B 2A69").is_ok());
/// ```
pub static ROHM_9750: NomParser<GenericPart> = NomParser {
    name: "ROHM 9750",
    f: |input| {
        (
            recognize(tag("9750").and(one_of("AB"))),
            char(' '),
            year1,
            month1_123abc,
            digits(2),
        )
            .map(|(kind, _, year, month, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM ??9753
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_9753.parse("9753 4862").is_ok());
/// ```
pub static ROHM_9753: NomParser<GenericPart> = NomParser {
    name: "ROHM 9753",
    f: |input| {
        (tag("9753"), char(' '), year1, month1_123abc, digits(2))
            .map(|(kind, _, year, month, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(PartDateCode::YearMonth { year, month }),
            })
            .parse(input)
    },
};

/// ROHM BH7835AFS AGB amplifier
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BH7835AFS.parse("BH7835AFS 337 T22").is_ok());
/// ```
pub static ROHM_BH7835AFS: NomParser<GenericPart> = NomParser {
    name: "ROHM BH7835AFS",
    f: |input| {
        (
            tag("BH7835AFS"),
            char(' '),
            year1_week2,
            char(' '),
            alnum_uppers(1),
            digits(2),
        )
            .map(|(kind, _, date_code, _, _, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ROHM ICD2-R
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_ICD2_R.parse("Nintendo ICD2-R 435 179").is_ok());
/// ```
pub static ROHM_ICD2_R: NomParser<GenericPart> = NomParser {
    name: "ROHM ICD2_R",
    f: |input| {
        (
            preceded(tag("Nintendo "), tag("ICD2-R")),
            char(' '),
            year1_week2,
            char(' '),
            alnum_uppers(1),
            digits(2),
        )
            .map(|(kind, _, date_code, _, _, _)| GenericPart {
                kind: String::from(kind),
                manufacturer: Some(Manufacturer::Rohm),
                date_code: Some(date_code),
            })
            .parse(input)
    },
};

/// ROHM MBC3 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_MBC3.parse("MBC3 BU3631K 802 127").is_ok());
/// ```
pub static ROHM_MBC3: NomParser<Mapper> = NomParser {
    name: "ROHM MBC3",
    f: |input| {
        lines3(
            tag("MBC3"),
            tag("BU3631K"),
            terminated(year1_week2, (tag(" "), alnum_uppers(1), digits(2))),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc3,
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM MBC3A (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_MBC3A.parse("MBC-3 A BU3632K 004 H64").is_ok());
/// ```
pub static ROHM_MBC3A: NomParser<Mapper> = NomParser {
    name: "ROHM MBC3A",
    f: |input| {
        lines3(
            tag("MBC-3 A"),
            tag("BU3632K"),
            terminated(year1_week2, (tag(" "), alnum_uppers(1), digits(2))),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc3A,
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM MBC3B (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_MBC3B.parse("MBC-3 B BU3634K 135 H48").is_ok());
/// ```
pub static ROHM_MBC3B: NomParser<Mapper> = NomParser {
    name: "ROHM MBC3B",
    f: |input| {
        lines3(
            tag("MBC-3 B"),
            tag("BU3634K"),
            terminated(year1_week2, (tag(" "), alnum_uppers(1), digits(2))),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc3B,
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM MBC30 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_MBC30.parse("MBC-30 BU3633AK 046 175").is_ok());
/// ```
pub static ROHM_MBC30: NomParser<Mapper> = NomParser {
    name: "ROHM MBC30",
    f: |input| {
        lines3(
            tag("MBC-30"),
            tag("BU3633AK"),
            terminated(year1_week2, (tag(" "), alnum_uppers(1), digits(2))),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc30,
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM MBC5 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_MBC5.parse("MBC5 BU3650K 229 H51").is_ok());
/// assert!(parser::rohm::ROHM_MBC5.parse("MBC-5 BU3650K 049 186").is_ok());
/// ```
pub static ROHM_MBC5: NomParser<Mapper> = NomParser {
    name: "ROHM MBC5",
    f: |input| {
        lines3(
            tag("MBC5").or(tag("MBC-5")),
            tag("BU3650K"),
            terminated(year1_week2, (tag(" "), alnum_uppers(1), digits(2))),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc5,
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM MBC7 (QFP-56)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_MBC7.parse("MBC-7 BU3667KS 041 170").is_ok());
/// ```
pub static ROHM_MBC7: NomParser<Mapper> = NomParser {
    name: "ROHM MBC7",
    f: |input| {
        lines3(
            tag("MBC-7"),
            tag("BU3667KS"),
            terminated(year1_week2, (tag(" "), alnum_uppers(1), digits(2))),
        )
        .map(|(_, _, date_code)| Mapper {
            kind: MapperChip::Mbc7,
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM BR62256F (SOP-28, 4.5-5.5V)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BR62256F.parse("BR62256F-70LL 817 126").is_ok());
/// assert!(parser::rohm::ROHM_BR62256F.parse("BR62256F-70LL 845 131A").is_ok());
/// assert!(parser::rohm::ROHM_BR62256F.parse("BR62256F-70LL 031 150NA").is_ok());
/// ```
pub static ROHM_BR62256F: NomParser<GenericPart> = NomParser {
    name: "ROHM BR62256F",
    f: |input| {
        lines2(
            tag("BR62256F-70LL"),
            terminated(
                year1_week2,
                (
                    tag(" "),
                    digits(3),
                    opt(alt((
                        nom::bytes::complete::tag("NA"),
                        nom::bytes::complete::tag("A"),
                    ))),
                ),
            ),
        )
        .map(|(kind, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM BR6265BF (SOP-28, 4.5-5.5V, 64 Kibit / 8 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_BR6265BF.parse("BR6265BF-10SL 737 189N").is_ok());
/// ```
pub static ROHM_BR6265BF: NomParser<GenericPart> = NomParser {
    name: "ROHM BR6265BF",
    f: |input| {
        lines2(
            tag("BR6265BF-10SL"),
            terminated(year1_week2, (tag(" "), digits(3), tag("N"))),
        )
        .map(|(kind, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM XLJ6265AF (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_XLJ6265AF.parse("XLJ6265AF-10SL 437 159").is_ok());
/// ```
pub static ROHM_XLJ6265AF: NomParser<GenericPart> = NomParser {
    name: "ROHM XLJ6265AF",
    f: |input| {
        lines2(
            tag("XLJ6265AF-10SL"),
            terminated(year1_week2, (tag(" "), digits(3))),
        )
        .map(|(kind, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// ROHM XLJ6265BF (SOP-28, 5V, 64 Kibit / 8 KiB)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::rohm::ROHM_XLJ6265BF.parse("XLJ6265BF-10SL 640 171N").is_ok());
/// ```
pub static ROHM_XLJ6265BF: NomParser<GenericPart> = NomParser {
    name: "ROHM XLJ6265BF",
    f: |input| {
        lines2(
            tag("XLJ6265BF-10SL"),
            terminated(year1_week2, (tag(" "), digits(3), tag("N"))),
        )
        .map(|(kind, date_code)| GenericPart {
            kind: String::from(kind),
            manufacturer: Some(Manufacturer::Rohm),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
