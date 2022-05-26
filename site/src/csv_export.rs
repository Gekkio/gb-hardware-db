use anyhow::Error;
use std::{borrow::Cow, io, marker::PhantomData};

use crate::legacy::{HasDateCode, LegacyChip, LegacySubmission};

mod agb;
mod ags;
mod cartridge;
mod cgb;
mod dmg;
mod gbs;
mod mgb;
mod mgl;
mod oxy;
mod sgb;
mod sgb2;

pub trait ToCsv: Sized {
    fn csv_builder() -> Builder<Self>;
}

pub fn write_submission_csv<W, M, P>(
    writer: W,
    url_prefix: &'static str,
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
        .add("url", move |s| {
            format!("{url_prefix}/{}/{}.html", s.code, s.slug).csv()
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
        .add_date_code()
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
    pub fn add_date_code(self) -> Self
    where
        T: HasDateCode,
    {
        let mut result = self;
        result = result.add("calendar_short", |v| v.date_code().calendar_short().csv());
        result = result.add("calendar", |v| v.date_code().calendar().csv());
        result = result.add("year", |v| v.date_code().year.csv());
        result = result.add("month", |v| v.date_code().month.csv());
        result = result.add("week", |v| v.date_code().week.csv());
        result
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
