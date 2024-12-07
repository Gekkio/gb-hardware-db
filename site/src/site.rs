// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use anyhow::Error;
use gbhwdb_model::{
    config::cartridge::{BoardConfig, BoardPart, GamePlatform, PartRole},
    Console,
};
use itertools::Itertools;
use lexical_sort::natural_lexical_cmp;
use log::error;
use maud::{Markup, Render};
use slug::slugify;
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, create_dir_all},
    path::{Path, PathBuf},
    sync::OnceLock,
};
use time::OffsetDateTime;

use crate::{
    legacy::LegacySubmission,
    template::{
        cartridge_page::CartridgePage,
        cartridges::Cartridges,
        console_page::ConsolePage,
        console_submission_list::ConsoleSubmissionList,
        contributor_cartridges::ContributorCartridges,
        dmg_console_page::DmgConsolePage,
        dmg_submission_list::DmgSubmissionList,
        game::Game,
        home::Home,
        mapper::{Mapper, MapperCfg},
        markdown::Markdown,
        markdown_page::MarkdownPage,
        page,
        platform_cartridges::PlatformCartridges,
    },
    SiteData,
};

static MAPPER_CFGS: OnceLock<Vec<MapperCfg>> = OnceLock::new();

pub fn build_site() -> Site {
    let mut site = Site::new();
    site.add_page(["index"], |data| {
        let content = Markdown::parse(&fs::read_to_string("site/content/home.markdown")?);
        let today = OffsetDateTime::now_local()
            .unwrap_or_else(|_| OffsetDateTime::now_utc())
            .date();
        let counts = data.submissions.counts();
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
        "site/content/consoles.markdown",
    );
    site.add_markdown_page(
        ["contribute", "index"],
        "Contribute",
        SiteSection::Consoles(None),
        "site/content/contribute.markdown",
    );
    site.add_markdown_page(
        ["contribute", "sgb"],
        "Super Game Boy (SGB) contribution instructions",
        SiteSection::Consoles(Some(Console::Sgb)),
        "site/content/contribute-sgb.markdown",
    );
    site.add_markdown_page(
        ["contribute", "sgb2"],
        "Super Game Boy 2 (SGB2) contribution instructions",
        SiteSection::Consoles(Some(Console::Sgb2)),
        "site/content/contribute-sgb2.markdown",
    );
    site.add_markdown_page(
        ["contribute", "oxy"],
        "Game Boy Micro (OXY) contribution instructions",
        SiteSection::Consoles(Some(Console::Oxy)),
        "site/content/contribute-oxy.markdown",
    );
    site.add_markdown_page(
        ["contribute", "cartridges"],
        "Game cartridge contribution instructions",
        SiteSection::Consoles(None),
        "site/content/contribute-cartridges.markdown",
    );
    for console in Console::ALL {
        site.add_page(["consoles", console.id(), "index"], move |data| {
            let data = &data.submissions;
            Ok(Page {
                title: format!("{} ({})", console.name(), console.code()).into(),
                section: SiteSection::Consoles(Some(console)),
                content: match console {
                    Console::Dmg => DmgSubmissionList::new(&data.dmg).render(),
                    Console::Sgb => ConsoleSubmissionList::new(&data.sgb)
                        .render_console_column(false)
                        .render(),
                    Console::Mgb => ConsoleSubmissionList::new(&data.mgb).render(),
                    Console::Mgl => ConsoleSubmissionList::new(&data.mgl).render(),
                    Console::Sgb2 => ConsoleSubmissionList::new(&data.sgb2)
                        .render_console_column(false)
                        .render(),
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
            f: impl Fn(&LegacySubmission<M, P>) -> Markup,
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
                create_pages(console, &data.submissions.dmg, |s| {
                    DmgConsolePage::new(s).render()
                })
            }),
            Console::Sgb => Box::new(move |data| {
                create_pages(console, &data.submissions.sgb, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Mgb => Box::new(move |data| {
                create_pages(console, &data.submissions.mgb, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Mgl => Box::new(move |data| {
                create_pages(console, &data.submissions.mgl, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Sgb2 => Box::new(move |data| {
                create_pages(console, &data.submissions.sgb2, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Cgb => Box::new(move |data| {
                create_pages(console, &data.submissions.cgb, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Agb => Box::new(move |data| {
                create_pages(console, &data.submissions.agb, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Ags => Box::new(move |data| {
                create_pages(console, &data.submissions.ags, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Gbs => Box::new(move |data| {
                create_pages(console, &data.submissions.gbs, |s| {
                    ConsolePage::new(s).render()
                })
            }),
            Console::Oxy => Box::new(move |data| {
                create_pages(console, &data.submissions.oxy, |s| {
                    ConsolePage::new(s).render()
                })
            }),
        });
    }
    let mapper_cfgs = MAPPER_CFGS.get_or_init(|| {
        vec![
            MapperCfg {
                id: "no-mapper",
                name: "No mapper",
                parts: &[PartRole::Rom],
                match_fn: Box::new(|cfg, _| cfg == BoardConfig::Aaac || cfg == BoardConfig::DmgAaa),
            },
            MapperCfg {
                id: "mbc1",
                name: "MBC1",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind.starts_with("MBC1"))
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mbc2",
                name: "MBC2",
                parts: &[PartRole::Rom, PartRole::Mapper, PartRole::SupervisorReset],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind.starts_with("MBC2"))
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mbc3",
                name: "MBC3",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                    PartRole::Crystal,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind != "MBC30" && kind.starts_with("MBC3"))
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mbc30",
                name: "MBC30",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                    PartRole::Crystal,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind == "MBC30")
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mbc5",
                name: "MBC5",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind.starts_with("MBC5"))
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mbc6",
                name: "MBC6",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind == "MBC6")
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mbc7",
                name: "MBC7",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Eeprom,
                    PartRole::Accelerometer,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind == "MBC7")
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "mmm01",
                name: "MMM01",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind == "MMM01")
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "huc1",
                name: "HuC-1",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind.starts_with("HuC-1"))
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "huc3",
                name: "HuC-3",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Ram,
                    PartRole::SupervisorReset,
                    PartRole::HexInverter,
                    PartRole::Crystal,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind.starts_with("HuC-3"))
                        .unwrap_or(false)
                }),
            },
            MapperCfg {
                id: "tama5",
                name: "TAMA5",
                parts: &[
                    PartRole::Rom,
                    PartRole::Mapper,
                    PartRole::Mcu,
                    PartRole::Rtc,
                    PartRole::SupervisorReset,
                    PartRole::Crystal,
                ],
                match_fn: Box::new(|_, part| {
                    part.and_then(|part| part.kind.as_ref())
                        .map(|kind| kind == "TAMA5")
                        .unwrap_or(false)
                }),
            },
        ]
    });
    site.add_page(["cartridges", "index"], move |_| {
        let content = Markdown::parse(&fs::read_to_string("site/content/cartridges.markdown")?);
        Ok(Page {
            title: "Cartridges".into(),
            section: SiteSection::Cartridges(None),
            content: Cartridges { content }.render(),
        })
    });
    site.add_page(["cartridges", "gb"], move |data| {
        Ok(Page {
            title: "Game Boy cartridges".into(),
            section: SiteSection::Cartridges(Some(GamePlatform::Gb)),
            content: PlatformCartridges {
                platform: GamePlatform::Gb,
                mapper_cfgs,
                cfgs: &data.cfgs,
                submissions: &data.submissions.cartridges,
            }
            .render(),
        })
    });
    site.add_page(["cartridges", "gbc"], move |data| {
        Ok(Page {
            title: "Game Boy Color cartridges".into(),
            section: SiteSection::Cartridges(Some(GamePlatform::Gbc)),
            content: PlatformCartridges {
                platform: GamePlatform::Gbc,
                mapper_cfgs,
                cfgs: &data.cfgs,
                submissions: &data.submissions.cartridges,
            }
            .render(),
        })
    });
    site.add_page(["cartridges", "gba"], move |data| {
        Ok(Page {
            title: "Game Boy Advance cartridges".into(),
            section: SiteSection::Cartridges(Some(GamePlatform::Gba)),
            content: PlatformCartridges {
                platform: GamePlatform::Gba,
                mapper_cfgs,
                cfgs: &data.cfgs,
                submissions: &data.submissions.cartridges,
            }
            .render(),
        })
    });
    site.page_sets.push(Box::new(move |data| {
        data.submissions
            .cartridges
            .iter()
            .sorted_by_key(|submission| &submission.code)
            .chunk_by(|submission| &submission.code)
            .into_iter()
            .map(|(code, group)| {
                let cfg = data.cfgs[code].clone();
                let submissions = group.collect::<Vec<_>>();
                let path = SitePath(vec![
                    Cow::Borrowed("cartridges"),
                    Cow::Owned(cfg.rom_id.clone()),
                    Cow::Borrowed("index"),
                ]);
                let page = Page {
                    title: Cow::Owned(cfg.name.clone()),
                    section: SiteSection::Cartridges(Some(cfg.platform)),
                    content: Game {
                        cfg: &cfg,
                        submissions,
                    }
                    .render(),
                };
                (path, page)
            })
            .collect()
    }));
    site.page_sets.push(Box::new(move |data| {
        data.submissions
            .cartridges
            .iter()
            .map(move |submission| {
                let cfg = &submission.metadata.cfg;
                let path = SitePath(vec![
                    Cow::Borrowed("cartridges"),
                    Cow::Owned(submission.code.clone()),
                    Cow::Owned(submission.slug.clone()),
                ]);
                let page = Page {
                    title: format!(
                        "{}: {title} [{contributor}]",
                        cfg.name,
                        title = submission.title,
                        contributor = submission.contributor
                    )
                    .into(),
                    section: SiteSection::Cartridges(Some(cfg.platform)),
                    content: CartridgePage::new(submission).render(),
                };
                (path, page)
            })
            .collect()
    }));
    site.page_sets.push(Box::new(move |data| {
        data.submissions
            .cartridges
            .iter()
            .map(|submission| {
                let board = &submission.metadata.board;
                let mapper = board
                    .cfg
                    .parts()
                    .find(|(_, part)| matches!(part, BoardPart::Mapper(_)))
                    .and_then(|(designator, _)| board.parts.get(&designator));
                let key = mapper_cfgs
                    .iter()
                    .position(|cfg| (cfg.match_fn)(board.cfg, mapper));
                (key, submission)
            })
            .sorted_by_key(|&(key, _)| key)
            .chunk_by(|&(key, _)| key)
            .into_iter()
            .filter_map(|(cfg_idx, group)| {
                cfg_idx.map(|idx| (idx, group.map(|(_, submission)| submission)))
            })
            .map(|(cfg_idx, group)| {
                let cfg = &mapper_cfgs[cfg_idx];
                let submissions = group
                    .sorted_unstable_by(|a, b| {
                        natural_lexical_cmp(&a.metadata.cfg.name, &b.metadata.cfg.name)
                            .then_with(|| a.sort_group.as_ref().cmp(&b.sort_group.as_ref()))
                    })
                    .collect::<Vec<_>>();
                let path = SitePath(vec![Cow::Borrowed("cartridges"), Cow::Borrowed(cfg.id)]);
                let page = Page {
                    title: Cow::Borrowed(cfg.name),
                    section: SiteSection::Cartridges(None),
                    content: Mapper { cfg, submissions }.render(),
                };
                (path, page)
            })
            .collect()
    }));
    site.page_sets.push(Box::new(move |data| {
        let mut result = Vec::new();

        for (contributor, submissions) in data.submissions.by_contributor() {
            if submissions.counts().cartridges > 0 {
                let contributor_slug = slugify(&contributor);
                let path = SitePath(vec![
                    Cow::Borrowed("cartridges"),
                    Cow::Borrowed("contributors"),
                    Cow::Owned(contributor_slug),
                ]);
                let page = Page {
                    title: Cow::from(format!("Cartridge submissions by {contributor}")),
                    section: SiteSection::Cartridges(None),
                    content: ContributorCartridges {
                        contributor,
                        submissions: &submissions,
                    }
                    .render(),
                };
                result.push((path, page))
            }
        }
        result
    }));

    site
}

pub struct Page {
    pub title: Cow<'static, str>,
    pub section: SiteSection,
    pub content: Markup,
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
                    error!("{}", err)
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
    Cartridges(Option<GamePlatform>),
}
