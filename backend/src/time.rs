use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(into = "u8", try_from = "u8")]
pub struct Week(u8);

impl TryFrom<u8> for Week {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1..=53 => Ok(Week(value)),
            _ => Err(value),
        }
    }
}

impl From<Week> for u8 {
    fn from(week: Week) -> Self {
        week.0
    }
}

impl fmt::Display for Week {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
#[serde(into = "u8", try_from = "u8")]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month as u8
    }
}

impl TryFrom<u8> for Month {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(value),
        }
    }
}

impl Month {
    pub fn name(&self) -> &'static str {
        match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.name(), f)
    }
}
