use anyhow::Error;
use percy_dom::{View, VirtualNode};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt, fs,
    path::{Path, PathBuf},
};

use crate::template::{markdown::Markdown, markdown_page::MarkdownPage, page};

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
    site
}

pub struct Page {
    pub title: Cow<'static, str>,
    pub section: SiteSection,
    pub generator: Box<dyn Fn() -> Result<VirtualNode, Error>>,
}

impl Page {
    pub fn generate(&self, counts: &SubmissionCounts) -> Result<String, Error> {
        let content = (self.generator)()?;
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
        SitePath(segments.into_iter().copied().map(Cow::Borrowed).collect())
    }
}

pub struct Site {
    pub pages: HashMap<SitePath, Page>,
    pub counts: SubmissionCounts,
}

impl Site {
    pub fn new() -> Self {
        Site {
            pages: HashMap::new(),
            counts: SubmissionCounts::default(),
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
                generator: Box::new(move || {
                    let markdown = Markdown::parse(&fs::read_to_string(&source)?);
                    Ok(MarkdownPage { markdown }.render())
                }),
            },
        );
    }
    pub fn generate_all(&mut self, target_dir: impl AsRef<Path>) -> Result<(), Error> {
        let target_dir = target_dir.as_ref();
        for (path, page) in &self.pages {
            match page.generate(&self.counts) {
                Ok(content) => {
                    let target_file = path.join(target_dir);
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Console {
    Dmg,
    Sgb,
    Mgb,
    Mgl,
    Sgb2,
    Cgb,
    Agb,
    Ags,
    Gbs,
    Oxy,
}

impl Console {
    pub const ALL: [Console; 10] = [
        Console::Dmg,
        Console::Sgb,
        Console::Mgb,
        Console::Mgl,
        Console::Sgb2,
        Console::Cgb,
        Console::Agb,
        Console::Ags,
        Console::Gbs,
        Console::Oxy,
    ];
    pub fn id(&self) -> &'static str {
        match self {
            Console::Dmg => "dmg",
            Console::Sgb => "sgb",
            Console::Mgb => "mgb",
            Console::Mgl => "mgl",
            Console::Sgb2 => "sgb2",
            Console::Cgb => "cgb",
            Console::Agb => "agb",
            Console::Ags => "ags",
            Console::Gbs => "gbs",
            Console::Oxy => "oxy",
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Console::Dmg => "Game Boy",
            Console::Sgb => "Super Game Boy",
            Console::Mgb => "Game Boy Pocket",
            Console::Mgl => "Game Boy Light",
            Console::Sgb2 => "Super Game Boy 2",
            Console::Cgb => "Game Boy Color",
            Console::Agb => "Game Boy Advance",
            Console::Ags => "Game Boy Advance SP",
            Console::Gbs => "Game Boy Player",
            Console::Oxy => "Game Boy Micro",
        }
    }
}

impl fmt::Display for Console {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
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
