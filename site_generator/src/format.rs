use std::borrow::Cow;

use gbhwdb_backend::time::{Month, Week};

pub fn calendar(year: Option<u16>, month: Option<Month>, week: Option<Week>) -> String {
    let year = year.map(|year| year.to_string()).unwrap_or_default();
    let prefix = month
        .map(|month| Cow::Borrowed(month.name()))
        .or_else(|| week.map(|week| Cow::from(format!("Week {week}"))));
    match prefix {
        Some(prefix) => format!("{prefix}/{year}"),
        _ => year,
    }
}

pub fn calendar_short(year: Option<u16>, month: Option<Month>, week: Option<Week>) -> String {
    let year = year.map(|year| year.to_string()).unwrap_or_default();
    let prefix = month
        .map(|month| Cow::Borrowed(&month.name()[..3]))
        .or_else(|| week.map(|week| Cow::from(week.to_string())));
    match prefix {
        Some(prefix) => format!("{prefix}/{year}"),
        _ => year,
    }
}
