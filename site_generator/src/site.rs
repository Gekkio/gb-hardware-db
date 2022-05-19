use anyhow::Error;
use gbhwdb_backend::Console;
use percy_dom::{View, VirtualNode};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};
use time::OffsetDateTime;

use crate::{
    legacy::LegacySubmission,
    template::{
        console_page::ConsolePage, console_submission_list::ConsoleSubmissionList,
        dmg_console_page::DmgConsolePage, dmg_submission_list::DmgSubmissionList, home::Home,
        markdown::Markdown, markdown_page::MarkdownPage, page,
    },
    SiteData,
};

pub fn build_site() -> Site {
    let mut site = Site::new();
    site.add_page(["index"], |data| {
        let content = Markdown::parse(&fs::read_to_string("content/home.markdown")?);
        let today = OffsetDateTime::now_local()
            .unwrap_or_else(|_| OffsetDateTime::now_utc())
            .date();
        let counts = data.counts();
        let cartridge_submission_count = counts.cartridges;
        let console_submission_count = counts.consoles.values().sum();
        Ok(Page {
            title: Cow::Borrowed("Home"),
            section: SiteSection::Consoles(None),
            content: Home {
                content,
                today,
                cartridge_submission_count,
                console_submission_count,
            }
            .render(),
        })
    });
    site.add_markdown_page(
        ["consoles", "index"],
        "Consoles",
        SiteSection::Consoles(None),
        "content/consoles.markdown",
    );
    site.add_markdown_page(
        ["contribute", "index"],
        "Contribute",
        SiteSection::Consoles(None),
        "content/contribute.markdown",
    );
    site.add_markdown_page(
        ["contribute", "sgb"],
        "Super Game Boy (SGB) contribution instructions",
        SiteSection::Consoles(Some(Console::Sgb)),
        "content/contribute-sgb.markdown",
    );
    site.add_markdown_page(
        ["contribute", "sgb2"],
        "Super Game Boy 2 (SGB2) contribution instructions",
        SiteSection::Consoles(Some(Console::Sgb2)),
        "content/contribute-sgb2.markdown",
    );
    site.add_markdown_page(
        ["contribute", "oxy"],
        "Game Boy Micro (OXY) contribution instructions",
        SiteSection::Consoles(Some(Console::Oxy)),
        "content/contribute-oxy.markdown",
    );
    site.add_markdown_page(
        ["contribute", "cartridges"],
        "Game cartridge contribution instructions",
        SiteSection::Consoles(None),
        "content/contribute-cartridges.markdown",
    );
    for console in Console::ALL {
        site.add_page(["consoles", console.id(), "index"], move |data| {
            Ok(Page {
                title: format!("{} ({})", console.name(), console.code()).into(),
                section: SiteSection::Consoles(Some(console)),
                content: match console {
                    Console::Dmg => DmgSubmissionList::new(&data.dmg).render(),
                    Console::Sgb => ConsoleSubmissionList::new(&data.sgb).render(),
                    Console::Mgb => ConsoleSubmissionList::new(&data.mgb).render(),
                    Console::Mgl => ConsoleSubmissionList::new(&data.mgl).render(),
                    Console::Sgb2 => ConsoleSubmissionList::new(&data.sgb2).render(),
                    Console::Cgb => ConsoleSubmissionList::new(&data.cgb).render(),
                    Console::Agb => ConsoleSubmissionList::new(&data.agb).render(),
                    Console::Ags => ConsoleSubmissionList::new(&data.ags).render(),
                    Console::Gbs => ConsoleSubmissionList::new(&data.gbs).render(),
                    Console::Oxy => ConsoleSubmissionList::new(&data.oxy).render(),
                },
            })
        });
        fn create_pages<M, P>(
            console: Console,
            submissions: &[LegacySubmission<M, P>],
            f: impl Fn(&LegacySubmission<M, P>) -> VirtualNode,
        ) -> Vec<(SitePath, Page)> {
            submissions
                .iter()
                .map(move |submission| {
                    let path = SitePath(vec![
                        Cow::Borrowed("consoles"),
                        Cow::Borrowed(console.id()),
                        Cow::Owned(submission.slug.clone()),
                    ]);
                    let page = Page {
                        title: format!(
                            "{}: {title} [{contributor}]",
                            console.code(),
                            title = submission.title,
                            contributor = submission.contributor
                        )
                        .into(),
                        section: SiteSection::Consoles(Some(console)),
                        content: f(submission),
                    };
                    (path, page)
                })
                .collect()
        }
        site.page_sets.push(match console {
            Console::Dmg => Box::new(move |data| {
                create_pages(console, &data.dmg, |s| DmgConsolePage::new(s).render())
            }),
            Console::Sgb => Box::new(move |data| {
                create_pages(console, &data.sgb, |s| ConsolePage::new(s).render())
            }),
            Console::Mgb => Box::new(move |data| {
                create_pages(console, &data.mgb, |s| ConsolePage::new(s).render())
            }),
            Console::Mgl => Box::new(move |data| {
                create_pages(console, &data.mgl, |s| ConsolePage::new(s).render())
            }),
            Console::Sgb2 => Box::new(move |data| {
                create_pages(console, &data.sgb2, |s| ConsolePage::new(s).render())
            }),
            Console::Cgb => Box::new(move |data| {
                create_pages(console, &data.cgb, |s| ConsolePage::new(s).render())
            }),
            Console::Agb => Box::new(move |data| {
                create_pages(console, &data.agb, |s| ConsolePage::new(s).render())
            }),
            Console::Ags => Box::new(move |data| {
                create_pages(console, &data.ags, |s| ConsolePage::new(s).render())
            }),
            Console::Gbs => Box::new(move |data| {
                create_pages(console, &data.gbs, |s| ConsolePage::new(s).render())
            }),
            Console::Oxy => Box::new(move |data| {
                create_pages(console, &data.oxy, |s| ConsolePage::new(s).render())
            }),
        });
    }
    site
}

pub struct Page {
    pub title: Cow<'static, str>,
    pub section: SiteSection,
    pub content: VirtualNode,
}

impl Page {
    pub fn generate(self) -> Result<String, Error> {
        Ok(page(&self.title, self.section, self.content))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SitePath(Vec<Cow<'static, str>>);

impl SitePath {
    pub fn join<P: AsRef<Path>>(&self, base: P) -> PathBuf {
        let mut result = base.as_ref().to_path_buf();
        for segment in &self.0 {
            result.push(segment.as_ref());
        }
        result.set_extension("html");
        result
    }
}

impl<const N: usize> From<[&'static str; N]> for SitePath {
    fn from(segments: [&'static str; N]) -> Self {
        SitePath(segments.into_iter().map(Cow::Borrowed).collect())
    }
}

pub struct Site {
    pub pages: HashMap<SitePath, Box<dyn Fn(&SiteData) -> Result<Page, Error>>>,
    pub page_sets: Vec<Box<dyn Fn(&SiteData) -> Vec<(SitePath, Page)>>>,
}

impl Site {
    pub fn new() -> Self {
        Site {
            pages: HashMap::new(),
            page_sets: Vec::new(),
        }
    }
    pub fn add_markdown_page<P: Into<SitePath>, S: AsRef<Path>>(
        &mut self,
        path: P,
        title: &'static str,
        section: SiteSection,
        source: S,
    ) {
        let source = source.as_ref().to_path_buf();
        self.pages.insert(
            path.into(),
            Box::new(move |_| {
                let markdown = Markdown::parse(&fs::read_to_string(&source)?);
                Ok(Page {
                    title: Cow::Borrowed(title),
                    section,
                    content: MarkdownPage { markdown }.render(),
                })
            }),
        );
    }
    pub fn add_page<P: Into<SitePath>>(
        &mut self,
        path: P,
        generator: impl Fn(&SiteData) -> Result<Page, Error> + 'static,
    ) {
        self.pages.insert(path.into(), Box::new(generator));
    }
    pub fn generate_all(
        &mut self,
        data: &SiteData,
        target_dir: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let target_dir = target_dir.as_ref();
        for (path, generator) in &self.pages {
            match generator(data).and_then(|page| page.generate()) {
                Ok(content) => {
                    let target_file = path.join(target_dir);
                    if let Some(parent) = target_file.parent() {
                        create_dir_all(parent)?;
                    }
                    fs::write(target_file, content.as_bytes())?;
                }
                Err(err) => {
                    eprintln!("{}", err)
                }
            }
        }
        for page_set in &self.page_sets {
            for (path, page) in page_set(data) {
                let target_file = path.join(target_dir);
                if let Some(parent) = target_file.parent() {
                    create_dir_all(parent)?;
                }
                fs::write(target_file, page.generate()?)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SubmissionCounts {
    pub cartridges: u32,
    pub consoles: HashMap<Console, u32>,
}

impl SubmissionCounts {
    pub fn update(&mut self, console: Console, count: u32) {
        self.consoles.insert(console, count);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SiteSection {
    Consoles(Option<Console>),
    Cartridges,
}
