// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use gbhwdb_model::{Console, config::cartridge::GamePlatform};
use maud::{Markup, Render, html};

use crate::site::SiteSection;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SiteHeader {
    pub section: SiteSection,
}

impl SiteHeader {
    fn primary_nav(&self) -> Markup {
        html! {
            nav.site-primary-nav {
                ul {
                    li.active[matches!(self.section, SiteSection::Consoles(_))] {
                        a href="/" { "Consoles" }
                    }
                    li.active[matches!(self.section, SiteSection::Cartridges(_))] {
                        a href="/cartridges" { "Game cartridges" }
                    }
                }
            }
        }
    }
    fn secondary_nav(&self) -> Markup {
        html! {
            nav.site-secondary-nav {
                @if let SiteSection::Consoles(selected) = self.section {
                    ul {
                        @for console in Console::ALL {
                            li.active[selected == Some(console)] {
                                a href={ "/consoles/" (console.id()) } {
                                    strong { (console.code()) }
                                    span.name { (console.name()) }
                                }
                            }
                        }
                    }
                }
                @if let SiteSection::Cartridges(selected) = self.section {
                    ul {
                        @for class in GamePlatform::ALL {
                            li.active[selected == Some(class)] {
                                a href={ "/cartridges/" (class.id()) ".html" } {
                                    strong { (class.name()) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Render for SiteHeader {
    fn render(&self) -> Markup {
        html! {
            header.site-header {
                div.site-header__primary {
                    h1.site-header__title {
                        a href="/" {
                            "Game Boy hardware database"
                            aside { "by Gekkio and contributors" }
                        }
                    }
                    (self.primary_nav())
                }
                (self.secondary_nav())
            }
        }
    }
}
