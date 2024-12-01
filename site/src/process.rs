// SPDX-FileCopyrightText: 2017-2024 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::{
    parser::Year,
    time::{Jun, Month, Week},
};

pub mod part;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct DateCode {
    pub year: Option<u16>,
    pub month: Option<Month>,
    pub jun: Option<Jun>,
    pub week: Option<Week>,
}

impl DateCode {
    pub fn year_month(year: Option<u16>, month: Option<Month>) -> Self {
        DateCode {
            year,
            month,
            ..DateCode::default()
        }
    }
    pub fn loose_year_month(
        year_hint: Option<u16>,
        year: Option<Year>,
        month: Option<Month>,
    ) -> Self {
        DateCode {
            year: to_full_year(year_hint, year),
            month,
            ..DateCode::default()
        }
    }
    pub fn loose_year_week(year_hint: Option<u16>, year: Option<Year>, week: Option<Week>) -> Self {
        DateCode {
            year: to_full_year(year_hint, year),
            week,
            ..DateCode::default()
        }
    }
    pub fn calendar(&self) -> Option<String> {
        match (self.year, self.month, self.week) {
            (Some(year), Some(month), _) => match self.jun {
                Some(jun) => {
                    let range = jun.range(year, month);
                    Some(format!(
                        "{month} {from}-{to}/{year}",
                        month = &month.name()[..3],
                        from = range.start(),
                        to = range.end(),
                    ))
                }
                None => Some(format!("{month}/{year}")),
            },
            (Some(year), _, Some(week)) => Some(format!("Week {week}/{year}")),
            (Some(year), _, _) => Some(year.to_string()),
            _ => None,
        }
    }
}

impl From<(Option<u16>, Option<Week>)> for DateCode {
    fn from((year, week): (Option<u16>, Option<Week>)) -> Self {
        DateCode {
            year,
            week,
            ..DateCode::default()
        }
    }
}

impl From<(Option<u16>, Option<Month>)> for DateCode {
    fn from((year, month): (Option<u16>, Option<Month>)) -> Self {
        DateCode {
            year,
            month,
            ..DateCode::default()
        }
    }
}

pub fn to_full_year(year_hint: Option<u16>, part_year: Option<Year>) -> Option<u16> {
    (match (year_hint, part_year) {
        (_, Some(Year::Full(year))) => Some(year),
        (Some(year_hint), Some(Year::Partial(year))) => Some(guess_full_year(year_hint, year)),
        _ => None,
    })
    .map(|year| {
        assert!(
            (1988..2010).contains(&year),
            "suspicious year {year} calculated from {year_hint:?}:{part_year:?}"
        );
        year
    })
}

pub fn guess_full_year(hint: u16, partial_year: u8) -> u16 {
    let partial_year = u16::from(partial_year);
    let decades = [1980, 1990, 2000];
    decades
        .into_iter()
        .map(|decade: u16| decade + partial_year)
        .min_by_key(|&year| hint.abs_diff(year))
        .unwrap_or(0)
}

#[test]
fn test_guess_full_year() {
    assert_eq!(1992, guess_full_year(1992, 2));
    assert_eq!(1989, guess_full_year(1989, 9));
    assert_eq!(1990, guess_full_year(1990, 0));
    assert_eq!(1999, guess_full_year(1998, 9));
    assert_eq!(2000, guess_full_year(2005, 0));
}
