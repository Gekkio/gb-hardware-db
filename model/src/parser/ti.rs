// SPDX-FileCopyrightText: Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use nom::{Parser as _, bytes::streaming::tag, sequence::terminated};

use super::{
    GenericPart, Manufacturer, Mapper, MapperChip, NomParser,
    for_nom::{alnum_uppers, lines3, uppers, year1_month1_123abc},
};

/// TI SN74LV2416 supervisor
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::ti::TI_SN74LV2416.parse("LV2416 17M A23D").is_ok());
/// assert!(parser::ti::TI_SN74LV2416.parse("LV2416 13M A8R3").is_ok());
/// assert!(parser::ti::TI_SN74LV2416.parse("LV2416 0CM A73E").is_ok());
/// ```
pub static TI_SN74LV2416: NomParser<GenericPart> = NomParser {
    name: "TI SN74LV2416",
    f: |input| {
        lines3(
            tag("LV2416"),
            terminated(year1_month1_123abc, tag("M")),
            tag("A").and(alnum_uppers(3)),
        )
        .map(|(_, date_code, _)| GenericPart {
            kind: "SN74LV2416".to_owned(),
            manufacturer: Some(Manufacturer::TexasInstruments),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};

/// Texas Instruments MBC5 (QFP-32)
///
/// ```
/// use gbhwdb_model::parser::{self, LabelParser};
/// assert!(parser::ti::TI_MBC5.parse("11CH8VT MBC5 2417").is_ok());
/// ```
pub static TI_MBC5: NomParser<Mapper> = NomParser {
    name: "TI MBC5",
    f: |input| {
        lines3(
            terminated(year1_month1_123abc, (uppers(1), alnum_uppers(3), tag("T"))),
            tag("MBC5"),
            tag("2417"),
        )
        .map(|(date_code, _, _)| Mapper {
            kind: MapperChip::Mbc5,
            manufacturer: Some(Manufacturer::TexasInstruments),
            date_code: Some(date_code),
        })
        .parse(input)
    },
};
