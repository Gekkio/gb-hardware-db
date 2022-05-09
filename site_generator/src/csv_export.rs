use anyhow::Error;
use std::{borrow::Cow, io, marker::PhantomData};

use crate::legacy::{
    console::{
        LegacyDmgJackBoard, LegacyDmgLcdBoard, LegacyDmgMainboard, LegacyDmgMetadata,
        LegacyDmgPowerBoard, LegacyLcdPanel,
    },
    LegacyChip, LegacySubmission,
};

pub trait ToCsv: Sized {
    fn csv_builder() -> Builder<Self>;
}

impl ToCsv for LegacyDmgMetadata {
    fn csv_builder() -> Builder<Self> {
        Builder::<Self>::new()
            .add("color", |m| (&m.color).csv())
            .add("calendar_short", |m| {
                calendar_short(m.year, m.month, None).csv()
            })
            .add("calendar", |m| calendar(m.year, m.month, None).csv())
            .add("year", |m| m.year.csv())
            .add("month", |m| m.month.csv())
            .nest(
                "mainboard",
                |m| Some(&m.mainboard),
                || {
                    Builder::<LegacyDmgMainboard>::new()
                        .add("type", |m| (&m.kind).csv())
                        .add("extra_label", |m| (&m.extra_label).csv())
                        .add("stamp", |m| (&m.stamp).csv())
                        .add("circled_letters", |m| (&m.circled_letters).csv())
                },
            )
            .nest("cpu", |m| m.mainboard.cpu.as_ref(), chip)
            .nest("work_ram", |m| m.mainboard.work_ram.as_ref(), chip)
            .nest("video_ram", |m| m.mainboard.video_ram.as_ref(), chip)
            .nest("amplifier", |m| m.mainboard.amplifier.as_ref(), chip)
            .nest("crystal", |m| m.mainboard.crystal.as_ref(), chip)
            .nest(
                "lcd_board",
                |m| m.lcd_board.as_ref(),
                || {
                    Builder::<LegacyDmgLcdBoard>::new()
                        .add("type", |b| (&b.kind).csv())
                        .add("circled_letters", |b| (&b.circled_letters).csv())
                        .add("stamp", |b| (&b.stamp).csv())
                        .add("calendar_short", |b| {
                            calendar_short(b.year, b.month, None).csv()
                        })
                        .add("calendar", |b| calendar(b.year, b.month, None).csv())
                        .add("year", |b| b.year.csv())
                        .add("month", |b| b.month.csv())
                },
            )
            .nest(
                "lcd_panel",
                |m| m.lcd_board.as_ref().and_then(|b| b.lcd_panel.as_ref()),
                || {
                    Builder::<LegacyLcdPanel>::new()
                        .add("label", |p| (&p.label).csv())
                        .add("calendar_short", |p| {
                            calendar_short(p.year, p.month, None).csv()
                        })
                        .add("calendar", |p| calendar(p.year, p.month, None).csv())
                },
            )
            .nest(
                "column_driver",
                |m| {
                    m.lcd_board
                        .as_ref()
                        .and_then(|b| b.lcd_panel.as_ref().and_then(|p| p.column_driver.as_ref()))
                },
                chip,
            )
            .nest(
                "row_driver",
                |m| {
                    m.lcd_board
                        .as_ref()
                        .and_then(|b| b.lcd_panel.as_ref().and_then(|p| p.row_driver.as_ref()))
                },
                chip,
            )
            .nest(
                "regulator",
                |m| m.lcd_board.as_ref().and_then(|b| b.regulator.as_ref()),
                chip,
            )
            .nest(
                "power_board",
                |m| m.power_board.as_ref(),
                || {
                    Builder::<LegacyDmgPowerBoard>::new()
                        .add("type", |b| (&b.kind).csv())
                        .add("label", |b| (&b.label).csv())
                        .add("calendar_short", |b| {
                            calendar_short(b.year, b.month, None).csv()
                        })
                        .add("calendar", |b| calendar(b.year, b.month, None).csv())
                        .add("year", |b| b.year.csv())
                        .add("month", |b| b.month.csv())
                },
            )
            .nest(
                "jack_board",
                |m| m.jack_board.as_ref(),
                || {
                    Builder::<LegacyDmgJackBoard>::new()
                        .add("type", |b| (&b.kind).csv())
                        .add("extra_label", |b| (&b.extra_label).csv())
                },
            )
    }
}

pub fn write_console_submission_csv<W, M, P>(
    writer: W,
    submissions: &[LegacySubmission<M, P>],
) -> Result<(), Error>
where
    W: io::Write,
    M: ToCsv,
{
    let mut w = csv::Writer::from_writer(writer);

    let header = Builder::<LegacySubmission<M, P>>::new()
        .add("code", |s| (&s.code).csv())
        .add("title", |s| (&s.title).csv())
        .add("slug", |s| (&s.slug).csv())
        .add("url", |s| {
            format!(
                "https://gbhwdb.gekkio.fi/consoles/{}/{}.html",
                s.code, s.slug
            )
            .csv()
        })
        .add("contributor", |s| (&s.contributor).csv());

    let meta = M::csv_builder();

    w.write_record(header.fields().chain(meta.fields()))?;
    for s in submissions {
        w.write_record(header.record(s).chain(meta.record(&s.metadata)))?;
    }
    w.flush()?;

    Ok(())
}

fn chip() -> Builder<LegacyChip> {
    Builder::<LegacyChip>::new()
        .add("kind", |c| (&c.kind).csv())
        .add("label", |c| (&c.label).csv())
        .add("manufacturer", |c| (&c.manufacturer).csv())
        .add("calendar_short", |c| {
            calendar_short(c.year, c.month, c.week).csv()
        })
        .add("calendar", |c| calendar(c.year, c.month, c.week).csv())
        .add("year", |c| c.year.csv())
        .add("month", |c| c.month.csv())
        .add("week", |c| c.week.csv())
}

trait Field<'a> {
    fn csv(self) -> Cow<'a, str>;
}

impl Field<'static> for String {
    fn csv(self) -> Cow<'static, str> {
        self.into()
    }
}

impl<'a> Field<'a> for &'a String {
    fn csv(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> Field<'a> for &'a Option<String> {
    fn csv(self) -> Cow<'a, str> {
        self.as_deref().map(Cow::from).unwrap_or_default()
    }
}

impl<T> Field<'static> for Option<T>
where
    T: ToString,
{
    fn csv(self) -> Cow<'static, str> {
        self.as_ref()
            .map(ToString::to_string)
            .map(Cow::from)
            .unwrap_or_default()
    }
}

pub struct Builder<T> {
    fields: Vec<(String, Box<dyn Fn(&T) -> Cow<str>>)>,
    _phantom: PhantomData<T>,
}

impl<T> Builder<T> {
    pub fn new() -> Self {
        Builder {
            fields: Vec::new(),
            _phantom: PhantomData,
        }
    }
    pub fn add<FN: 'static>(mut self, name: &'static str, f: FN) -> Self
    where
        for<'a> FN: Fn(&'a T) -> Cow<'a, str>,
    {
        self.fields
            .push((name.to_owned(), Box::new(move |value| f(value))));
        self
    }
    pub fn record<'a>(&'a self, value: &'a T) -> impl Iterator<Item = Cow<[u8]>> + '_ {
        self.fields.iter().map(|(_, f)| match f(value) {
            Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
            Cow::Owned(s) => Cow::Owned(s.into_bytes()),
        })
    }
    pub fn fields(&self) -> impl Iterator<Item = &str> + '_ {
        self.fields.iter().map(|(name, _)| name.as_str())
    }
    pub fn nest<N, G, F>(mut self, prefix: &'static str, g: G, f: F) -> Self
    where
        N: 'static,
        G: Fn(&T) -> Option<&N> + Clone + 'static,
        F: FnOnce() -> Builder<N>,
    {
        for (name, getter) in f().fields {
            let name = match prefix {
                "" => name,
                _ => format!("{prefix}_{name}"),
            };
            let g = g.clone();
            let getter: Box<dyn Fn(&T) -> Cow<str>> = Box::new(move |value| match g(value) {
                Some(nested) => getter(nested),
                None => Cow::default(),
            });
            self.fields.push((name, getter));
        }
        self
    }
}

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

fn calendar(year: Option<u16>, month: Option<u8>, week: Option<u8>) -> String {
    let year = year.map(|year| year.to_string()).unwrap_or_default();
    let prefix = month
        .and_then(month_name)
        .map(Cow::Borrowed)
        .or_else(|| week.map(|week| Cow::from(week.to_string())));
    match prefix {
        Some(prefix) => format!("{prefix}/{year}"),
        _ => year,
    }
}

fn calendar_short(year: Option<u16>, month: Option<u8>, week: Option<u8>) -> String {
    let year = year.map(|year| year.to_string()).unwrap_or_default();
    let prefix = month
        .and_then(month_name)
        .map(|month| Cow::Borrowed(&month[..3]))
        .or_else(|| week.map(|week| Cow::from(format!("Week {week}"))));
    match prefix {
        Some(prefix) => format!("{prefix}/{year}"),
        _ => year,
    }
}
