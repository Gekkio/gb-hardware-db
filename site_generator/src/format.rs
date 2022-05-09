use std::borrow::Cow;

fn month_name(month: u8) -> Option<&'static str> {
    match month {
        1 => Some("January"),
        2 => Some("February"),
        3 => Some("March"),
        4 => Some("April"),
        5 => Some("May"),
        6 => Some("June"),
        7 => Some("July"),
        8 => Some("August"),
        9 => Some("September"),
        10 => Some("October"),
        11 => Some("November"),
        12 => Some("December"),
        _ => None,
    }
}

pub fn calendar(year: Option<u16>, month: Option<u8>, week: Option<u8>) -> String {
    let year = year.map(|year| year.to_string()).unwrap_or_default();
    let prefix = month
        .and_then(month_name)
        .map(Cow::Borrowed)
        .or_else(|| week.map(|week| Cow::from(format!("Week {week}"))));
    match prefix {
        Some(prefix) => format!("{prefix}/{year}"),
        _ => year,
    }
}

pub fn calendar_short(year: Option<u16>, month: Option<u8>, week: Option<u8>) -> String {
    let year = year.map(|year| year.to_string()).unwrap_or_default();
    let prefix = month
        .and_then(month_name)
        .map(|month| Cow::Borrowed(&month[..3]))
        .or_else(|| week.map(|week| Cow::from(week.to_string())));
    match prefix {
        Some(prefix) => format!("{prefix}/{year}"),
        _ => year,
    }
}
