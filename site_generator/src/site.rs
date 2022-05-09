use anyhow::Error;
use gbhwdb_backend::Console;
use percy_dom::{View, VirtualNode};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
};

use crate::{
    template::{markdown::Markdown, markdown_page::MarkdownPage, page},
    SiteData,
};

pub fn build_site() -> Site {
    let mut site = Site::new();
    site.add_markdown_page(
        ["index"],
        "Home",
        SiteSection::Consoles(None),
        "content/home.markdown",
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
    site.add_page(
        ["consoles", "sgb", "index"],
        Page {
            title: "Super Game Boy (SGB)".into(),
            section: SiteSection::Consoles(Some(Console::Sgb)),
            generator: Box::new(|data| {
                Ok(
                    crate::template::console_submission_list::ConsoleSubmissionList {
                        submissions: &data.sgb,
                    }
                    .render(),
                )
            }),
        },
    );
    site.add_page(
        ["consoles", "sgb2", "index"],
        Page {
            title: "Super Game Boy 2 (SGB2)".into(),
            section: SiteSection::Consoles(Some(Console::Sgb2)),
            generator: Box::new(|data| {
                Ok(
                    crate::template::console_submission_list::ConsoleSubmissionList {
                        submissions: &data.sgb2,
                    }
                    .render(),
                )
            }),
        },
    );
    site.add_page(
        ["consoles", "mgb", "index"],
        Page {
            title: "Game Boy Pocket (MGB)".into(),
            section: SiteSection::Consoles(Some(Console::Mgb)),
            generator: Box::new(|data| {
                Ok(
                    crate::template::console_submission_list::ConsoleSubmissionList {
                        submissions: &data.mgb,
                    }
                    .render(),
                )
            }),
        },
    );
    site.add_page(
        ["consoles", "mgl", "index"],
        Page {
            title: "Game Boy Light (MGL)".into(),
            section: SiteSection::Consoles(Some(Console::Mgl)),
            generator: Box::new(|data| {
                Ok(
                    crate::template::console_submission_list::ConsoleSubmissionList {
                        submissions: &data.mgl,
                    }
                    .render(),
                )
            }),
        },
    );
    site
}

pub struct Page {
    pub title: Cow<'static, str>,
    pub section: SiteSection,
    pub generator: Box<dyn Fn(&SiteData) -> Result<VirtualNode, Error>>,
}

impl Page {
    pub fn generate(&self, data: &SiteData, counts: &SubmissionCounts) -> Result<String, Error> {
        let content = (self.generator)(data)?;
        Ok(page(&self.title, self.section, content, counts))
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
    pub pages: HashMap<SitePath, Page>,
}

impl Site {
    pub fn new() -> Self {
        Site {
            pages: HashMap::new(),
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
            Page {
                title: Cow::Borrowed(title),
                section,
                generator: Box::new(move |_| {
                    let markdown = Markdown::parse(&fs::read_to_string(&source)?);
                    Ok(MarkdownPage { markdown }.render())
                }),
            },
        );
    }
    pub fn add_page<P: Into<SitePath>>(&mut self, path: P, page: Page) {
        self.pages.insert(path.into(), page);
    }
    pub fn generate_all(
        &mut self,
        data: &SiteData,
        target_dir: impl AsRef<Path>,
    ) -> Result<(), Error> {
        let target_dir = target_dir.as_ref();
        let counts = data.counts();
        for (path, page) in &self.pages {
            match page.generate(data, &counts) {
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
